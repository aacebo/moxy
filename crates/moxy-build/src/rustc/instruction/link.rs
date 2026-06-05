/// The `cargo::rustc-link-*` directive family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Link {
    /// `cargo::rustc-link-arg=FLAG`
    Arg(String),
    /// `cargo::rustc-link-arg-bin=BIN=FLAG`
    ArgBin(String, String),
    /// `cargo::rustc-link-arg-bins=FLAG`
    ArgBins(String),
    /// `cargo::rustc-link-arg-tests=FLAG`
    ArgTests(String),
    /// `cargo::rustc-link-arg-examples=FLAG`
    ArgExamples(String),
    /// `cargo::rustc-link-arg-benches=FLAG`
    ArgBenches(String),
    /// `cargo::rustc-link-arg-cdylib=FLAG`
    ArgCdylib(String),
    /// `cargo::rustc-link-lib=[KIND[:MOD]=]NAME[:RENAME]`
    Lib(String),
    /// `cargo::rustc-link-search=[KIND=]PATH`
    Search(String),
}

impl Link {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Arg(_) => "rustc-link-arg",
            Self::ArgBin(..) => "rustc-link-arg-bin",
            Self::ArgBins(_) => "rustc-link-arg-bins",
            Self::ArgTests(_) => "rustc-link-arg-tests",
            Self::ArgExamples(_) => "rustc-link-arg-examples",
            Self::ArgBenches(_) => "rustc-link-arg-benches",
            Self::ArgCdylib(_) => "rustc-link-arg-cdylib",
            Self::Lib(_) => "rustc-link-lib",
            Self::Search(_) => "rustc-link-search",
        }
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgBin(bin, flag) => write!(f, "{}={bin}={flag}", self.as_str()),
            Self::Arg(value)
            | Self::ArgBins(value)
            | Self::ArgTests(value)
            | Self::ArgExamples(value)
            | Self::ArgBenches(value)
            | Self::ArgCdylib(value)
            | Self::Lib(value)
            | Self::Search(value) => write!(f, "{}={value}", self.as_str()),
        }
    }
}
