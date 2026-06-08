#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "lowercase"))]
pub enum Delim {
    #[default]
    None,
    Paren,
    Brace,
    Bracket,
}

impl Delim {
    #[inline]
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '(' | ')' => Some(Self::Paren),
            '[' | ']' => Some(Self::Bracket),
            '{' | '}' => Some(Self::Brace),
            _ => None,
        }
    }

    #[inline]
    pub fn from_open(ch: char) -> Option<Self> {
        match ch {
            '(' => Some(Self::Paren),
            '[' => Some(Self::Bracket),
            '{' => Some(Self::Brace),
            _ => None,
        }
    }

    #[inline]
    pub fn from_close(ch: char) -> Option<Self> {
        match ch {
            ')' => Some(Self::Paren),
            ']' => Some(Self::Bracket),
            '}' => Some(Self::Brace),
            _ => None,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Paren => "paren",
            Self::Brace => "brace",
            Self::Bracket => "bracket",
            Self::None => "none",
        }
    }

    #[inline]
    pub fn open(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Brace => '{',
            Self::Bracket => '[',
            Self::Paren => '(',
        }
    }

    #[inline]
    pub fn close(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Brace => '}',
            Self::Bracket => ']',
            Self::Paren => ')',
        }
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    #[inline]
    pub fn is_brace(&self) -> bool {
        matches!(self, Self::Brace)
    }

    #[inline]
    pub fn is_bracket(&self) -> bool {
        matches!(self, Self::Bracket)
    }

    #[inline]
    pub fn is_paren(&self) -> bool {
        matches!(self, Self::Paren)
    }
}
