use moxy_token::{Lit, LitBool, LitByte, LitByteStr, LitCStr, LitChar, LitFloat, LitInt, LitStr, LitVerbatim, TokenTree};

use crate::{Parse, ParseError, Parser, Peek};

impl Peek for Lit {
    fn peek(parser: &Parser) -> bool {
        let Some(next) = parser.next() else {
            return false;
        };

        next.is_literal()
    }
}

impl Parse for Lit {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Literal(v)) => Ok(v.clone()),
            _ => parser.error("expected literal").into(),
        }
    }
}

macro_rules! impl_lit_parse {
    ($($ty:ty => $variant:ident, $name:literal),* $(,)?) => {
        $(
            impl Peek for $ty {
                fn peek(parser: &Parser) -> bool {
                    let Some(next) = parser.next() else {
                        return false;
                    };

                    matches!(next, TokenTree::Literal(Lit::$variant(_)))
                }
            }

            impl Parse for $ty {
                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    match parser.parse::<Lit>()? {
                        Lit::$variant(v) => Ok(v),
                        _ => parser
                            .error(concat!("expected ", $name, " literal"))
                            .into(),
                    }
                }
            }
        )*
    };
}

impl_lit_parse! {
    LitInt      => Int,      "integer",
    LitFloat    => Float,    "float",
    LitStr      => Str,      "string",
    LitByteStr  => ByteStr,  "byte string",
    LitCStr     => CStr,     "C string",
    LitChar     => Char,     "character",
    LitByte     => Byte,     "byte",
    LitBool     => Bool,     "boolean",
    LitVerbatim => Verbatim, "verbatim",
}
