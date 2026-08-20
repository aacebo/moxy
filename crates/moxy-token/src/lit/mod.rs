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
    Int(Int),
    UInt(UInt),
    Float(Float),
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
        Lit::Str(LitStr::new(value, Span::default()))
    }

    #[inline]
    pub fn char(value: char) -> Self {
        Lit::Char(LitChar::new(value, Span::default()))
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
            _ => Lit::Verbatim(LitVerbatim::new(repr, span)),
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
        LitUsize::new(value, true, Span::default()).into()
    }

    pub fn usize_unsuffixed(value: usize) -> Self {
        LitUsize::new(value, false, Span::default()).into()
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
        LitIsize::new(value, true, Span::default()).into()
    }

    pub fn isize_unsuffixed(value: isize) -> Self {
        LitIsize::new(value, false, Span::default()).into()
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
            Lit::Int(v) => v.repr(),
            Lit::UInt(v) => v.repr(),
            Lit::Float(v) => v.repr(),
            Lit::Str(v) => v.repr(),
            Lit::ByteStr(v) => v.repr(),
            Lit::CStr(v) => v.repr(),
            Lit::Char(v) => v.repr(),
            Lit::Byte(v) => v.repr(),
            Lit::Bool(v) => v.repr(),
            Lit::Verbatim(v) => v.repr(),
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        match self {
            Lit::Int(v) => v.span(),
            Lit::UInt(v) => v.span(),
            Lit::Float(v) => v.span(),
            Lit::Str(v) => v.span(),
            Lit::ByteStr(v) => v.span(),
            Lit::CStr(v) => v.span(),
            Lit::Char(v) => v.span(),
            Lit::Byte(v) => v.span(),
            Lit::Bool(v) => v.span(),
            Lit::Verbatim(v) => v.span(),
        }
    }

    #[inline]
    pub fn set_span(&mut self, span: Span) {
        match self {
            Lit::Int(v) => v.set_span(span),
            Lit::UInt(v) => v.set_span(span),
            Lit::Float(v) => v.set_span(span),
            Lit::Str(v) => v.set_span(span),
            Lit::ByteStr(v) => v.set_span(span),
            Lit::CStr(v) => v.set_span(span),
            Lit::Char(v) => v.set_span(span),
            Lit::Byte(v) => v.set_span(span),
            Lit::Bool(v) => v.set_span(span),
            Lit::Verbatim(v) => v.set_span(span),
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
        matches!(self, Lit::Int(_) | Lit::UInt(_))
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self, Lit::Float(_))
    }

    #[inline]
    pub fn is_str(&self) -> bool {
        matches!(self, Lit::Str(_))
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        matches!(self, Lit::Bool(_))
    }

    #[inline]
    pub fn as_str(&self) -> Option<&LitStr> {
        if let Lit::Str(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn as_bool(&self) -> Option<&LitBool> {
        if let Lit::Bool(v) = self { Some(v) } else { None }
    }

    /// The integer value as `u64`, when this is an unsigned int, or a non-negative
    /// signed int that fits.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Lit::UInt(v) => u64::try_from(v.as_u128()).ok(),
            Lit::Int(v) => u64::try_from(v.as_i128()).ok(),
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
            return Ok((end, Lit::ByteStr(v)));
        }
        if let Ok((end, v)) = LitCStr::scan(cursor) {
            return Ok((end, Lit::CStr(v)));
        }
        if let Ok((end, v)) = LitByte::scan(cursor) {
            return Ok((end, Lit::Byte(v)));
        }
        if let Ok((end, v)) = LitStr::scan(cursor) {
            return Ok((end, Lit::Str(v)));
        }
        if let Ok((end, v)) = LitChar::scan(cursor) {
            return Ok((end, Lit::Char(v)));
        }

        // Numbers: UInt (u*-suffixed) → Float (f*/fractional) → Int (i*/unsuffixed default).
        if let Ok((end, v)) = UInt::scan(cursor) {
            return Ok((end, Lit::UInt(v)));
        }
        if let Ok((end, v)) = Float::scan(cursor) {
            return Ok((end, Lit::Float(v)));
        }
        if let Ok((end, v)) = Int::scan(cursor) {
            return Ok((end, Lit::Int(v)));
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::ToTokenStream;

    fn lit(src: &str) -> Lit {
        let ts = TokenStream::from_str(src).unwrap();
        Lit::parse(&mut ts.parse()).unwrap()
    }

    #[test]
    fn roundtrip_is_lossless() {
        for src in [
            "0xFF",
            "1_000u64",
            "0o17",
            "0b1010",
            "1.5e3f64",
            "r#\"a\"b\"#",
            "b\"by\\x00\"",
            "'\\n'",
            "c\"x\"",
            "\"a\\tb\"",
            "42",
            "1.5",
            "true",
            "false",
        ] {
            assert_eq!(lit(src).to_token_stream().to_string(), src, "roundtrip {src}");
        }
    }

    #[test]
    fn classifies_and_decodes() {
        assert!(matches!(lit("42"), Lit::Int(Int::I32(v)) if v.value() == 42 && !v.suffixed()));
        assert!(matches!(lit("42u64"), Lit::UInt(UInt::U64(v)) if v.value() == 42 && v.suffixed()));
        assert!(matches!(lit("0xFF"), Lit::Int(Int::I32(v)) if v.value() == 255));
        assert!(matches!(lit("1_000"), Lit::Int(Int::I32(v)) if v.value() == 1000));
        assert!(matches!(lit("1.5f32"), Lit::Float(Float::F32(v)) if v.value() == 1.5 && v.suffixed()));
        assert!(matches!(lit("1.5"), Lit::Float(Float::F64(v)) if v.value() == 1.5 && !v.suffixed()));
        assert!(matches!(lit("'\\n'"), Lit::Char(v) if v.value() == '\n'));
        assert!(matches!(lit("b'x'"), Lit::Byte(v) if v.value() == b'x'));
    }

    #[test]
    fn decodes_string_escapes() {
        assert_eq!(lit("\"a\\tb\"").as_str().unwrap().value(), "a\tb");
        assert_eq!(lit("r#\"a\\tb\"#").as_str().unwrap().value(), "a\\tb");
    }

    #[test]
    fn bool_lexes_as_literal() {
        assert!(matches!(lit("true"), Lit::Bool(v) if v.value()));
        assert!(matches!(lit("false"), Lit::Bool(v) if !v.value()));
    }

    #[test]
    fn eq_ignores_span() {
        let a = Lit::u32_unsuffixed(7);
        let mut b = Lit::u32_unsuffixed(7);
        b.set_span(Span::call_site());
        assert_eq!(a, b);
    }

    #[test]
    fn from_chains() {
        let leaf = LitI8::new(5, false, Span::default());
        assert!(matches!(Int::from(leaf.clone()), Int::I8(_)));
        assert!(matches!(Lit::from(leaf), Lit::Int(Int::I8(_))));
    }

    #[test]
    fn from_repr_verbatim_fallback() {
        let v = Lit::from_repr("@@@", Span::default());
        assert!(matches!(v, Lit::Verbatim(_)));
        assert_eq!(v.to_token_stream().to_string(), "@@@");
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn integer_serializes_as_string() {
            assert_eq!(serde_json::to_value(lit("42")).unwrap(), serde_json::json!("42"));
        }

        #[test]
        fn string_serializes_with_quotes() {
            assert_eq!(serde_json::to_value(lit("\"hi\"")).unwrap(), serde_json::json!("\"hi\""));
        }
    }
}
