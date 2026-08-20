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
