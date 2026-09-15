use moxy_token::{
    Lit, LitBool, LitByte, LitByteStr, LitCStr, LitChar, LitF32, LitF64, LitFloat, LitInt, LitStr, LitVerbatim, TokenTree,
};

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Lit {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_literal()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Literal(v)) => Ok(v.clone()),
            _ => Err(parser.error("expected literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.offset(1).into()
    }
}

impl Parse for LitF32 {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Float(LitFloat::F32(_)))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse::<Lit>()? {
            Lit::Float(LitFloat::F32(value)) => Ok(value),
            _ => Err(parser.error("expected `f32` literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.offset(1).into()
    }
}

impl Parse for LitF64 {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Float(LitFloat::F64(_)))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse::<Lit>()? {
            Lit::Float(LitFloat::F64(value)) => Ok(value),
            _ => Err(parser.error("expected `f64` literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.offset(1).into()
    }
}

macro_rules! impl_lit_parse {
    ($($ty:ty => $variant:ident, $name:literal),* $(,)?) => {
        $(
            impl Parse for $ty {
                fn peek(cursor: Cursor<'_>) -> bool {
                    let Some(next) = cursor.curr() else {
                        return false;
                    };

                    matches!(next, TokenTree::Literal(Lit::$variant(_)))
                }

                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    match parser.parse::<Lit>()? {
                        Lit::$variant(v) => Ok(v),
                        _ => Err(parser.error(concat!("expected ", $name, " literal"))),
                    }
                }

                fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
                    cursor.offset(1).into()
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
