use moxy_token::{Punct, TokenTree, punct};

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Punct {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_punct()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Punct(v)) => Ok(*v),
            _ => Err(parser.error("expected punctuation")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

macro_rules! impl_punct_parse {
    ($($name:ident),* $(,)?) => {
        $(
            impl Parse for punct::$name {
                fn peek(cursor: Cursor<'_>) -> bool {
                    let Some(next) = cursor.curr() else {
                        return false;
                    };

                    matches!(
                        next,
                        TokenTree::Punct(Punct::$name(_)),
                    )
                }

                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    match parser.parse()? {
                        Punct::$name(v) => Ok(v),
                        _ => parser.error(format!("expected `{}` punctuation", punct::$name::TEXT)).into(),
                    }
                }

                fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
                    Self::peek(cursor).then(|| cursor.offset(1))
                }
            }
        )*
    };
}

impl_punct_parse! {
    And,
    Or,
    Not,
    Tilde,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Eq,
    Lt,
    Gt,
    At,
    Dot,
    Comma,
    Semi,
    Colon,
    Pound,
    Dollar,
    Question,
    Quote,
    Underscore,
}
