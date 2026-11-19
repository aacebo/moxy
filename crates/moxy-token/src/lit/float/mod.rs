mod f32;
mod f64;

pub use f32::*;
pub use f64::*;

use crate::lex::{Cursor, LexError, Scan};
use crate::{Lit, Span, Spanner, ToTokens, TokenStream, TokenTree};

/// A parsed Rust float literal token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum LitFloat {
    F32(LitF32),
    F64(LitF64),
}

impl LitFloat {
    #[inline]
    pub fn repr(&self) -> &str {
        match self {
            Self::F32(v) => v.repr(),
            Self::F64(v) => v.repr(),
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        match self {
            Self::F32(v) => v.span(),
            Self::F64(v) => v.span(),
        }
    }

    #[inline]
    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::F32(v) => v.set_span(span),
            Self::F64(v) => v.set_span(span),
        }
    }

    /// The value widened to `f64`.
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::F32(v) => v.value() as f64,
            Self::F64(v) => v.value(),
        }
    }

    /// Build a float literal from a scanned `repr`. Accepts an `f32`/`f64` suffix or an
    /// unsuffixed fractional/exponent form (→ `F64`). Returns `None` otherwise.
    pub(crate) fn from_repr(repr: &str, span: Span) -> Option<Self> {
        let (digits, suffix) = split_suffix(repr);

        if !is_float(digits, suffix) {
            return None;
        }

        let suffixed = !suffix.is_empty();

        match suffix {
            "f32" => {
                let value = digits.replace('_', "").parse::<f32>().ok()?;
                Some(Self::F32(LitF32::from_parts(value, suffixed, repr, span)))
            }
            "f64" | "" => {
                let value = digits.replace('_', "").parse::<f64>().ok()?;
                Some(Self::F64(LitF64::from_parts(value, suffixed, repr, span)))
            }
            _ => None,
        }
    }
}

impl LitFloat {
    /// Splits a float of a digit run, a dot, and an optional digit run, as in
    /// `x.0.1` and `x.0. 1`, into a tuple index, the dot, and the second tuple
    /// index if there is one, each with its part of the span. The compiler
    /// lexes `0.1` and `0.` there as one float literal.
    #[doc(hidden)]
    pub fn split_tuple_index(&self) -> Option<(crate::LitInt, crate::Dot, Option<crate::LitInt>)> {
        let repr = self.repr();
        let (head, tail) = repr.split_once('.')?;
        let digits = |text: &str| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit());

        if !digits(head) || !(tail.is_empty() || digits(tail)) {
            return None;
        }

        let (head_span, rest) = self.span().split(head.len());
        let (dot_span, tail_span) = rest.split(1);
        let mut first: crate::LitInt = head.parse().ok()?;
        first.set_span(head_span);
        let second = if tail.is_empty() {
            None
        } else {
            let mut second: crate::LitInt = tail.parse().ok()?;
            second.set_span(tail_span);
            Some(second)
        };

        Some((first, crate::Dot::new(dot_span), second))
    }
}

impl Scan for LitFloat {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let end = scan_number(cursor)?;
        let len = end.offset() as usize - cursor.offset() as usize;
        let repr = &cursor.rest()[..len];
        let span = cursor.span_to(&end);

        match Self::from_repr(repr, span) {
            Some(float) => Ok((end, float)),
            None => cursor.error().into(),
        }
    }
}

impl Spanner for LitFloat {
    fn span(&self) -> Span {
        self.span()
    }
}

impl ToTokens for LitFloat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(TokenTree::Literal(self.clone().into()));
    }
}

impl std::fmt::Display for LitFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.repr())
    }
}

impl From<LitFloat> for Lit {
    fn from(value: LitFloat) -> Self {
        Self::Float(value)
    }
}

#[cfg(feature = "serde")]
impl From<LitFloat> for String {
    fn from(value: LitFloat) -> Self {
        value.repr().to_string()
    }
}

// Each constructor panics if the value is not finite; see `LitF64::new`
impl Lit {
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
}

/// Split a numeric repr into its `digits` body and a trailing type suffix.
fn split_suffix(repr: &str) -> (&str, &str) {
    const SUFFIXES: [&str; 12] = [
        "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize", "f32", "f64",
    ];

    for s in SUFFIXES {
        if let Some(body) = repr.strip_suffix(s) {
            if (s == "f32" || s == "f64") && (body.starts_with("0x") || body.starts_with("0X")) {
                continue;
            }
            return (body, s);
        }
    }

    (repr, "")
}

/// `true` when the repr (after suffix split) is a float form.
fn is_float(digits: &str, suffix: &str) -> bool {
    suffix.starts_with('f')
        || (!digits.starts_with("0x")
            && !digits.starts_with("0o")
            && !digits.starts_with("0b")
            && (digits.contains('.') || digits.contains('e') || digits.contains('E')))
}

pub(crate) fn scan_number(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    let first = c.first().ok_or(c.error())?;

    if !first.is_ascii_digit() {
        return c.error().into();
    }

    let mut cur = c;

    if first == '0' {
        let next = cur.advance();

        match next.first() {
            Some('x' | 'X') => {
                cur = digits(next.advance(), |ch| ch.is_ascii_hexdigit())?;
                return Ok(suffix(cur));
            }
            Some('o' | 'O') => {
                cur = digits(next.advance(), |ch| matches!(ch, '0'..='7'))?;
                return radix_end(cur, 8);
            }
            Some('b' | 'B') => {
                cur = digits(next.advance(), |ch| matches!(ch, '0' | '1'))?;
                return radix_end(cur, 2);
            }
            _ => {}
        }
    }

    cur = digits(cur, |ch| ch.is_ascii_digit())?;

    if cur.starts_with(".") {
        let after_dot = cur.advance();

        match after_dot.first() {
            Some(ch) if ch.is_ascii_digit() => cur = digits_opt(after_dot, |ch| ch.is_ascii_digit()),
            // `1..2`, `1.max(2)`, and `x.0._a` keep the dot out of the literal
            Some(ch) if ch == '.' || ch == '_' || ch.is_alphabetic() => {}
            // A trailing dot, as in `3.` or `(x / 3.)`, ends a float literal
            _ => return Ok(after_dot),
        }
    }

    if let Some('e' | 'E') = cur.first() {
        cur = cur.advance();

        if let Some('+' | '-') = cur.first() {
            cur = cur.advance();
        }

        // As in rustc, even in macro input: `1e` and `1em` are errors
        cur = digits(cur, |ch| ch.is_ascii_digit()).map_err(|e| e.message("expected at least one digit in exponent"))?;
    }

    Ok(suffix(cur))
}

fn digits(c: Cursor<'_>, pred: fn(char) -> bool) -> Result<Cursor<'_>, LexError> {
    let mut cur = c;
    let mut found = false;

    loop {
        match cur.first() {
            Some('_') => cur = cur.advance(),
            Some(ch) if pred(ch) => {
                found = true;
                cur = cur.advance_by(ch.len_utf8());
            }
            _ => break,
        }
    }

    if !found {
        return c.error().into();
    }

    Ok(cur)
}

fn digits_opt(c: Cursor<'_>, pred: fn(char) -> bool) -> Cursor<'_> {
    digits(c, pred).unwrap_or(c)
}

fn suffix(c: Cursor<'_>) -> Cursor<'_> {
    match c.first() {
        Some(ch) if ch == '_' || unicode_ident::is_xid_start(ch) => {
            c.advance_by(ch.len_utf8()).skip_while(unicode_ident::is_xid_continue)
        }
        _ => c,
    }
}

// The end of a binary or octal literal, where a decimal digit is an invalid
// digit of the literal, as in `0b12`, rather than a second token
fn radix_end(cursor: Cursor<'_>, radix: u32) -> Result<Cursor<'_>, LexError> {
    match cursor.first() {
        Some(ch) if ch.is_ascii_digit() => cursor
            .error()
            .message(format!("invalid digit for a base {radix} literal"))
            .into(),
        _ => Ok(suffix(cursor)),
    }
}
