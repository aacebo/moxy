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
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '(' | ')' => Some(Self::Paren),
            '[' | ']' => Some(Self::Bracket),
            '{' | '}' => Some(Self::Brace),
            _ => None,
        }
    }

    pub fn from_open(ch: char) -> Option<Self> {
        match ch {
            '(' => Some(Self::Paren),
            '[' => Some(Self::Bracket),
            '{' => Some(Self::Brace),
            _ => None,
        }
    }

    pub fn from_close(ch: char) -> Option<Self> {
        match ch {
            ')' => Some(Self::Paren),
            ']' => Some(Self::Bracket),
            '}' => Some(Self::Brace),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Paren => "paren",
            Self::Brace => "brace",
            Self::Bracket => "bracket",
            Self::None => "none",
        }
    }

    pub fn open(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Brace => '{',
            Self::Bracket => '[',
            Self::Paren => '(',
        }
    }

    pub fn close(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Brace => '}',
            Self::Bracket => ']',
            Self::Paren => ')',
        }
    }
}

impl From<proc_macro::Delimiter> for Delim {
    fn from(value: proc_macro::Delimiter) -> Self {
        match value {
            proc_macro::Delimiter::Parenthesis => Self::Paren,
            proc_macro::Delimiter::Brace => Self::Brace,
            proc_macro::Delimiter::Bracket => Self::Bracket,
            proc_macro::Delimiter::None => Self::None,
        }
    }
}

impl From<Delim> for proc_macro::Delimiter {
    fn from(value: Delim) -> Self {
        match value {
            Delim::Paren => proc_macro::Delimiter::Parenthesis,
            Delim::Brace => proc_macro::Delimiter::Brace,
            Delim::Bracket => proc_macro::Delimiter::Bracket,
            Delim::None => proc_macro::Delimiter::None,
        }
    }
}
