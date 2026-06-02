/// The `cargo::rustc-cfg` / `cargo::rustc-check-cfg` directives.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cfg {
    /// `cargo::rustc-cfg=KEY[="VALUE"]`
    Set(String),
    /// `cargo::rustc-check-cfg=CHECK_CFG`
    Check(String),
}

impl Cfg {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Set(_) => "rustc-cfg",
            Self::Check(_) => "rustc-check-cfg",
        }
    }
}

impl std::fmt::Display for Cfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Set(value) | Self::Check(value) => write!(f, "{}={value}", self.as_str()),
        }
    }
}
