mod enums;
mod fields;
mod generics;
mod structs;
mod variants;

use moxy_ast::Punctuated;
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
        moxy::ast::Declaration::Struct(target) => (
            &target.ident,
            generics::Generics::from(target.generics.clone()),
            structs::expand(target),
        ),
        moxy::ast::Declaration::Enum(target) => (
            &target.ident,
            generics::Generics::from(target.generics.clone()),
            enums::expand(target),
        ),
        moxy::ast::Declaration::Union(target) => {
            return target.span().error("`Meta` cannot be derived for unions").emit();
        }
    };

    moxy::template! {
        impl {{ generics.impl_params }} ::moxy::ast::FromMeta for {{ ident }} {{ generics.type_params }} {{ generics.where_clause }} {
            fn from_meta(meta: &::moxy::ast::Meta) -> Result<Self, ::moxy::ast::ParseError> {
                let parser = match &meta.content {
                    ::moxy::ast::MetaContent::List(group) => &::moxy::ast::Parser::from_tokens(&group.tokens),
                    _ => return Err(::moxy::ast::ParseError::new(
                        ::moxy::token::Spanner::span(meta),
                        "expected parenthesized meta arguments",
                    )),
                };
                {{ body }}
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
            return parser.error("expected a meta rule name").into();
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
