/// The `cargo::rerun-*` directive family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rerun {
    /// `cargo::rerun-if-changed=PATH`
    IfChanged(String),
    /// `cargo::rerun-if-env-changed=NAME`
    IfEnvChanged(String),
}

impl Rerun {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IfChanged(_) => "rerun-if-changed",
            Self::IfEnvChanged(_) => "rerun-if-env-changed",
        }
    }
}

impl std::fmt::Display for Rerun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IfChanged(value) | Self::IfEnvChanged(value) => {
                write!(f, "{}={value}", self.as_str())
            }
        }
    }
}
