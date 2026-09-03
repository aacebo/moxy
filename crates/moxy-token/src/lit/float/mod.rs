mod f32;
mod f64;

pub use f32::*;
pub use f64::*;

use crate::lex::{Cursor, LexError, Scan};
use crate::{Lit, Span, Spanner};

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

impl std::fmt::Display for LitFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.repr())
    }
}

#[cfg(feature = "serde")]
impl From<LitFloat> for String {
    fn from(value: LitFloat) -> Self {
        value.repr().to_string()
    }
}

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

fn scan_number(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
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
                return Ok(suffix(cur));
            }
            Some('b' | 'B') => {
                cur = digits(next.advance(), |ch| matches!(ch, '0' | '1'))?;
                return Ok(suffix(cur));
            }
            _ => {}
        }
    }

    cur = digits(cur, |ch| ch.is_ascii_digit())?;

    if cur.starts_with(".") {
        let after_dot = cur.advance();

        if let Some(ch) = after_dot.first() {
            if ch.is_ascii_digit() {
                cur = digits_opt(after_dot, |ch| ch.is_ascii_digit());
            }
        }
    }

    if let Some('e' | 'E') = cur.first() {
        cur = cur.advance();

        if let Some('+' | '-') = cur.first() {
            cur = cur.advance();
        }

        cur = digits(cur, |ch| ch.is_ascii_digit())?;
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
