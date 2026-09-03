use moxy_token::{
    Lit, LitBool, LitByte, LitByteStr, LitCStr, LitChar, LitF32, LitF64, LitFloat, LitInt, LitStr, LitVerbatim, TokenTree,
};

use crate::{Parse, ParseError, Parser, Peek};

impl Peek for Lit {
    fn peek(parser: &Parser) -> bool {
        let Some(next) = parser.curr() else {
            return false;
        };

        next.is_literal()
    }
}

impl Parse for Lit {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Literal(v)) => Ok(v.clone()),
            _ => Err(parser.error("expected literal")),
        }
    }
}

macro_rules! impl_lit_parse {
    ($($ty:ty => $variant:ident, $name:literal),* $(,)?) => {
        $(
            impl Peek for $ty {
                fn peek(parser: &Parser) -> bool {
                    let Some(next) = parser.curr() else {
                        return false;
                    };

                    matches!(next, TokenTree::Literal(Lit::$variant(_)))
                }
            }

            impl Parse for $ty {
                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    match parser.parse::<Lit>()? {
                        Lit::$variant(v) => Ok(v),
                        _ => Err(parser.error(concat!("expected ", $name, " literal"))),
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

impl Peek for LitF32 {
    fn peek(parser: &Parser) -> bool {
        matches!(parser.curr(), Some(TokenTree::Literal(Lit::Float(LitFloat::F32(_)))))
    }
}

impl Parse for LitF32 {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse::<Lit>()? {
            Lit::Float(LitFloat::F32(value)) => Ok(value),
            _ => Err(parser.error("expected `f32` literal")),
        }
    }
}

impl Peek for LitF64 {
    fn peek(parser: &Parser) -> bool {
        matches!(parser.curr(), Some(TokenTree::Literal(Lit::Float(LitFloat::F64(_)))))
    }
}

impl Parse for LitF64 {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse::<Lit>()? {
            Lit::Float(LitFloat::F64(value)) => Ok(value),
            _ => Err(parser.error("expected `f64` literal")),
        }
    }
}
