use crate::Cursor;

use super::*;

/// A tagged AST representation of Rust meta argument syntax.
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
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<MetaValue>() || cursor.peek::<Meta>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Path>() {
            Ok(Self::Meta(parser.parse()?))
        } else {
            Ok(Self::Value(parser.parse()?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Path>() {
            cursor.skip::<Meta>()
        } else {
            cursor.skip::<MetaValue>()
        }
    }
}

impl Spanner for MetaArgument {
    fn span(&self) -> Span {
        match self {
            Self::Meta(v) => v.span(),
            Self::Value(v) => v.span(),
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
