mod foreign_item_fn;
mod foreign_item_macro;
mod foreign_item_static;
mod foreign_item_type;

pub use foreign_item_fn::*;
pub use foreign_item_macro::*;
pub use foreign_item_static::*;
pub use foreign_item_type::*;

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An item inside an `extern` block.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<ForeignItemStatic>()
            || cursor.peek::<ForeignItemType>()
            || cursor.peek::<ForeignItemFn>()
            || cursor.peek::<ForeignItemMacro>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<ForeignItemStatic>() {
            return Ok(Self::Static(parser.parse()?));
        }

        if parser.peek::<ForeignItemType>() {
            return Ok(Self::Type(parser.parse()?));
        }

        if parser.peek::<ForeignItemFn>() {
            return Ok(Self::Fn(parser.parse()?));
        }

        Ok(Self::Macro(parser.parse()?))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<ForeignItemStatic>() {
            cursor.skip::<ForeignItemStatic>()
        } else if cursor.peek::<ForeignItemType>() {
            cursor.skip::<ForeignItemType>()
        } else if cursor.peek::<ForeignItemFn>() {
            cursor.skip::<ForeignItemFn>()
        } else {
            cursor.skip::<ForeignItemMacro>()
        }
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
