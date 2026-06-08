#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "lowercase"))]
pub enum Spacing {
    #[default]
    Alone,
    Joint,
}

impl Spacing {
    #[inline]
    pub fn is_alone(&self) -> bool {
        matches!(self, Self::Alone)
    }

    #[inline]
    pub fn is_joint(&self) -> bool {
        matches!(self, Self::Joint)
    }

    #[inline]
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

#[cfg(test)]
mod tests {
    mod display {
        use crate::Spacing;

        #[test]
        fn writes_as_str() {
            assert_eq!(format!("{}", Spacing::Alone), "alone");
            assert_eq!(format!("{}", Spacing::Joint), "joint");
        }
    }
}
