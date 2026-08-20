use crate::lex::{Cursor, LexError, Scan};
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{Parse, Span, Spanner};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitStr {
    value: String,
    repr: Box<str>,
    span: Span,
}

impl LitStr {
    #[inline]
    pub fn new(value: &str, span: Span) -> Self {
        Self {
            value: value.to_string(),
            repr: format!("{value:?}").into_boxed_str(),
            span,
        }
    }

    #[inline]
    pub(crate) fn from_parts(value: String, repr: &str, span: Span) -> Self {
        Self {
            value,
            repr: repr.into(),
            span,
        }
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
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

impl PartialEq for LitStr {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for LitStr {}

impl std::hash::Hash for LitStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for LitStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Spanner for LitStr {
    fn span(&self) -> Span {
        self.span
    }
}

impl Scan for LitStr {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let end = scan_cooked(cursor, "\"").or_else(|_| scan_raw(cursor, "r"))?;
        let len = end.offset() as usize - cursor.offset() as usize;
        let repr = &cursor.rest()[..len];
        let span = cursor.span_to(&end);

        match decode_string_body(repr) {
            Some(value) => Ok((end, Self::from_parts(value, repr, span))),
            None => cursor.error().into(),
        }
    }
}

impl Parse for LitStr {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::Str(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected string literal").into()),
        }
    }
}

impl From<LitStr> for Lit {
    fn from(value: LitStr) -> Self {
        Self::Str(value)
    }
}

#[cfg(feature = "serde")]
impl From<LitStr> for String {
    fn from(value: LitStr) -> Self {
        value.repr.into_string()
    }
}

fn scan_cooked<'a>(c: Cursor<'a>, open: &str) -> Result<Cursor<'a>, LexError> {
    if !c.starts_with(open) {
        return c.error().into();
    }

    let mut c = c.advance(open.len());

    loop {
        match c.first() {
            None => return c.error().into(),
            Some('"') => return Ok(c.advance(1)),
            Some('\\') => c = escape(c.advance(1))?,
            Some(ch) => c = c.advance(ch.len_utf8()),
        }
    }
}

fn scan_raw<'a>(start: Cursor<'a>, open: &str) -> Result<Cursor<'a>, LexError> {
    if !start.starts_with(open) {
        return start.error().into();
    }

    let mut cur = start.advance(open.len());
    let mut hashes = 0u32;

    while cur.starts_with("#") {
        hashes += 1;
        cur = cur.advance(1);
    }

    if !cur.starts_with("\"") {
        return start.error().into();
    }

    cur = cur.advance(1);

    let closing: String = std::iter::once('"')
        .chain(std::iter::repeat_n('#', hashes as usize))
        .collect();

    loop {
        if cur.is_empty() {
            return start.error().into();
        }

        if cur.starts_with(&closing) {
            return Ok(cur.advance(closing.len()));
        }

        if let Some(ch) = cur.first() {
            cur = cur.advance(ch.len_utf8());
        } else {
            return start.error().into();
        }
    }
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

/// Decode the body of a string repr (after any `b`/`c` prefix the caller strips).
/// Handles cooked (`"…"`) and raw (`r"…"` / `r#"…"#`) forms.
fn decode_string_body(repr: &str) -> Option<String> {
    if let Some(rest) = repr.strip_prefix('r') {
        let hashes = rest.bytes().take_while(|b| *b == b'#').count();
        let open = 1 + hashes;
        let inner = &rest[open..rest.len() - open];
        return Some(inner.to_string());
    }

    let inner = repr.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut chars = inner.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            out.push(decode_escape(&mut chars)?);
        } else {
            out.push(c);
        }
    }

    Some(out)
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
