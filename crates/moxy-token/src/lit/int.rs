use crate::lex::Cursor;
use crate::lit::Lit;
use crate::parser::{ParseError, ParseStream};
use crate::{LexError, Parse, Scan, Span, Spanner};

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub struct LitInt {
    span: Span,
    repr: Box<str>,
    radix: Radix,
    suffix: IntSuffix,
    value: u128,
}

impl LitInt {
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn repr(&self) -> &str {
        &self.repr
    }

    #[inline]
    pub fn radix(&self) -> Radix {
        self.radix
    }

    #[inline]
    pub fn suffix(&self) -> IntSuffix {
        self.suffix
    }

    #[inline]
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    #[inline]
    pub fn value(&self) -> u128 {
        self.value
    }
}

impl PartialEq for LitInt {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for LitInt {}

impl std::hash::Hash for LitInt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for LitInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr)
    }
}

impl std::str::FromStr for LitInt {
    type Err = LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cursor = Cursor::new(s, 0);
        let (_, value) = Self::scan(cursor)?;
        Ok(value)
    }
}

impl Spanner for LitInt {
    fn span(&self) -> Span {
        self.span
    }
}

impl Scan for LitInt {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let end = scan(cursor)?;
        let len = end.offset() as usize - cursor.offset() as usize;
        let repr = &cursor.rest()[..len];
        let span = cursor.span_to(&end);
        let stripped_repr = repr.replace("_", "");
        let (stripped_repr, suffix) = IntSuffix::split(&stripped_repr).map_err(|msg| cursor.error().message(msg))?;
        let (radix, stripped_repr) = Radix::split(stripped_repr).map_err(|msg| cursor.error().message(msg))?;
        let value = u128::from_str_radix(stripped_repr, radix.into()).map_err(|err| cursor.error().message(err))?;

        Ok((
            end,
            Self {
                span,
                repr: repr.into(),
                radix,
                suffix,
                value,
            },
        ))
    }
}

impl Parse for LitInt {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match stream.parse::<Lit>()? {
            Lit::Int(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected integer literal").into()),
        }
    }
}

impl From<LitInt> for Lit {
    fn from(value: LitInt) -> Self {
        Self::Int(value)
    }
}

#[cfg(feature = "serde")]
impl From<LitInt> for String {
    fn from(value: LitInt) -> Self {
        value.repr.into_string()
    }
}

impl Lit {
    pub fn u8_suffixed(value: u8) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::U8,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn u8_unsuffixed(value: u8) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn u16_suffixed(value: u16) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::U16,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn u16_unsuffixed(value: u16) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn u32_suffixed(value: u32) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::U32,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn u32_unsuffixed(value: u32) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn u64_suffixed(value: u64) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::U64,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn u64_unsuffixed(value: u64) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn usize_suffixed(value: usize) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::USize,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn usize_unsuffixed(value: usize) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn i8_suffixed(value: i8) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::I8,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn i8_unsuffixed(value: i8) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn i16_suffixed(value: i16) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::I16,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn i16_unsuffixed(value: i16) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn i32_suffixed(value: i32) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::I32,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn i32_unsuffixed(value: i32) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn i64_suffixed(value: i64) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::I64,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn i64_unsuffixed(value: i64) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn isize_suffixed(value: isize) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            suffix: IntSuffix::ISize,
            value: value as u128,
            ..Default::default()
        })
    }

    pub fn isize_unsuffixed(value: isize) -> Self {
        Self::Int(LitInt {
            repr: value.to_string().into(),
            value: value as u128,
            ..Default::default()
        })
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(u8)]
pub enum Radix {
    Binary = 2,
    Octal = 8,
    #[default]
    Decimal = 10,
    Hex = 16,
}

impl Radix {
    #[inline]
    pub fn split(repr: &str) -> Result<(Self, &str), String> {
        const PREFIXES: [&str; 6] = ["0b", "0B", "0o", "0O", "0x", "0X"];

        for s in PREFIXES {
            if let Some(body) = repr.strip_prefix(s) {
                return Ok((s.parse()?, body));
            }
        }

        Ok((Self::Decimal, repr))
    }

    #[inline]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}

impl From<Radix> for u8 {
    fn from(value: Radix) -> Self {
        value.to_u8()
    }
}

impl From<Radix> for u32 {
    fn from(value: Radix) -> Self {
        value as u32
    }
}

impl std::str::FromStr for Radix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0b" | "0B" => Ok(Self::Binary),
            "0o" | "0O" => Ok(Self::Octal),
            "0x" | "0X" => Ok(Self::Hex),
            "" => Ok(Self::Decimal),
            v => Err(format!("expected integer literal radix, received {v}")),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum IntSuffix {
    #[default]
    None,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

impl IntSuffix {
    #[inline]
    pub fn split(repr: &str) -> Result<(&str, Self), String> {
        const SUFFIXES: [&str; 12] = [
            "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
        ];

        for s in SUFFIXES {
            if let Some(body) = repr.strip_suffix(s) {
                return Ok((body, s.parse()?));
            }
        }

        Ok((repr, Self::None))
    }

    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::ISize => "isize",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::USize => "usize",
        }
    }
}

impl std::fmt::Display for IntSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for IntSuffix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::None),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "i128" => Ok(Self::I128),
            "isize" => Ok(Self::ISize),
            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "u128" => Ok(Self::U128),
            "usize" => Ok(Self::USize),
            v => Err(format!("expected integer literal suffix, received {v}")),
        }
    }
}

fn scan(c: Cursor<'_>) -> Result<Cursor<'_>, LexError> {
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
