mod enums;
mod fields;
mod structs;
mod variants;

use moxy_ast::List;
use moxy_diagnostic::SpanExt;
use moxy_token::{Ident, LitStr, Spanner, ToTokenStream, TokenStream};

fn parse_error(message: Option<&LitStr>, ident: &LitStr, span: TokenStream, path: TokenStream, fallback: &str) -> TokenStream {
    match message {
        Some(message) => moxy::template! {
            ::moxy::ast::ParseError::new(
                {{ span }},
                ::std::format!(
                    {{ message }},
                    ident = {{ ident }},
                    path = {{ path }},
                ),
            )
        },
        None => {
            let fallback = LitStr::new(fallback, moxy::token::Span::call_site());
            moxy::template! { ::moxy::ast::ParseError::new({{ span }}, {{ fallback }}) }
        }
    }
}

pub fn expand(tokens: TokenStream) -> TokenStream {
    let target = match moxy::parse!(tokens as moxy::ast::Declaration) {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let (ident, generics, body) = match &target {
        moxy::ast::Declaration::Struct(target) => (&target.ident, &target.generics, structs::expand(target)),
        moxy::ast::Declaration::Enum(target) => (&target.ident, &target.generics, enums::expand(target)),
        moxy::ast::Declaration::Union(target) => {
            return target.span().error("`FromMeta` cannot be derived for unions").emit();
        }
    };

    let (impl_generics, type_generics, where_clause) = generics.split();
    let where_clause = if let Some(mut clause) = where_clause.cloned() {
        for param in &generics.params {
            if let moxy::ast::GenericParam::Type(param) = param {
                let tokens = moxy::template! {
                    {{ param.ident }}: ::moxy::ast::FromMeta
                };

                clause.predicates.push_value(match moxy::parse!(tokens) {
                    Err(err) => return err.to_compile_error(),
                    Ok(v) => v,
                });

                clause.predicates.push_punct(Default::default());
            }
        }

        Some(clause)
    } else {
        None
    };

    moxy::template! {
        impl {{ impl_generics }} ::moxy::ast::FromMeta for {{ ident }} {{ type_generics }} {{ where_clause }} {
            fn from_meta(meta: &::moxy::ast::Meta) -> Result<Self, ::moxy::ast::ParseError> {
                let parser = match &meta.content {
                    ::moxy::ast::MetaContent::List(group) => &::moxy::ast::Parser::from_group(group),
                    ::moxy::ast::MetaContent::Expr { eq, .. } => {
                        return Err(::moxy::ast::ParseError::expected(eq, "parenthesized meta arguments"));
                    }
                    ::moxy::ast::MetaContent::Unit => {
                        return Err(::moxy::ast::ParseError::new_spanned(
                            &meta.path,
                            "expected parenthesized meta arguments, found no value",
                        ));
                    }
                };
                {{ body }}
            }
        }
    }
}

fn rules_group(attr: &moxy::ast::Attribute) -> Result<&moxy_token::Group, moxy::ast::ParseError> {
    let expected = "`#[meta(...)]` with parenthesized rules";

    match &attr.content {
        moxy::ast::MetaContent::List(group) => Ok(group),
        moxy::ast::MetaContent::Unit => Err(moxy::ast::ParseError::new(
            attr.span(),
            format!("expected {expected}, found no value"),
        )),
        moxy::ast::MetaContent::Expr { eq, expr } => {
            if let Some(value) = moxy_token::__private::short_token_text(expr) {
                let found = moxy_token::__private::quoted(&format!("{eq} {value}"));
                Err(moxy::ast::ParseError::new_spanned(
                    &attr.content,
                    format!("expected {expected}, found {found}"),
                ))
            } else {
                Err(moxy::ast::ParseError::expected(eq, expected))
            }
        }
    }
}

#[derive(Clone)]
enum MetaRule {
    Default(Option<moxy::ast::Expr>),
    Rename(LitStr),
    Message(LitStr),
}

impl Spanner for MetaRule {
    fn span(&self) -> moxy::token::Span {
        match self {
            Self::Default(v) => v.span(),
            Self::Rename(v) => v.span(),
            Self::Message(v) => v.span(),
        }
    }
}

impl moxy::ast::Parse for MetaRule {
    fn peek(cursor: moxy::ast::Cursor<'_>) -> bool {
        moxy::ast::Meta::peek(cursor)
    }

    fn parse(parser: &moxy::ast::Parser) -> Result<Self, moxy::ast::ParseError> {
        let meta = <moxy::ast::Meta as moxy::ast::Parse>::parse(parser)?;
        let Some(ident) = meta.path.as_ident() else {
            let path = moxy::token::ToTokenStream::to_token_stream(&meta.path);
            return moxy::ast::ParseError::new(meta.path.span(), format!("expected meta rule name, found `{path}`")).into();
        };

        match ident.text() {
            "default" => match &meta.content {
                moxy::ast::MetaContent::Unit => Ok(Self::Default(None)),
                moxy::ast::MetaContent::Expr { expr, .. } => Ok(Self::Default(Some(moxy::parse!(expr)?))),
                _ => parser.error("`default` must be bare or use `=`").into(),
            },
            "rename" => match &meta.content {
                moxy::ast::MetaContent::Expr { expr, .. } => Ok(Self::Rename(moxy::parse!(expr)?)),
                _ => parser.error("`rename` must use a string literal value").into(),
            },
            "message" => match &meta.content {
                moxy::ast::MetaContent::Expr { expr, .. } => Ok(Self::Message(moxy::parse!(expr)?)),
                _ => parser.error("`message` must use a string literal value").into(),
            },
            _ => parser.error(format!("unknown meta rule `{ident}`")).into(),
        }
    }

    fn skip(cursor: moxy::ast::Cursor<'_>) -> Option<moxy::ast::Cursor<'_>> {
        moxy::ast::Meta::skip(cursor)
    }
}
