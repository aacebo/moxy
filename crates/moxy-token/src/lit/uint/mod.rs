mod u128;
mod u16;
mod u32;
mod u64;
mod u8;
mod usize;

pub use u8::*;
pub use u16::*;
pub use u32::*;
pub use u64::*;
pub use u128::*;
pub use usize::*;

use crate::lex::{Cursor, LexError, Scan};
use crate::{Span, Spanner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum LitUInt {
    U8(LitU8),
    U16(LitU16),
    U32(LitU32),
    U64(LitU64),
    U128(LitU128),
    USize(LitUSize),
}

impl LitUInt {
    #[inline]
    pub fn repr(&self) -> &str {
        match self {
            Self::U8(v) => v.repr(),
            Self::U16(v) => v.repr(),
            Self::U32(v) => v.repr(),
            Self::U64(v) => v.repr(),
            Self::U128(v) => v.repr(),
            Self::USize(v) => v.repr(),
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        match self {
            Self::U8(v) => v.span(),
            Self::U16(v) => v.span(),
            Self::U32(v) => v.span(),
            Self::U64(v) => v.span(),
            Self::U128(v) => v.span(),
            Self::USize(v) => v.span(),
        }
    }

    #[inline]
    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::U8(v) => v.set_span(span),
            Self::U16(v) => v.set_span(span),
            Self::U32(v) => v.set_span(span),
            Self::U64(v) => v.set_span(span),
            Self::U128(v) => v.set_span(span),
            Self::USize(v) => v.set_span(span),
        }
    }

    #[inline]
    pub fn suffixed(&self) -> bool {
        match self {
            Self::U8(v) => v.suffixed(),
            Self::U16(v) => v.suffixed(),
            Self::U32(v) => v.suffixed(),
            Self::U64(v) => v.suffixed(),
            Self::U128(v) => v.suffixed(),
            Self::USize(v) => v.suffixed(),
        }
    }

    /// The value widened to `u128` (all unsigned int variants fit).
    pub fn as_u128(&self) -> u128 {
        match self {
            Self::U8(v) => v.value() as u128,
            Self::U16(v) => v.value() as u128,
            Self::U32(v) => v.value() as u128,
            Self::U64(v) => v.value() as u128,
            Self::U128(v) => v.value(),
            Self::USize(v) => v.value() as u128,
        }
    }

    /// Build an unsigned-integer literal from a scanned `repr`. Only `u*`-suffixed reprs
    /// are unsigned; returns `None` otherwise (unsuffixed ints belong to [`super::LitInt`]).
    pub(crate) fn from_repr(repr: &str, span: Span) -> Option<Self> {
        let (digits, suffix) = split_suffix(repr);

        if is_float(digits, suffix) {
            return None;
        }

        let magnitude = parse_int_u128(digits)?;
        let suffixed = true;

        match suffix {
            "u8" => Some(Self::U8(LitU8::from_parts(
                u8::try_from(magnitude).ok()?,
                suffixed,
                repr,
                span,
            ))),
            "u16" => Some(Self::U16(LitU16::from_parts(
                u16::try_from(magnitude).ok()?,
                suffixed,
                repr,
                span,
            ))),
            "u32" => Some(Self::U32(LitU32::from_parts(
                u32::try_from(magnitude).ok()?,
                suffixed,
                repr,
                span,
            ))),
            "u64" => Some(Self::U64(LitU64::from_parts(
                u64::try_from(magnitude).ok()?,
                suffixed,
                repr,
                span,
            ))),
            "u128" => Some(Self::U128(LitU128::from_parts(
                u128::try_from(magnitude).ok()?,
                suffixed,
                repr,
                span,
            ))),
            "usize" => Some(Self::USize(LitUSize::from_parts(
                usize::try_from(magnitude).ok()?,
                suffixed,
                repr,
                span,
            ))),
            _ => None,
        }
    }
}

impl Scan for LitUInt {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let end = scan_number(cursor)?;
        let len = end.offset() as usize - cursor.offset() as usize;
        let repr = &cursor.rest()[..len];
        let span = cursor.span_to(&end);

        match Self::from_repr(repr, span) {
            Some(uint) => Ok((end, uint)),
            None => cursor.error().into(),
        }
    }
}

impl Spanner for LitUInt {
    fn span(&self) -> Span {
        self.span()
    }
}

impl std::fmt::Display for LitUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.repr())
    }
}

#[cfg(feature = "serde")]
impl From<LitUInt> for String {
    fn from(value: LitUInt) -> Self {
        value.repr().to_string()
    }
}

/// Split a numeric repr into its `digits` body and a trailing type suffix.
fn split_suffix(repr: &str) -> (&str, &str) {
    const SUFFIXES: [&str; 14] = [
        "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32", "f64",
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

/// Parse the integer digit body into a `u128`, applying base prefix and stripping `_`.
fn parse_int_u128(digits: &str) -> Option<u128> {
    let (radix, body) = if let Some(rest) = digits.strip_prefix("0x").or_else(|| digits.strip_prefix("0X")) {
        (16, rest)
    } else if let Some(rest) = digits.strip_prefix("0o").or_else(|| digits.strip_prefix("0O")) {
        (8, rest)
    } else if let Some(rest) = digits.strip_prefix("0b").or_else(|| digits.strip_prefix("0B")) {
        (2, rest)
    } else {
        (10, digits)
    };

    u128::from_str_radix(&body.replace('_', ""), radix).ok()
}

fn scan_number(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    let first = c.first().ok_or(c.error())?;

    if !first.is_ascii_digit() {
        return c.error().into();
    }

    let mut cur = c;

    if first == '0' {
        let next = cur.advance(1);

        match next.first() {
            Some('x' | 'X') => {
                cur = digits(next.advance(1), |ch| ch.is_ascii_hexdigit())?;
                return Ok(suffix(cur));
            }
            Some('o' | 'O') => {
                cur = digits(next.advance(1), |ch| matches!(ch, '0'..='7'))?;
                return Ok(suffix(cur));
            }
            Some('b' | 'B') => {
                cur = digits(next.advance(1), |ch| matches!(ch, '0' | '1'))?;
                return Ok(suffix(cur));
            }
            _ => {}
        }
    }

    cur = digits(cur, |ch| ch.is_ascii_digit())?;

    if cur.starts_with(".") {
        let after_dot = cur.advance(1);

        if let Some(ch) = after_dot.first() {
            if ch.is_ascii_digit() {
                cur = digits_opt(after_dot, |ch| ch.is_ascii_digit());
            }
        }
    }

    if let Some('e' | 'E') = cur.first() {
        cur = cur.advance(1);

        if let Some('+' | '-') = cur.first() {
            cur = cur.advance(1);
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
            Some('_') => cur = cur.advance(1),
            Some(ch) if pred(ch) => {
                found = true;
                cur = cur.advance(ch.len_utf8());
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
            c.advance(ch.len_utf8()).skip_while(unicode_ident::is_xid_continue)
        }
        _ => c,
    }
}
