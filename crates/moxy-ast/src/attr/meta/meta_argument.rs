use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum MetaArgument {
    Meta(Meta),
    Value(MetaValue),
}

impl MetaArgument {
    pub fn is_meta(&self) -> bool {
        matches!(self, Self::Meta(_))
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_meta(&self) -> Option<&Meta> {
        match self {
            Self::Meta(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_value(&self) -> Option<&MetaValue> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
        }
    }

    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Meta(v) => Some(&v.path),
            Self::Value(_) => None,
        }
    }
}

impl Parse for MetaArgument {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Path>() {
            Ok(Self::Meta(parser.parse()?))
        } else {
            Ok(Self::Value(parser.parse()?))
        }
    }
}

impl ToTokens for MetaArgument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Meta(v) => v.to_tokens(tokens),
            Self::Value(v) => v.to_tokens(tokens),
        }
    }
}
