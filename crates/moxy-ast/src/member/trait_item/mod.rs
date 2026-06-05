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

#[doc = "An item inside a `trait` definition."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TraitItem {
    Fn(TraitItemFn),
    Const(Box<TraitItemConst>),
    Type(TraitItemType),
    Macro(TraitItemMacro),
}

impl Spanner for TraitItem {
    fn span(&self) -> Span {
        match self {
            TraitItem::Fn(v) => v.span(),
            TraitItem::Const(v) => v.span(),
            TraitItem::Type(v) => v.span(),
            TraitItem::Macro(v) => v.span(),
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
        TraitItem::Const(Box::new(v))
    }
}

impl Parse for TraitItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<TraitItemConst>().is_some() {
            return Ok(TraitItem::Const(Box::new(stream.parse()?)));
        }
        if stream.peek::<TraitItemType>().is_some() {
            return Ok(TraitItem::Type(stream.parse()?));
        }
        if stream.peek::<TraitItemFn>().is_some() {
            return Ok(TraitItem::Fn(stream.parse()?));
        }
        Ok(TraitItem::Macro(stream.parse()?))
    }
}

impl ToTokens for TraitItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            TraitItem::Fn(v) => v.to_tokens(t),
            TraitItem::Const(v) => v.to_tokens(t),
            TraitItem::Type(v) => v.to_tokens(t),
            TraitItem::Macro(v) => v.to_tokens(t),
        }
    }
}
