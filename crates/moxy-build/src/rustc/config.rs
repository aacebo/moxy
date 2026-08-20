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
    /// [`version::read`]).
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
