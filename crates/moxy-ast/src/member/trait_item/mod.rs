use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

mod trait_item_const;
mod trait_item_fn;
mod trait_item_macro;
mod trait_item_type;

pub use trait_item_const::*;
pub use trait_item_fn::*;
pub use trait_item_macro::*;
pub use trait_item_type::*;

/// An item inside a `trait` definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TraitItem {
    Fn(TraitItemFn),
    Const(Box<TraitItemConst>),
    Type(TraitItemType),
    Macro(TraitItemMacro),
}

impl TraitItem {
    pub fn is_fn(&self) -> bool {
        matches!(self, Self::Fn(_))
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn as_fn(&self) -> Option<&TraitItemFn> {
        if let Self::Fn(v) = self { Some(v) } else { None }
    }

    pub fn as_const(&self) -> Option<&TraitItemConst> {
        if let Self::Const(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_type(&self) -> Option<&TraitItemType> {
        if let Self::Type(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&TraitItemMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }
}

impl Spanner for TraitItem {
    fn span(&self) -> Span {
        match self {
            Self::Fn(v) => v.span(),
            Self::Const(v) => v.span(),
            Self::Type(v) => v.span(),
            Self::Macro(v) => v.span(),
        }
    }
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(impl From<$ty> for TraitItem { fn from(v: $ty) -> Self { TraitItem::$variant(v) } })+
    };
}

impl_from! {
    Fn => TraitItemFn,
    Type => TraitItemType,
    Macro => TraitItemMacro,
}

impl From<TraitItemConst> for TraitItem {
    fn from(v: TraitItemConst) -> Self {
        Self::Const(Box::new(v))
    }
}

impl Parse for TraitItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if let Some(item) = stream.parse_if::<TraitItemConst>() {
            return Ok(Self::Const(Box::new(item)));
        }

        if let Some(item) = stream.parse_if::<TraitItemType>() {
            return Ok(Self::Type(item));
        }

        if let Some(item) = stream.parse_if::<TraitItemFn>() {
            return Ok(Self::Fn(item));
        }

        Ok(Self::Macro(stream.parse()?))
    }
}

impl ToTokens for TraitItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Fn(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
            Self::Type(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
        }
    }
}
