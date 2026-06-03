#[derive(Debug)]
pub enum FmtError {
    Std(std::fmt::Error),
}

impl From<std::fmt::Error> for FmtError {
    fn from(value: std::fmt::Error) -> Self {
        Self::Std(value)
    }
}

impl std::fmt::Display for FmtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Std(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for FmtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Std(err) => Some(err),
        }
    }
}
