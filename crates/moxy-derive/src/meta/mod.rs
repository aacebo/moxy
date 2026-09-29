mod enums;
mod fields;
mod structs;
mod variants;

use moxy_ast::Punctuated;
use moxy_diagnostic::SpanExt;
use moxy_token::{Ident, LitStr, Spanner, ToTokenStream, TokenStream};

pub fn expand(tokens: TokenStream) -> TokenStream {
    let target = match moxy::parse!(tokens as moxy::ast::Declaration) {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let moxy::ast::Declaration::Struct(target) = target else {
        return target.span().error("`Meta` can only be derived for structs").emit();
    };

    let body = structs::expand(&target);
    let ident = &target.ident;

    moxy::template! {
        impl ::moxy::ast::Parse for {{ ident }} {
            fn peek(cursor: ::moxy::ast::Cursor<'_>) -> bool {
                <::moxy::ast::Meta as ::moxy::ast::Parse>::peek(cursor)
            }

            fn parse(parser: &::moxy::ast::Parser) -> Result<Self, ::moxy::ast::ParseError> {
                {{ body }}
            }

            fn skip(mut cursor: ::moxy::ast::Cursor<'_>) -> Option<::moxy::ast::Cursor<'_>> {
                cursor = <::moxy::ast::Meta as ::moxy::ast::Parse>::skip(cursor)?;

                while <::moxy::ast::Token![,]>::peek(cursor) {
                    cursor = <::moxy::ast::Token![,]>::skip(cursor)?;
                    cursor = <::moxy::ast::Meta as ::moxy::ast::Parse>::skip(cursor)?;
                }

                Some(cursor)
            }
        }
    }
}

#[derive(Clone)]
enum MetaRule {
    Default(Option<moxy::ast::Expr>),
    Rename(LitStr),
}

impl Spanner for MetaRule {
    fn span(&self) -> moxy::token::Span {
        match self {
            Self::Default(value) => value.span(),
            Self::Rename(value) => value.span(),
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
            _ => parser.error(format!("unknown meta rule `{ident}`")).into(),
        }
    }

    fn skip(cursor: moxy::ast::Cursor<'_>) -> Option<moxy::ast::Cursor<'_>> {
        moxy::ast::Meta::skip(cursor)
    }
}
