use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

mod foreign_item_fn;
mod foreign_item_macro;
mod foreign_item_static;
mod foreign_item_type;

pub use foreign_item_fn::*;
pub use foreign_item_macro::*;
pub use foreign_item_static::*;
pub use foreign_item_type::*;

/// An item inside an `extern` block.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
}

impl ForeignItem {
    pub fn is_fn(&self) -> bool {
        matches!(self, Self::Fn(_))
    }

    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn as_fn(&self) -> Option<&ForeignItemFn> {
        if let Self::Fn(v) = self { Some(v) } else { None }
    }

    pub fn as_static(&self) -> Option<&ForeignItemStatic> {
        if let Self::Static(v) = self { Some(v) } else { None }
    }

    pub fn as_type(&self) -> Option<&ForeignItemType> {
        if let Self::Type(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&ForeignItemMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }
}

impl Spanner for ForeignItem {
    fn span(&self) -> Span {
        match self {
            Self::Fn(v) => v.span(),
            Self::Static(v) => v.span(),
            Self::Type(v) => v.span(),
            Self::Macro(v) => v.span(),
        }
    }
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(impl From<$ty> for ForeignItem { fn from(v: $ty) -> Self { ForeignItem::$variant(v) } })+
    };
}

impl_from! {
    Fn => ForeignItemFn,
    Static => ForeignItemStatic,
    Type => ForeignItemType,
    Macro => ForeignItemMacro,
}

impl Parse for ForeignItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if let Some(item) = stream.parse_if::<ForeignItemStatic>() {
            return Ok(Self::Static(item));
        }

        if let Some(item) = stream.parse_if::<ForeignItemType>() {
            return Ok(Self::Type(item));
        }

        if let Some(item) = stream.parse_if::<ForeignItemFn>() {
            return Ok(Self::Fn(item));
        }

        Ok(Self::Macro(stream.parse()?))
    }
}

impl ToTokens for ForeignItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Fn(v) => v.to_tokens(t),
            Self::Static(v) => v.to_tokens(t),
            Self::Type(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
        }
    }
}
