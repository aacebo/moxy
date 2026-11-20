use moxy_token::*;

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Lit {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_literal()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(v)) => {
                parser.advance();
                Ok(v.clone())
            }
            _ => Err(parser.expected("literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitF32 {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Float(LitFloat::F32(_)))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Float(LitFloat::F32(value)))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("`f32` literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitF64 {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Float(LitFloat::F64(_)))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Float(LitFloat::F64(value)))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("`f64` literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitInt {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Int(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Int(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("integer literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitFloat {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Float(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Float(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("float literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitStr {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Str(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Str(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("string literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitByteStr {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::ByteStr(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::ByteStr(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("byte string literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitCStr {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::CStr(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::CStr(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("C string literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitChar {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Char(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Char(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("character literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitByte {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Byte(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Byte(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("byte literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for LitBool {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Bool(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Bool(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("boolean literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for bool {
    fn peek(cursor: Cursor<'_>) -> bool {
        LitBool::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(<LitBool as Parse>::parse(parser)?.value())
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        LitBool::skip(cursor)
    }
}

impl Parse for LitVerbatim {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Verbatim(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Literal(Lit::Verbatim(value))) => {
                let value = value.clone();
                parser.advance();
                Ok(value)
            }
            _ => Err(parser.expected("verbatim literal")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}
