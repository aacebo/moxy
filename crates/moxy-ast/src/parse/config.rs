#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParseConfig {
    pub trace: bool,
}

pub enum Ansi {
    Blue,
    Green,
    Red,
    Reset,
}

impl std::fmt::Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blue => write!(f, "\x1b[34m"),
            Self::Green => write!(f, "\x1b[32m"),
            Self::Red => write!(f, "\x1b[31m"),
            Self::Reset => write!(f, "\x1b[0m"),
        }
    }
}
