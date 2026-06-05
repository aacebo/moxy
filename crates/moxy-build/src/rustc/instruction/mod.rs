mod cfg;
mod link;
mod rerun;

#[doc(inline)]
pub use cfg::*;
#[doc(inline)]
pub use link::*;
#[doc(inline)]
pub use rerun::*;

/// A cargo build-script output instruction.
///
/// These are the `cargo::...` lines a `build.rs` prints to stdout for cargo to
/// interpret. Related directives are grouped into the [`Link`], [`Cfg`] and
/// [`Rerun`] sum types; the rest are flat variants.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    /// `cargo::rustc-link-*`
    Link(Link),
    /// `cargo::rustc-cfg` / `cargo::rustc-check-cfg`
    Cfg(Cfg),
    /// `cargo::rerun-*`
    Rerun(Rerun),
    /// `cargo::rustc-flags=FLAGS`
    Flags(String),
    /// `cargo::rustc-env=VAR=VALUE`
    Env(String, String),
    /// `cargo::warning=MESSAGE`
    Warning(String),
    /// `cargo::error=MESSAGE`
    Error(String),
    /// `cargo::metadata=KEY=VALUE`
    Metadata(String, String),
}

impl Instruction {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Link(link) => link.as_str(),
            Self::Cfg(cfg) => cfg.as_str(),
            Self::Rerun(rerun) => rerun.as_str(),
            Self::Flags(_) => "rustc-flags",
            Self::Env(..) => "rustc-env",
            Self::Warning(_) => "warning",
            Self::Error(_) => "error",
            Self::Metadata(..) => "metadata",
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cargo::")?;

        match self {
            Self::Link(link) => write!(f, "{link}"),
            Self::Cfg(cfg) => write!(f, "{cfg}"),
            Self::Rerun(rerun) => write!(f, "{rerun}"),
            Self::Flags(flags) => write!(f, "rustc-flags={flags}"),
            Self::Env(key, value) => write!(f, "rustc-env={key}={value}"),
            Self::Warning(message) => write!(f, "warning={message}"),
            Self::Error(message) => write!(f, "error={message}"),
            Self::Metadata(key, value) => write!(f, "metadata={key}={value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_cfg() {
        assert_eq!(
            Instruction::Cfg(Cfg::Set("nightly".into())).to_string(),
            "cargo::rustc-cfg=nightly"
        );
        assert_eq!(
            Instruction::Cfg(Cfg::Check("cfg(nightly)".into())).to_string(),
            "cargo::rustc-check-cfg=cfg(nightly)"
        );
    }

    #[test]
    fn display_link() {
        assert_eq!(
            Instruction::Link(Link::Lib("static=foo".into())).to_string(),
            "cargo::rustc-link-lib=static=foo"
        );
        assert_eq!(
            Instruction::Link(Link::ArgBin("mybin".into(), "-Lfoo".into())).to_string(),
            "cargo::rustc-link-arg-bin=mybin=-Lfoo"
        );
        assert_eq!(
            Instruction::Link(Link::Search("native=/usr/lib".into())).to_string(),
            "cargo::rustc-link-search=native=/usr/lib"
        );
    }

    #[test]
    fn display_rerun() {
        assert_eq!(
            Instruction::Rerun(Rerun::IfChanged("build.rs".into())).to_string(),
            "cargo::rerun-if-changed=build.rs"
        );
        assert_eq!(
            Instruction::Rerun(Rerun::IfEnvChanged("RUSTC".into())).to_string(),
            "cargo::rerun-if-env-changed=RUSTC"
        );
    }

    #[test]
    fn display_flat() {
        assert_eq!(Instruction::Flags("-l foo".into()).to_string(), "cargo::rustc-flags=-l foo");
        assert_eq!(
            Instruction::Env("KEY".into(), "VAL".into()).to_string(),
            "cargo::rustc-env=KEY=VAL"
        );
        assert_eq!(Instruction::Warning("oops".into()).to_string(), "cargo::warning=oops");
        assert_eq!(Instruction::Error("boom".into()).to_string(), "cargo::error=boom");
        assert_eq!(
            Instruction::Metadata("KEY".into(), "VAL".into()).to_string(),
            "cargo::metadata=KEY=VAL"
        );
    }

    #[test]
    fn as_str_keys() {
        assert_eq!(Link::ArgCdylib("x".into()).as_str(), "rustc-link-arg-cdylib");
        assert_eq!(Cfg::Check("x".into()).as_str(), "rustc-check-cfg");
        assert_eq!(Rerun::IfEnvChanged("x".into()).as_str(), "rerun-if-env-changed");
        assert_eq!(Instruction::Warning("x".into()).as_str(), "warning");
        assert_eq!(Instruction::Link(Link::Lib("x".into())).as_str(), "rustc-link-lib");
    }
}
