#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ParseVersionError;

impl std::fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not parse rustc version")
    }
}

impl std::error::Error for ParseVersionError {}

impl std::str::FromStr for super::Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = if s.contains("release:") {
            Self::parse_verbose(s)
        } else {
            Self::parse(s)
        };

        parsed.ok_or(ParseVersionError)
    }
}

impl std::fmt::Display for super::Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

        if self.channel != super::Channel::Stable {
            write!(f, "-{}", self.channel)?;
        }

        Ok(())
    }
}
