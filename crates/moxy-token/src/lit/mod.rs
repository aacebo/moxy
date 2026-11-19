use crate::lex::{Cursor, LexError, Scan};
use crate::{Span, Spanner, ToTokens, TokenStream, TokenTree};

mod r#bool;
mod byte;
mod byte_str;
mod c_str;
mod char;
mod float;
mod int;
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
pub use verbatim::*;

/// A parsed Rust  literal token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub enum Lit {
    Int(LitInt),
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
    pub fn string(value: &str) -> Self {
        Self::Str(LitStr::new(value, Span::default()))
    }

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

    #[inline]
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
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
    pub fn is_c_str(&self) -> bool {
        matches!(self, Self::CStr(_))
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    #[inline]
    pub fn is_char(&self) -> bool {
        matches!(self, Self::Char(_))
    }

    #[inline]
    pub fn is_byte(&self) -> bool {
        matches!(self, Self::Byte(_))
    }

    #[inline]
    pub fn is_byte_str(&self) -> bool {
        matches!(self, Self::ByteStr(_))
    }

    #[inline]
    pub fn is_verbatim(&self) -> bool {
        matches!(self, Self::Verbatim(_))
    }

    #[inline]
    pub fn as_str(&self) -> Option<&LitStr> {
        if let Self::Str(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn as_c_str(&self) -> Option<&LitCStr> {
        if let Self::CStr(v) = self { Some(v) } else { None }
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
    pub fn as_char(&self) -> Option<&LitChar> {
        if let Self::Char(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn as_byte(&self) -> Option<&LitByte> {
        if let Self::Byte(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn as_byte_str(&self) -> Option<&LitByteStr> {
        if let Self::ByteStr(v) = self { Some(v) } else { None }
    }

    #[inline]
    pub fn to_token_tree(&self) -> TokenTree {
        TokenTree::Literal(self.clone())
    }

    #[inline]
    pub fn into_token_tree(self) -> TokenTree {
        TokenTree::Literal(self)
    }

    pub fn repr(&self) -> &str {
        match self {
            Self::Int(v) => v.repr(),
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

    pub fn span(&self) -> Span {
        match self {
            Self::Int(v) => v.span(),
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

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Int(v) => v.set_span(span),
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
        let (end, lit) = Self::scan_unsuffixed(cursor)?;

        // A suffix on a quoted literal, as in `"a"suffix`, which the compiler
        // lexes as part of the literal; numbers scan their own suffixes
        if !matches!(lit, Self::Int(_) | Self::Float(_) | Self::Bool(_) | Self::Verbatim(_))
            && let Some(ch) = end.first()
            && (ch == '_' || unicode_ident::is_xid_start(ch))
        {
            let suffixed = end.advance_by(ch.len_utf8()).skip_while(unicode_ident::is_xid_continue);
            let len = suffixed.offset() as usize - cursor.offset() as usize;
            let verbatim = LitVerbatim::new(&cursor.rest()[..len], cursor.span_to(&suffixed));
            return Ok((suffixed, Self::Verbatim(verbatim)));
        }

        Ok((end, lit))
    }
}

impl Lit {
    fn scan_unsuffixed(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        if let Ok((end, v)) = LitBool::scan(cursor) {
            return Ok((end, Self::Bool(v)));
        }

        // A quote after a literal prefix always starts that literal, so its
        // error, such as an invalid escape, is the error
        if cursor.starts_with("b'") {
            return LitByte::scan(cursor).map(|(end, v)| (end, Self::Byte(v)));
        }

        if cursor.starts_with("b\"") || cursor.starts_with("br\"") || cursor.starts_with("br#") {
            return LitByteStr::scan(cursor).map(|(end, v)| (end, Self::ByteStr(v)));
        }

        if cursor.starts_with("c\"") || cursor.starts_with("cr\"") || cursor.starts_with("cr#") {
            return LitCStr::scan(cursor).map(|(end, v)| (end, Self::CStr(v)));
        }

        if cursor.starts_with("\"") || cursor.starts_with("r\"") || cursor.starts_with("r#\"") || cursor.starts_with("r##") {
            return LitStr::scan(cursor).map(|(end, v)| (end, Self::Str(v)));
        }

        // A quote and a backslash start a character literal, never a lifetime
        if cursor.starts_with("'\\") {
            return LitChar::scan(cursor).map(|(end, v)| (end, Self::Char(v)));
        }

        if let Ok((end, v)) = LitChar::scan(cursor) {
            return Ok((end, Self::Char(v)));
        }

        if let Ok((end, v)) = LitFloat::scan(cursor) {
            return Ok((end, Self::Float(v)));
        }

        if let Ok((end, v)) = LitInt::scan(cursor) {
            return Ok((end, Self::Int(v)));
        }

        // A number the typed literals cannot hold keeps its text, as the
        // compiler's lexer does: `1.0f16`, `1f128`, a custom suffix as in `2b`,
        // or an integer past `u128::MAX`
        if cursor.first().is_some_and(|ch| ch.is_ascii_digit()) {
            let end = float::scan_number(cursor)?;
            let len = end.offset() as usize - cursor.offset() as usize;
            return Ok((
                end,
                Self::Verbatim(LitVerbatim::new(&cursor.rest()[..len], cursor.span_to(&end))),
            ));
        }

        cursor.error().into()
    }
}

impl ToTokens for Lit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(TokenTree::Literal(self.clone()));
    }
}
