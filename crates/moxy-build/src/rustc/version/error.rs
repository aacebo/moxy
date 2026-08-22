#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ParseVersionError;

impl std::fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not parse rustc version")
    }
}

impl std::error::Error for ParseVersionError {}
