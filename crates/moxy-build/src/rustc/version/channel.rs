#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Channel {
    Dev,
    Nightly,
    Beta,
    #[default]
    Stable,
}

impl Channel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Nightly => "nightly",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    /// Parses the channel from a version string such as `1.96.0-nightly`.
    ///
    /// Stable releases are identified by the absence of a `-` suffix; the
    /// pre-release channels are identified by their suffix (or bare name).
    pub fn parse(version: &str) -> Option<Self> {
        let version = version.trim();

        if version.contains("-dev") || version == "dev" {
            Some(Self::Dev)
        } else if version.contains("-nightly") || version == "nightly" {
            Some(Self::Nightly)
        } else if version.contains("-beta") || version == "beta" {
            Some(Self::Beta)
        } else if !version.contains('-') {
            Some(Self::Stable)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nightly() {
        assert_eq!(Channel::parse("1.96.0-nightly"), Some(Channel::Nightly));
    }

    #[test]
    fn parse_beta() {
        assert_eq!(Channel::parse("1.74.0-beta"), Some(Channel::Beta));
    }

    #[test]
    fn parse_dev() {
        assert_eq!(Channel::parse("1.0.0-dev"), Some(Channel::Dev));
    }

    #[test]
    fn parse_stable() {
        assert_eq!(Channel::parse("1.96.0"), Some(Channel::Stable));
    }

    #[test]
    fn parse_bare_words() {
        assert_eq!(Channel::parse("nightly"), Some(Channel::Nightly));
        assert_eq!(Channel::parse("beta"), Some(Channel::Beta));
        assert_eq!(Channel::parse("dev"), Some(Channel::Dev));
    }

    #[test]
    fn parse_unknown_suffix() {
        assert_eq!(Channel::parse("1.0.0-foo"), None);
    }
}
