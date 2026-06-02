use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::{Parse, TokenStream};

mod impl_item_const;
mod impl_item_fn;
mod impl_item_macro;
mod impl_item_type;

pub use impl_item_const::*;
pub use impl_item_fn::*;
pub use impl_item_macro::*;
pub use impl_item_type::*;

#[doc = "An item inside an `impl` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ImplItem {
    Fn(ImplItemFn),
    Const(ImplItemConst),
    Type(ImplItemType),
    Macro(ImplItemMacro),
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(impl From<$ty> for ImplItem { fn from(v: $ty) -> Self { ImplItem::$variant(v) } })+
    };
}
impl_from! {
    Fn => ImplItemFn,
    Const => ImplItemConst,
    Type => ImplItemType,
    Macro => ImplItemMacro,
}

impl Parse for ImplItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<ImplItemConst>().is_some() {
            return Ok(ImplItem::Const(stream.parse()?));
        }
        if stream.peek::<ImplItemType>().is_some() {
            return Ok(ImplItem::Type(stream.parse()?));
        }
        if stream.peek::<ImplItemFn>().is_some() {
            return Ok(ImplItem::Fn(stream.parse()?));
        }
        Ok(ImplItem::Macro(stream.parse()?))
    }
}

impl ToTokens for ImplItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            ImplItem::Fn(v) => v.to_tokens(t),
            ImplItem::Const(v) => v.to_tokens(t),
            ImplItem::Type(v) => v.to_tokens(t),
            ImplItem::Macro(v) => v.to_tokens(t),
        }
    }
}
