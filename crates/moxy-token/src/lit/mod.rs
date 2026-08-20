use crate::lex::{Cursor, LexError, Scan};
use crate::parser::{ParseError, ParseStream};
use crate::{Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

pub mod float;
pub mod int;
pub mod uint;

mod r#bool;
mod byte;
mod byte_str;
mod c_str;
mod char;
mod str;
mod verbatim;

pub use r#bool::*;
pub use byte::*;
pub use byte_str::*;
pub use c_str::*;
pub use char::*;
pub use float::*;
pub use int::*;
pub use str::*;
pub use uint::*;
pub use verbatim::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub enum Lit {
    Int(LitInt),
    UInt(LitUInt),
    Float(LitFloat),
    Str(LitStr),
    ByteStr(LitByteStr),
    CStr(LitCStr),
    Char(LitChar),
    Byte(LitByte),
    Bool(LitBool),
    Verbatim(LitVerbatim),
}

impl Lit {
    #[inline]
    pub fn string(value: &str) -> Self {
        Self::Str(LitStr::new(value, Span::default()))
    }

    #[inline]
    pub fn char(value: char) -> Self {
        Self::Char(LitChar::new(value, Span::default()))
    }

    /// Classify and decode an arbitrary literal repr into the matching variant. Used by
    /// the proc-macro bridges, which only have the source text. Falls back to
    /// [`LitVerbatim`] for anything that doesn't lex as a known literal.
    pub fn from_repr(repr: &str, span: Span) -> Self {
        use std::str::FromStr;

        match TokenStream::from_str(repr).ok().and_then(|ts| ts.into_iter().next()) {
            Some(TokenTree::Literal(mut lit)) if lit.repr() == repr => {
                lit.set_span(span);
                lit
            }
            _ => Self::Verbatim(LitVerbatim::new(repr, span)),
        }
    }

    pub fn u8_suffixed(value: u8) -> Self {
        LitU8::new(value, true, Span::default()).into()
    }

    pub fn u8_unsuffixed(value: u8) -> Self {
        LitU8::new(value, false, Span::default()).into()
    }

    pub fn u16_suffixed(value: u16) -> Self {
        LitU16::new(value, true, Span::default()).into()
    }

    pub fn u16_unsuffixed(value: u16) -> Self {
        LitU16::new(value, false, Span::default()).into()
    }

    pub fn u32_suffixed(value: u32) -> Self {
        LitU32::new(value, true, Span::default()).into()
    }

    pub fn u32_unsuffixed(value: u32) -> Self {
        LitU32::new(value, false, Span::default()).into()
    }

    pub fn u64_suffixed(value: u64) -> Self {
        LitU64::new(value, true, Span::default()).into()
    }

    pub fn u64_unsuffixed(value: u64) -> Self {
        LitU64::new(value, false, Span::default()).into()
    }

    pub fn usize_suffixed(value: usize) -> Self {
        LitUSize::new(value, true, Span::default()).into()
    }

    pub fn usize_unsuffixed(value: usize) -> Self {
        LitUSize::new(value, false, Span::default()).into()
    }

    pub fn i8_suffixed(value: i8) -> Self {
        LitI8::new(value, true, Span::default()).into()
    }

    pub fn i8_unsuffixed(value: i8) -> Self {
        LitI8::new(value, false, Span::default()).into()
    }

    pub fn i16_suffixed(value: i16) -> Self {
        LitI16::new(value, true, Span::default()).into()
    }

    pub fn i16_unsuffixed(value: i16) -> Self {
        LitI16::new(value, false, Span::default()).into()
    }

    pub fn i32_suffixed(value: i32) -> Self {
        LitI32::new(value, true, Span::default()).into()
    }

    pub fn i32_unsuffixed(value: i32) -> Self {
        LitI32::new(value, false, Span::default()).into()
    }

    pub fn i64_suffixed(value: i64) -> Self {
        LitI64::new(value, true, Span::default()).into()
    }

    pub fn i64_unsuffixed(value: i64) -> Self {
        LitI64::new(value, false, Span::default()).into()
    }

    pub fn isize_suffixed(value: isize) -> Self {
        LitISize::new(value, true, Span::default()).into()
    }

    pub fn isize_unsuffixed(value: isize) -> Self {
        LitISize::new(value, false, Span::default()).into()
    }

    pub fn f32_suffixed(value: f32) -> Self {
        LitF32::new(value, true, Span::default()).into()
    }

    pub fn f32_unsuffixed(value: f32) -> Self {
        LitF32::new(value, false, Span::default()).into()
    }

    pub fn f64_suffixed(value: f64) -> Self {
        LitF64::new(value, true, Span::default()).into()
    }

    pub fn f64_unsuffixed(value: f64) -> Self {
        LitF64::new(value, false, Span::default()).into()
    }

    #[inline]
    pub fn repr(&self) -> &str {
        match self {
            Self::Int(v) => v.repr(),
            Self::UInt(v) => v.repr(),
            Self::Float(v) => v.repr(),
            Self::Str(v) => v.repr(),
            Self::ByteStr(v) => v.repr(),
            Self::CStr(v) => v.repr(),
            Self::Char(v) => v.repr(),
            Self::Byte(v) => v.repr(),
            Self::Bool(v) => v.repr(),
            Self::Verbatim(v) => v.repr(),
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        match self {
            Self::Int(v) => v.span(),
            Self::UInt(v) => v.span(),
            Self::Float(v) => v.span(),
            Self::Str(v) => v.span(),
            Self::ByteStr(v) => v.span(),
            Self::CStr(v) => v.span(),
            Self::Char(v) => v.span(),
            Self::Byte(v) => v.span(),
            Self::Bool(v) => v.span(),
            Self::Verbatim(v) => v.span(),
        }
    }

    #[inline]
    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Int(v) => v.set_span(span),
            Self::UInt(v) => v.set_span(span),
            Self::Float(v) => v.set_span(span),
            Self::Str(v) => v.set_span(span),
            Self::ByteStr(v) => v.set_span(span),
            Self::CStr(v) => v.set_span(span),
            Self::Char(v) => v.set_span(span),
            Self::Byte(v) => v.set_span(span),
            Self::Bool(v) => v.set_span(span),
            Self::Verbatim(v) => v.set_span(span),
        }
    }

    #[inline]
    pub fn to_token_tree(&self) -> TokenTree {
        TokenTree::Literal(self.clone())
    }

    #[inline]
    pub fn into_token_tree(self) -> TokenTree {
        TokenTree::Literal(self)
    }

    #[inline]
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_) | Self::UInt(_))
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    #[inline]
    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    #[inline]
    pub fn as_str(&self) -> Option<&LitStr> {
        if let Self::Str(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn as_bool(&self) -> Option<&LitBool> {
        if let Self::Bool(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn as_int(&self) -> Option<&LitInt> {
        if let Self::Int(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn as_float(&self) -> Option<&LitFloat> {
        if let Self::Float(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn as_uint(&self) -> Option<&LitUInt> {
        if let Self::UInt(v) = self { Some(v) } else { None }
    }

    /// The integer value as `u64`, when this is an unsigned int, or a non-negative
    /// signed int that fits.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::UInt(v) => u64::try_from(v.as_u128()).ok(),
            Self::Int(v) => u64::try_from(v.as_i128()).ok(),
            _ => None,
        }
    }
}

impl Spanner for Lit {
    fn span(&self) -> Span {
        self.span()
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.repr())
    }
}

#[cfg(feature = "serde")]
impl From<Lit> for String {
    fn from(value: Lit) -> Self {
        value.repr().to_string()
    }
}

impl Scan for Lit {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        // Strings / chars / bytes first — their prefix bytes (b c r ' ") are unambiguous.
        if let Ok((end, v)) = LitByteStr::scan(cursor) {
            return Ok((end, Self::ByteStr(v)));
        }

        if let Ok((end, v)) = LitCStr::scan(cursor) {
            return Ok((end, Self::CStr(v)));
        }

        if let Ok((end, v)) = LitByte::scan(cursor) {
            return Ok((end, Self::Byte(v)));
        }

        if let Ok((end, v)) = LitStr::scan(cursor) {
            return Ok((end, Self::Str(v)));
        }

        if let Ok((end, v)) = LitChar::scan(cursor) {
            return Ok((end, Self::Char(v)));
        }

        // Numbers: LitUInt (u*-suffixed) → LitFloat (f*/fractional) → LitInt (i*/unsuffixed default).
        if let Ok((end, v)) = LitUInt::scan(cursor) {
            return Ok((end, Self::UInt(v)));
        }

        if let Ok((end, v)) = LitFloat::scan(cursor) {
            return Ok((end, Self::Float(v)));
        }

        if let Ok((end, v)) = LitInt::scan(cursor) {
            return Ok((end, Self::Int(v)));
        }

        cursor.error().into()
    }
}

impl ToTokens for Lit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(TokenTree::Literal(self.clone()));
    }
}

impl Parse for Lit {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.advance() {
            Some(TokenTree::Literal(v)) => Ok(v.clone()),
            _ => Err(LexError::new(stream.span()).message("expected Literal").into()),
        }
    }
}
