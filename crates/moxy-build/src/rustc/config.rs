use crate::rustc::instruction::{Cfg, Instruction, Link, Rerun};
use crate::rustc::version::{self, Version};

/// Accumulates cargo build-script instructions for a `build.rs`, with helpers
/// for emitting diagnostics and asserting a minimum rustc version.
///
/// Build instructions, warnings and errors are collected, then flushed to
/// stdout in one go via [`Config::emit`].
#[derive(Debug, Clone, Default)]
pub struct Config {
    version: Option<Version>,
    instructions: Vec<Instruction>,
}

impl Config {
    /// Creates a config that auto-detects the installed rustc version (via
    /// [`Version::read`]).
    pub fn new() -> Self {
        Self {
            version: version::read(),
            instructions: Vec::new(),
        }
    }

    /// Creates a config with an explicit rustc version, without spawning rustc.
    pub fn with_version(version: Version) -> Self {
        Self {
            version: Some(version),
            instructions: Vec::new(),
        }
    }

    /// The detected (or injected) rustc version, if any.
    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    /// The accumulated instructions, in insertion order.
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    /// Pushes a raw [`Instruction`] — escape hatch for variants without a
    /// dedicated helper.
    pub fn push(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }

    /// `cargo::rustc-cfg=<key>`
    pub fn cfg(&mut self, key: impl Into<String>) -> &mut Self {
        self.push(Instruction::Cfg(Cfg::Set(key.into())))
    }

    /// `cargo::rustc-check-cfg=<spec>`
    pub fn check_cfg(&mut self, spec: impl Into<String>) -> &mut Self {
        self.push(Instruction::Cfg(Cfg::Check(spec.into())))
    }

    /// `cargo::rustc-env=<key>=<value>`
    pub fn env(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.push(Instruction::Env(key.into(), value.into()))
    }

    /// `cargo::rustc-link-lib=<spec>`
    pub fn link_lib(&mut self, spec: impl Into<String>) -> &mut Self {
        self.push(Instruction::Link(Link::Lib(spec.into())))
    }

    /// `cargo::rustc-link-search=<path>`
    pub fn link_search(&mut self, path: impl Into<String>) -> &mut Self {
        self.push(Instruction::Link(Link::Search(path.into())))
    }

    /// `cargo::rustc-link-arg=<flag>`
    pub fn link_arg(&mut self, flag: impl Into<String>) -> &mut Self {
        self.push(Instruction::Link(Link::Arg(flag.into())))
    }

    /// `cargo::rerun-if-changed=<path>`
    pub fn rerun_if_changed(&mut self, path: impl Into<String>) -> &mut Self {
        self.push(Instruction::Rerun(Rerun::IfChanged(path.into())))
    }

    /// `cargo::rerun-if-env-changed=<name>`
    pub fn rerun_if_env_changed(&mut self, name: impl Into<String>) -> &mut Self {
        self.push(Instruction::Rerun(Rerun::IfEnvChanged(name.into())))
    }

    /// `cargo::warning=<message>`
    pub fn warning(&mut self, message: impl Into<String>) -> &mut Self {
        self.push(Instruction::Warning(message.into()))
    }

    /// `cargo::error=<message>`
    pub fn error(&mut self, message: impl Into<String>) -> &mut Self {
        self.push(Instruction::Error(message.into()))
    }

    /// True if the detected toolchain is at least `min` (e.g. `"1.80.0"`).
    ///
    /// Returns `false` if `min` cannot be parsed or no version was detected.
    /// Use this to add instructions conditionally without failing the build.
    pub fn at_least(&self, min: &str) -> bool {
        match (Version::parse(min), &self.version) {
            (Some(min), Some(version)) => version.at_least(&min),
            _ => false,
        }
    }

    /// Requires the detected rustc to be at least `min` (e.g. `"1.80.0"`).
    ///
    /// Records a `cargo::error` — failing the build on [`Config::emit`] — when
    /// the toolchain is older than `min`, the version could not be detected, or
    /// `min` is not a valid version string.
    pub fn require_version(&mut self, min: &str) -> &mut Self {
        let Some(min) = Version::parse(min) else {
            return self.error(format!("invalid minimum rustc version: {min}"));
        };

        match &self.version {
            Some(version) if version.at_least(&min) => self,
            Some(version) => self.error(format!("requires rustc >= {min}, found {version}")),
            None => self.error(format!("requires rustc >= {min}, but rustc version could not be detected")),
        }
    }

    /// Prints every accumulated instruction as a `cargo::...` line on stdout.
    pub fn emit(&self) {
        for instruction in &self.instructions {
            println!("{instruction}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nightly(major: u16, minor: u16, patch: u16) -> Version {
        Version {
            major,
            minor,
            patch,
            channel: crate::rustc::version::Channel::Nightly,
        }
    }

    #[test]
    fn records_instructions_in_order() {
        let mut config = Config::with_version(nightly(1, 96, 0));
        config.check_cfg("cfg(nightly)").cfg("nightly").warning("heads up");

        let lines: Vec<String> = config.instructions().iter().map(|i| i.to_string()).collect();
        assert_eq!(
            lines,
            vec![
                "cargo::rustc-check-cfg=cfg(nightly)".to_string(),
                "cargo::rustc-cfg=nightly".to_string(),
                "cargo::warning=heads up".to_string(),
            ]
        );
    }

    #[test]
    fn require_version_satisfied_adds_no_error() {
        let mut config = Config::with_version(Version::parse("1.96.0").unwrap());
        config.require_version("1.80.0");
        assert!(config.instructions().is_empty());
    }

    #[test]
    fn require_version_too_old_adds_error() {
        let mut config = Config::with_version(Version::parse("1.74.0").unwrap());
        config.require_version("1.80.0");

        match config.instructions() {
            [Instruction::Error(msg)] => {
                assert!(msg.contains("requires rustc >= 1.80.0"));
                assert!(msg.contains("found 1.74.0"));
            }
            other => panic!("unexpected instructions: {other:?}"),
        }
    }

    #[test]
    fn require_version_undetected_adds_error() {
        let mut config = Config::default(); // version = None
        config.require_version("1.80.0");

        match config.instructions() {
            [Instruction::Error(msg)] => assert!(msg.contains("could not be detected")),
            other => panic!("unexpected instructions: {other:?}"),
        }
    }

    #[test]
    fn require_version_invalid_bound_adds_error() {
        let mut config = Config::with_version(Version::parse("1.96.0").unwrap());
        config.require_version("not-a-version");

        match config.instructions() {
            [Instruction::Error(msg)] => assert!(msg.contains("invalid minimum rustc version")),
            other => panic!("unexpected instructions: {other:?}"),
        }
    }

    #[test]
    fn at_least_cases() {
        let config = Config::with_version(Version::parse("1.80.0").unwrap());
        assert!(config.at_least("1.80.0"));
        assert!(config.at_least("1.79.5"));
        assert!(!config.at_least("1.81.0"));
        assert!(!config.at_least("garbage"));

        let undetected = Config::default();
        assert!(!undetected.at_least("1.0.0"));
    }

    #[test]
    fn reproduces_diagnostic_build_script() {
        // Mirrors crates/moxy-diagnostic/build.rs: on a feature-capable nightly
        // at least 1.31.0, declare and enable the `nightly` cfg.
        let mut config = Config::with_version(nightly(1, 96, 0));
        config.require_version("1.31.0");

        if config.at_least("1.31.0") {
            config.check_cfg("cfg(nightly)").cfg("nightly");
        }

        let lines: Vec<String> = config.instructions().iter().map(|i| i.to_string()).collect();
        assert_eq!(
            lines,
            vec![
                "cargo::rustc-check-cfg=cfg(nightly)".to_string(),
                "cargo::rustc-cfg=nightly".to_string(),
            ]
        );
    }
}
