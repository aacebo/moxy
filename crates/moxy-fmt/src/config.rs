#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FmtConfig {
    pub indent: Indent,
    pub newline: NewlineStyle,
    pub max_width: usize,
}

impl FmtConfig {
    pub fn with_indent(mut self, indent: Indent) -> Self {
        self.indent = indent;
        self
    }

    pub fn with_newline(mut self, newline: NewlineStyle) -> Self {
        self.newline = newline;
        self
    }

    pub fn with_max_width(mut self, max_width: usize) -> Self {
        self.max_width = max_width;
        self
    }
}

impl Default for FmtConfig {
    fn default() -> Self {
        Self {
            indent: Indent::default(),
            newline: NewlineStyle::default(),
            max_width: 80,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "value", rename_all = "lowercase")
)]
pub enum Indent {
    Tab(usize),
    Space(usize),
}

impl Indent {
    pub fn tab(width: usize) -> Self {
        Self::Tab(width)
    }

    pub fn space(width: usize) -> Self {
        Self::Space(width)
    }

    pub fn width(&self) -> usize {
        match self {
            Self::Tab(v) => *v,
            Self::Space(v) => *v,
        }
    }

    pub fn spaces(&self) -> usize {
        match self {
            Self::Tab(v) => v * 4,
            Self::Space(v) => *v,
        }
    }
}

impl Default for Indent {
    fn default() -> Self {
        Self::Tab(1)
    }
}

impl std::fmt::Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tab(v) => write!(f, "{}", "\t".repeat(*v)),
            Self::Space(v) => write!(f, "{}", " ".repeat(*v)),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum NewlineStyle {
    #[default]
    Auto,
    Unix,
    Windows,
}

impl NewlineStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Unix => "unix",
            Self::Windows => "windows",
        }
    }
}

impl std::fmt::Display for NewlineStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unix => write!(f, "\n"),
            Self::Windows => write!(f, "\r\n"),
            Self::Auto => {
                if cfg!(unix) {
                    write!(f, "\n")
                } else {
                    write!(f, "\r\n")
                }
            }
        }
    }
}
