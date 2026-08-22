use crate::rustc::version::ParseVersionError;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Channel {
    Dev,
    Nightly,
    Beta,
    #[default]
    Stable,
}

impl Channel {
    pub fn is_dev(self) -> bool {
        matches!(self, Self::Dev)
    }

    pub fn is_nightly(self) -> bool {
        matches!(self, Self::Nightly)
    }

    pub fn is_beta(self) -> bool {
        matches!(self, Self::Beta)
    }

    pub fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Nightly => "nightly",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Channel {
    type Err = Box<dyn std::error::Error>;

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        let version = version.trim();

        if version.contains("-dev") || version == "dev" {
            Ok(Self::Dev)
        } else if version.contains("-nightly") || version == "nightly" {
            Ok(Self::Nightly)
        } else if version.contains("-beta") || version == "beta" {
            Ok(Self::Beta)
        } else if !version.contains('-') {
            Ok(Self::Stable)
        } else {
            Err(ParseVersionError.into())
        }
    }
}
