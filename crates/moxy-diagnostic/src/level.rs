#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Unknown,
    Note,
    Help,
    Warning,
    Error,
}

impl Level {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub fn is_note(&self) -> bool {
        matches!(self, Self::Note)
    }

    pub fn is_help(&self) -> bool {
        matches!(self, Self::Help)
    }

    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warning)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
            Self::Unknown => "??",
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(nightly)]
impl From<Level> for proc_macro::Level {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::Error,
            Level::Warning => Self::Warning,
            Level::Note => Self::Note,
            Level::Help => Self::Help,
            Level::Unknown => Self::Note,
        }
    }
}

#[cfg(nightly)]
impl From<proc_macro::Level> for Level {
    fn from(value: proc_macro::Level) -> Self {
        match value {
            proc_macro::Level::Error => Self::Error,
            proc_macro::Level::Warning => Self::Warning,
            proc_macro::Level::Note => Self::Note,
            proc_macro::Level::Help => Self::Help,
            _ => Self::Unknown,
        }
    }
}
