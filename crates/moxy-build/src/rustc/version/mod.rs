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
pub fn read() -> Option<Version> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = std::process::Command::new(rustc)
        .arg("--verbose")
        .arg("--version")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;

    // Verbose output exposes a `release:` line; fall back to the terse
    // first line if it is somehow absent.
    Version::parse_verbose(&stdout).or_else(|| Version::parse(stdout.lines().next().unwrap_or("")))
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}

impl Version {
    pub fn parse(version: &str) -> Option<Self> {
        let version = version.trim();
        let token = if version.starts_with("rustc ") {
            version.split_whitespace().nth(1)?
        } else {
            version.split_whitespace().next()?
        };

        let channel = Channel::parse(token)?;
        let mut mmp = [0u16; 3];
        let triple = token.split('-').next().unwrap_or("");

        for (slot, part) in mmp.iter_mut().zip(triple.split('.')) {
            *slot = part.parse::<u16>().ok()?;
        }

        Some(Self {
            major: mmp[0],
            minor: mmp[1],
            patch: mmp[2],
            channel,
        })
    }

    pub fn parse_verbose(output: &str) -> Option<Self> {
        let release = output.lines().find_map(|line| line.trim().strip_prefix("release:"))?;
        Self::parse(release)
    }

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
