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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
}

impl Spanner for ForeignItem {
    fn span(&self) -> Span {
        match self {
            ForeignItem::Fn(v) => v.span(),
            ForeignItem::Static(v) => v.span(),
            ForeignItem::Type(v) => v.span(),
            ForeignItem::Macro(v) => v.span(),
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
        if stream.peek::<ForeignItemStatic>() {
            return Ok(ForeignItem::Static(stream.parse()?));
        }
        if stream.peek::<ForeignItemType>() {
            return Ok(ForeignItem::Type(stream.parse()?));
        }
        if stream.peek::<ForeignItemFn>() {
            return Ok(ForeignItem::Fn(stream.parse()?));
        }
        Ok(ForeignItem::Macro(stream.parse()?))
    }
}

impl ToTokens for ForeignItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            ForeignItem::Fn(v) => v.to_tokens(t),
            ForeignItem::Static(v) => v.to_tokens(t),
            ForeignItem::Type(v) => v.to_tokens(t),
            ForeignItem::Macro(v) => v.to_tokens(t),
        }
    }
}
