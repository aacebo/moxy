use moxy_token::{Keyword, TokenTree, keyword};

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Keyword {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_keyword()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Keyword(v)) => Ok(*v),
            _ => Err(parser.error("expected keyword")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.offset(1).into()
    }
}

macro_rules! impl_keyword_parse {
    ($($name:ident),* $(,)?) => {
        $(
            impl Parse for keyword::$name {
                fn peek(cursor: Cursor<'_>) -> bool {
                    let Some(next) = cursor.curr() else {
                        return false;
                    };

                    matches!(next, TokenTree::Keyword(Keyword::$name(_)))
                }

                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    match parser.parse::<Keyword>()? {
                        Keyword::$name(v) => Ok(v),
                        _ => Err(parser.error(format!("expected `{}` keyword", keyword::$name::TEXT))),
                    }
                }

                fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
                    cursor.offset(1).into()
                }
            }
        )*
    };
}

impl_keyword_parse! {
    As,
    Async,
    Auto,
    Await,
    Become,
    Box,
    Break,
    Const,
    Continue,
    Crate,
    Default,
    Do,
    Dyn,
    Else,
    Enum,
    Extern,
    Final,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Macro,
    MacroRules,
    Match,
    Mod,
    Move,
    Mut,
    Override,
    Priv,
    Pub,
    Raw,
    Ref,
    Return,
    SelfType,
    SelfValue,
    Static,
    Struct,
    Super,
    Trait,
    Try,
    Type,
    Typeof,
    Union,
    Unsafe,
    Unsized,
    Use,
    Virtual,
    Where,
    While,
    Yield,
}
