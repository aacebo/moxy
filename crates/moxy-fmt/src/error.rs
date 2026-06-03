#[derive(Debug)]
pub enum FormatError {
    IO(std::io::Error),
}

impl From<std::io::Error> for FormatError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for FormatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IO(err) => Some(err),
        }
    }
}
