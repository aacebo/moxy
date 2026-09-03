use crate::lex::{Cursor, LexError, Scan};
use crate::lit::Lit;
use crate::{Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitChar {
    value: char,
    repr: Box<str>,
    span: Span,
}

impl LitChar {
    #[inline]
    pub fn new(value: char, span: Span) -> Self {
        Self {
            value,
            repr: format!("{value:?}").into_boxed_str(),
            span,
        }
    }

    #[inline]
    pub(crate) fn from_parts(value: char, repr: &str, span: Span) -> Self {
        Self {
            value,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> char {
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

impl PartialEq for LitChar {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for LitChar {}

impl std::hash::Hash for LitChar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for LitChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitChar {
    fn span(&self) -> Span {
        self.span
    }
}

impl Scan for LitChar {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let end = scan_quoted(cursor, "'")?;
        let len = end.offset() as usize - cursor.offset() as usize;
        let repr = &cursor.rest()[..len];
        let span = cursor.span_to(&end);

        let inner = repr.strip_prefix('\'').and_then(|r| r.strip_suffix('\''));
        match inner.and_then(decode_one_char) {
            Some(value) => Ok((end, Self::from_parts(value, repr, span))),
            None => cursor.error().into(),
        }
    }
}

impl From<LitChar> for Lit {
    fn from(value: LitChar) -> Self {
        Self::Char(value)
    }
}

#[cfg(feature = "serde")]
impl From<LitChar> for String {
    fn from(value: LitChar) -> Self {
        value.repr.into_string()
    }
}

fn scan_quoted<'a>(c: Cursor<'a>, open: &str) -> Result<Cursor<'a>, LexError> {
    if !c.starts_with(open) {
        return c.error().into();
    }

    let start = c;
    let c = c.advance(open.len());
    let c = match c.first() {
        None | Some('\'') => return start.error().into(),
        Some('\\') => escape(c.advance(1))?,
        Some(ch) => c.advance(ch.len_utf8()),
    };

    if !c.starts_with("'") {
        return start.error().into();
    }

    Ok(c.advance(1))
}

fn escape(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    match c.first() {
        None => c.error().into(),
        Some('n' | 'r' | 't' | '\\' | '\'' | '"' | '0') => Ok(c.advance(1)),
        Some('x') => {
            let c = c.advance(1);
            let c = hex_digit(c)?;
            hex_digit(c)
        }
        Some('u') => {
            let c = c.advance(1);

            if !c.starts_with("{") {
                return c.error().into();
            }

            let mut c = c.advance(1);
            let mut count = 0;

            loop {
                match c.first() {
                    Some('}') if count > 0 => return Ok(c.advance(1)),
                    Some(ch) if ch.is_ascii_hexdigit() && count < 6 => {
                        count += 1;
                        c = c.advance(1);
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
        Some(ch) if ch.is_ascii_hexdigit() => Ok(c.advance(1)),
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
