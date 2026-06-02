#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "lowercase"))]
pub enum Spacing {
    #[default]
    Alone,
    Joint,
}

impl Spacing {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alone => "alone",
            Self::Joint => "joint",
        }
    }
}

impl std::fmt::Display for Spacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<proc_macro::Spacing> for Spacing {
    fn from(value: proc_macro::Spacing) -> Self {
        match value {
            proc_macro::Spacing::Alone => Self::Alone,
            proc_macro::Spacing::Joint => Self::Joint,
        }
    }
}

impl From<Spacing> for proc_macro::Spacing {
    fn from(value: Spacing) -> Self {
        match value {
            Spacing::Alone => proc_macro::Spacing::Alone,
            Spacing::Joint => proc_macro::Spacing::Joint,
        }
    }
}

#[cfg(test)]
mod tests {
    mod display {
        use crate::token::Spacing;

        #[test]
        fn writes_as_str() {
            assert_eq!(format!("{}", Spacing::Alone), "alone");
            assert_eq!(format!("{}", Spacing::Joint), "joint");
        }
    }
}
