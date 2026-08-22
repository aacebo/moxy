mod channel;
mod error;

#[doc(inline)]
pub use channel::*;
#[doc(inline)]
pub use error::*;

/// Reads the version of the installed rustc by invoking it.
///
/// The rustc binary is taken from the `RUSTC` environment variable, falling
/// back to `"rustc"`. Returns `None` if rustc cannot be run or its output
/// cannot be parsed.
pub fn read() -> Result<Version, Box<dyn std::error::Error>> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = std::process::Command::new(rustc).arg("--verbose").arg("--version").output()?;

    if !output.status.success() {
        return Err(ParseVersionError.into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    stdout.parse()
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}

impl Version {
    /// The `(major, minor, patch)` triple, ignoring channel.
    #[inline]
    pub fn triple(&self) -> (u16, u16, u16) {
        (self.major, self.minor, self.patch)
    }

    /// True if this version is at least `other` (compares the semver triple only).
    #[inline]
    pub fn at_least(&self, other: &Self) -> bool {
        self.triple() >= other.triple()
    }
}

impl std::str::FromStr for Version {
    type Err = Box<dyn std::error::Error>;

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        let version = if version.contains("release:") {
            version
                .lines()
                .find_map(|line| line.trim().strip_prefix("release:"))
                .ok_or(ParseVersionError)?
                .trim()
        } else {
            version.trim()
        };

        let token = if version.starts_with("rustc ") {
            version.split_whitespace().nth(1).ok_or(ParseVersionError)?
        } else {
            version.split_whitespace().next().ok_or(ParseVersionError)?
        };

        let channel = token.parse()?;
        let mut mmp = [0u16; 3];
        let triple = token.split('-').next().unwrap_or("");

        for (slot, part) in mmp.iter_mut().zip(triple.split('.')) {
            *slot = part.parse::<u16>()?;
        }

        Ok(Self {
            major: mmp[0],
            minor: mmp[1],
            patch: mmp[2],
            channel,
        })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

        if self.channel != Channel::Stable {
            write!(f, "-{}", self.channel)?;
        }

        Ok(())
    }
}
