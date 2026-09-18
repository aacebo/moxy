use crate::lex::{Cursor, LexError, Scan};
use crate::lit::Lit;
use crate::{Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitByte {
    value: u8,
    repr: Box<str>,
    span: Span,
}

impl LitByte {
    #[inline]
    pub(crate) fn from_parts(value: u8, repr: &str, span: Span) -> Self {
        Self {
            value,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> u8 {
        self.value
    }

    #[inline]
    pub fn repr(&self) -> &str {
        &self.repr
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl PartialEq for LitByte {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for LitByte {}

impl std::hash::Hash for LitByte {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for LitByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitByte {
    fn span(&self) -> Span {
        self.span
    }
}

impl Scan for LitByte {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let end = scan(cursor, "b'")?;
        let len = end.offset() as usize - cursor.offset() as usize;
        let repr = &cursor.rest()[..len];
        let span = cursor.span_to(&end);
        let inner = repr.strip_prefix("b'").and_then(|r| r.strip_suffix('\''));

        match inner.and_then(decode_one_char) {
            Some(value) => Ok((end, Self::from_parts(value as u8, repr, span))),
            _ => cursor.error().into(),
        }
    }
}

impl From<LitByte> for Lit {
    fn from(value: LitByte) -> Self {
        Self::Byte(value)
    }
}

#[cfg(feature = "serde")]
impl From<LitByte> for String {
    fn from(value: LitByte) -> Self {
        value.repr.into_string()
    }
}

fn scan<'a>(c: Cursor<'a>, open: &str) -> Result<Cursor<'a>, LexError> {
    if !c.starts_with(open) {
        return c.error().into();
    }

    let start = c;
    let c = c.advance_by(open.len());
    let c = match c.first() {
        None | Some('\'') => return start.error().into(),
        Some('\\') => escape(c.advance())?,
        Some(ch) => c.advance_by(ch.len_utf8()),
    };

    if !c.starts_with("'") {
        return start.error().into();
    }

    Ok(c.advance())
}

fn escape(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    match c.first() {
        None => c.error().into(),
        Some('n' | 'r' | 't' | '\\' | '\'' | '"' | '0') => Ok(c.advance()),
        Some('x') => {
            let c = c.advance();
            let c = hex_digit(c)?;
            hex_digit(c)
        }
        Some('u') => {
            let c = c.advance();

            if !c.starts_with("{") {
                return c.error().into();
            }

            let mut c = c.advance();
            let mut count = 0;

            loop {
                match c.first() {
                    Some('}') if count > 0 => return Ok(c.advance()),
                    Some(ch) if ch.is_ascii_hexdigit() && count < 6 => {
                        count += 1;
                        c = c.advance();
                    }
                    _ => return c.error().into(),
                }
            }
        }
        _ => c.error().into(),
    }
}

fn hex_digit(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    match c.first() {
        Some(ch) if ch.is_ascii_hexdigit() => Ok(c.advance()),
        _ => c.error().into(),
    }
}

/// Decode a single (possibly escaped) char from a char-literal inner body.
fn decode_one_char(inner: &str) -> Option<char> {
    let mut chars = inner.chars();
    let value = match chars.next()? {
        '\\' => decode_escape(&mut chars)?,
        c => c,
    };

    if chars.next().is_some() {
        return None;
    }

    Some(value)
}

fn decode_escape(chars: &mut std::str::Chars<'_>) -> Option<char> {
    match chars.next()? {
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        '\\' => Some('\\'),
        '\'' => Some('\''),
        '"' => Some('"'),
        '0' => Some('\0'),
        'x' => {
            let hi = chars.next()?.to_digit(16)?;
            let lo = chars.next()?.to_digit(16)?;
            char::from_u32(hi * 16 + lo)
        }
        'u' => {
            if chars.next()? != '{' {
                return None;
            }

            let mut value = 0u32;
            let mut count = 0;

            for c in chars.by_ref() {
                if c == '}' {
                    break;
                }

                value = value.checked_mul(16)?.checked_add(c.to_digit(16)?)?;
                count += 1;

                if count > 6 {
                    return None;
                }
            }

            char::from_u32(value)
        }
        _ => None,
    }
}
