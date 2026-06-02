use crate::Span;
use crate::token::lex::{Cursor, LexError};

#[derive(Debug, Clone)]
pub struct Literal {
    pub(crate) repr: Box<str>,
    pub(crate) span: Span,
}

macro_rules! lit_constructor {
    ($name:ident, $ty:ty, $fmt:expr) => {
        pub fn $name(value: $ty) -> Self {
            Self {
                repr: format!($fmt, value).into_boxed_str(),
                span: Span::default(),
            }
        }
    };
}

impl Literal {
    pub fn string(value: &str) -> Self {
        Self {
            repr: format!("{:?}", value).into_boxed_str(),
            span: Span::default(),
        }
    }

    pub fn character(value: char) -> Self {
        Self {
            repr: format!("{:?}", value).into_boxed_str(),
            span: Span::default(),
        }
    }

    lit_constructor!(u8_suffixed, u8, "{}u8");
    lit_constructor!(u16_suffixed, u16, "{}u16");
    lit_constructor!(u32_suffixed, u32, "{}u32");
    lit_constructor!(u64_suffixed, u64, "{}u64");
    lit_constructor!(usize_suffixed, usize, "{}usize");
    lit_constructor!(i8_suffixed, i8, "{}i8");
    lit_constructor!(i16_suffixed, i16, "{}i16");
    lit_constructor!(i32_suffixed, i32, "{}i32");
    lit_constructor!(i64_suffixed, i64, "{}i64");
    lit_constructor!(isize_suffixed, isize, "{}isize");
    lit_constructor!(f32_suffixed, f32, "{}f32");
    lit_constructor!(f64_suffixed, f64, "{}f64");
    lit_constructor!(u8_unsuffixed, u8, "{}");
    lit_constructor!(u16_unsuffixed, u16, "{}");
    lit_constructor!(u32_unsuffixed, u32, "{}");
    lit_constructor!(u64_unsuffixed, u64, "{}");
    lit_constructor!(usize_unsuffixed, usize, "{}");
    lit_constructor!(i8_unsuffixed, i8, "{}");
    lit_constructor!(i16_unsuffixed, i16, "{}");
    lit_constructor!(i32_unsuffixed, i32, "{}");
    lit_constructor!(i64_unsuffixed, i64, "{}");
    lit_constructor!(isize_unsuffixed, isize, "{}");
    lit_constructor!(f32_unsuffixed, f32, "{}");
    lit_constructor!(f64_unsuffixed, f64, "{}");

    pub fn from_repr(repr: &str, span: Span) -> Self {
        Self { repr: repr.into(), span }
    }

    pub fn repr(&self) -> &str {
        &self.repr
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl From<proc_macro::Literal> for Literal {
    fn from(value: proc_macro::Literal) -> Self {
        Self {
            repr: value.to_string().into_boxed_str(),
            span: value.span().into(),
        }
    }
}

impl From<Literal> for proc_macro::Literal {
    fn from(value: Literal) -> Self {
        value
            .repr
            .parse()
            .unwrap_or_else(|_| proc_macro::Literal::string(&value.repr))
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr)
    }
}

impl crate::token::lex::Scan for Literal {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let start = cursor;
        let end = scan_string(cursor)
            .or_else(|_| scan_byte_string(cursor))
            .or_else(|_| scan_raw_string(cursor))
            .or_else(|_| scan_raw_byte_string(cursor))
            .or_else(|_| scan_c_string(cursor))
            .or_else(|_| scan_raw_c_string(cursor))
            .or_else(|_| scan_byte_char(cursor))
            .or_else(|_| scan_character(cursor))
            .or_else(|_| scan_number(cursor))?;

        let len = end.offset() as usize - start.offset() as usize;
        let repr = start.rest()[..len].to_string().into_boxed_str();
        let span = start.span_to(&end);
        Ok((end, Self { repr, span }))
    }
}

impl crate::token::ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut crate::TokenStream) {
        tokens.extend_one(crate::Token::Literal(self.clone()).into());
    }
}

impl crate::Parse for Literal {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Literal(v))) => Ok(v.clone()),
            _ => Err(crate::token::lex::LexError::new(stream.span())
                .message("expected Literal")
                .into()),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Literal {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.repr.serialize(s)
    }
}

// --- Internal scanning helpers ---

fn scan_string(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    if !c.starts_with("\"") {
        return c.error().into();
    }

    cooked_string(c.advance(1))
}

fn scan_byte_string(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    if !c.starts_with("b\"") {
        return c.error().into();
    }

    cooked_string(c.advance(2))
}

fn scan_c_string(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    if !c.starts_with("c\"") {
        return c.error().into();
    }

    cooked_string(c.advance(2))
}

fn cooked_string(mut c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    loop {
        match c.first() {
            None => return c.error().into(),
            Some('"') => return Ok(c.advance(1)),
            Some('\\') => c = escape(c.advance(1))?,
            Some(ch) => c = c.advance(ch.len_utf8()),
        }
    }
}

fn scan_raw_string(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    if !c.starts_with("r") {
        return c.error().into();
    }

    raw_string_inner(c, c.advance(1))
}

fn scan_raw_byte_string(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    if !c.starts_with("br") {
        return c.error().into();
    }

    raw_string_inner(c, c.advance(2))
}

fn scan_raw_c_string(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    if !c.starts_with("cr") {
        return c.error().into();
    }

    raw_string_inner(c, c.advance(2))
}

fn raw_string_inner<'a>(start: Cursor<'a>, c: Cursor<'a>) -> Result<Cursor<'a>, LexError> {
    let mut hashes = 0u32;
    let mut cur = c;

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

        let ch = cur.first().unwrap();
        cur = cur.advance(ch.len_utf8());
    }
}

fn scan_character(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    if !c.starts_with("'") {
        return c.error().into();
    }

    let start = c;
    let c = c.advance(1);
    let c = match c.first() {
        None | Some('\'') => return start.error().into(),
        Some('\\') => escape(c.advance(1))?,
        Some(ch) => c.advance(ch.len_utf8()),
    };

    if !c.starts_with("'") {
        return start.error().into();
    }

    Ok(suffix(c.advance(1)))
}

fn scan_byte_char(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
    if !c.starts_with("b'") {
        return c.error().into();
    }

    let start = c;
    let c = c.advance(2);
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

    // Float: decimal point
    if cur.starts_with(".") {
        let after_dot = cur.advance(1);

        match after_dot.first() {
            Some(ch) if ch.is_ascii_digit() => {
                cur = after_dot;
                cur = digits_opt(cur, |ch| ch.is_ascii_digit());
            }
            _ => {}
        }
    }

    // Float: exponent
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

fn suffix(c: Cursor<'_>) -> Cursor<'_> {
    match c.first() {
        Some(ch) if ch == '_' || unicode_ident::is_xid_start(ch) => {
            c.advance(ch.len_utf8()).skip_while(unicode_ident::is_xid_continue)
        }
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    mod serde {
        use std::str::FromStr;

        use crate::TokenStream;
        use crate::token::Literal;

        #[test]
        fn integer_literal_serializes_as_string() {
            let ts = TokenStream::from_str("42").unwrap();
            let lit = ts.parse().parse::<Literal>().unwrap();
            assert_eq!(serde_json::to_value(&lit).unwrap(), serde_json::json!("42"));
        }

        #[test]
        fn string_literal_serializes_with_quotes() {
            let ts = TokenStream::from_str("\"hi\"").unwrap();
            let lit = ts.parse().parse::<Literal>().unwrap();
            assert_eq!(serde_json::to_value(&lit).unwrap(), serde_json::json!("\"hi\""));
        }
    }
}
