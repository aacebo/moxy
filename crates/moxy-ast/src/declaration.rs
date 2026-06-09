use moxy_token::parser::ParseError;
use moxy_token::{Parse, Span, Spanner, ToTokens};

use crate::{Attribute, Generics, Ident, Visibility, item};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "snake_case"))]
pub enum Declaration {
    Enum(item::ItemEnum),
    Struct(item::ItemStruct),
    Union(item::ItemUnion),
}

impl Declaration {
    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
    }

    pub fn as_enum(&self) -> Option<&item::ItemEnum> {
        match self {
            Self::Enum(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_struct(&self) -> Option<&item::ItemStruct> {
        match self {
            Self::Struct(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_union(&self) -> Option<&item::ItemUnion> {
        match self {
            Self::Union(v) => Some(v),
            _ => None,
        }
    }

    pub fn attrs(&self) -> &[Attribute] {
        match self {
            Self::Enum(v) => &v.attrs,
            Self::Struct(v) => &v.attrs,
            Self::Union(v) => &v.attrs,
        }
    }

    pub fn vis(&self) -> &Visibility {
        match self {
            Self::Enum(v) => &v.vis,
            Self::Struct(v) => &v.vis,
            Self::Union(v) => &v.vis,
        }
    }

    pub fn ident(&self) -> &Ident {
        match self {
            Self::Enum(v) => &v.ident,
            Self::Struct(v) => &v.ident,
            Self::Union(v) => &v.ident,
        }
    }

    pub fn generics(&self) -> &Generics {
        match self {
            Self::Enum(v) => &v.generics,
            Self::Struct(v) => &v.generics,
            Self::Union(v) => &v.generics,
        }
    }
}

impl From<item::ItemEnum> for Declaration {
    fn from(value: item::ItemEnum) -> Self {
        Self::Enum(value)
    }
}

impl From<item::ItemStruct> for Declaration {
    fn from(value: item::ItemStruct) -> Self {
        Self::Struct(value)
    }
}

impl From<item::ItemUnion> for Declaration {
    fn from(value: item::ItemUnion) -> Self {
        Self::Union(value)
    }
}

impl Spanner for Declaration {
    fn span(&self) -> Span {
        match self {
            Self::Enum(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Union(v) => v.span(),
        }
    }
}

impl ToTokens for Declaration {
    fn to_tokens(&self, tokens: &mut moxy_token::TokenStream) {
        match self {
            Self::Enum(v) => v.to_tokens(tokens),
            Self::Struct(v) => v.to_tokens(tokens),
            Self::Union(v) => v.to_tokens(tokens),
        }
    }
}

impl Parse for Declaration {
    fn parse(stream: &mut moxy_token::parser::ParseStream) -> Result<Self, moxy_token::parser::ParseError> {
        if let Some(v) = stream.parse_if::<item::ItemEnum>() {
            Ok(v.into())
        } else if let Some(v) = stream.parse_if::<item::ItemStruct>() {
            Ok(v.into())
        } else if let Some(v) = stream.parse_if::<item::ItemUnion>() {
            Ok(v.into())
        } else {
            Err(ParseError::new(stream.span(), "expected a user defined type declaration"))
        }
    }
}
