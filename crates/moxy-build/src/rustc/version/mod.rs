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
    pub fn parse(version: &str) -> Option<Version> {
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

        Some(Version {
            major: mmp[0],
            minor: mmp[1],
            patch: mmp[2],
            channel,
        })
    }

    pub fn parse_verbose(output: &str) -> Option<Version> {
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
    pub fn at_least(&self, other: &Version) -> bool {
        self.triple() >= other.triple()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse_terse_nightly() {
        let v = Version::parse("rustc 1.96.0-nightly (b90dc1e59 2026-03-04)").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 96);
        assert_eq!(v.patch, 0);
        assert_eq!(v.channel, Channel::Nightly);
    }

    #[test]
    fn parse_terse_stable() {
        let v = Version::parse("rustc 1.80.1 (abcdef 2024-08-08)").unwrap();
        assert_eq!((v.major, v.minor, v.patch), (1, 80, 1));
        assert_eq!(v.channel, Channel::Stable);
    }

    #[test]
    fn parse_bare_token() {
        let v = Version::parse("1.74.0-beta").unwrap();
        assert_eq!((v.major, v.minor, v.patch), (1, 74, 0));
        assert_eq!(v.channel, Channel::Beta);
    }

    #[test]
    fn parse_verbose_block() {
        let output = "\
rustc 1.96.0-nightly (b90dc1e59 2026-03-04)
binary: rustc
commit-hash: b90dc1e597db0bbc0cab0eccb39747b1a9d7e607
commit-date: 2026-03-04
host: aarch64-apple-darwin
release: 1.96.0-nightly
LLVM version: 22.1.0";

        let v = Version::parse_verbose(output).unwrap();
        assert_eq!((v.major, v.minor, v.patch), (1, 96, 0));
        assert_eq!(v.channel, Channel::Nightly);
    }

    #[test]
    fn parse_partial_components() {
        let v = Version::parse("1.80").unwrap();
        assert_eq!((v.major, v.minor, v.patch), (1, 80, 0));
        assert_eq!(v.channel, Channel::Stable);
    }

    #[test]
    fn parse_garbage_is_none() {
        assert!(Version::parse("not a version").is_none());
        assert!(Version::parse("1.x.0").is_none());
    }

    #[test]
    fn from_str_dispatches() {
        let terse: Version = "rustc 1.80.1 (abcdef 2024-08-08)".parse().unwrap();
        assert_eq!((terse.major, terse.minor, terse.patch), (1, 80, 1));

        let verbose: Version = "release: 1.96.0-nightly".parse().unwrap();
        assert_eq!(verbose.channel, Channel::Nightly);

        assert!(Version::from_str("garbage").is_err());
    }

    #[test]
    fn display_round_trip() {
        let stable = Version {
            major: 1,
            minor: 80,
            patch: 1,
            channel: Channel::Stable,
        };
        assert_eq!(stable.to_string(), "1.80.1");

        let nightly = Version {
            major: 1,
            minor: 96,
            patch: 0,
            channel: Channel::Nightly,
        };
        assert_eq!(nightly.to_string(), "1.96.0-nightly");
    }

    #[test]
    fn at_least_compares_triple() {
        let v = Version::parse("1.80.1").unwrap();
        assert!(v.at_least(&Version::parse("1.80.0").unwrap()));
        assert!(v.at_least(&Version::parse("1.80.1").unwrap()));
        assert!(!v.at_least(&Version::parse("1.81.0").unwrap()));
        assert!(!Version::parse("1.79.0").unwrap().at_least(&Version::parse("1.80.0").unwrap()));

        // channel is ignored
        assert!(
            Version::parse("1.80.0-nightly")
                .unwrap()
                .at_least(&Version::parse("1.80.0").unwrap())
        );
    }
}
