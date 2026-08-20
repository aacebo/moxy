use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

mod impl_item_const;
mod impl_item_fn;
mod impl_item_macro;
mod impl_item_type;

pub use impl_item_const::*;
pub use impl_item_fn::*;
pub use impl_item_macro::*;
pub use impl_item_type::*;

/// An item inside an `impl` block.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ImplItem {
    Fn(ImplItemFn),
    Const(Box<ImplItemConst>),
    Type(ImplItemType),
    Macro(ImplItemMacro),
}

impl ImplItem {
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

    pub fn as_fn(&self) -> Option<&ImplItemFn> {
        if let Self::Fn(v) = self { Some(v) } else { None }
    }

    pub fn as_const(&self) -> Option<&ImplItemConst> {
        if let Self::Const(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_type(&self) -> Option<&ImplItemType> {
        if let Self::Type(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&ImplItemMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }
}

impl Spanner for ImplItem {
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
        $(impl From<$ty> for ImplItem { fn from(v: $ty) -> Self { ImplItem::$variant(v) } })+
    };
}
impl_from! {
    Fn => ImplItemFn,
    Type => ImplItemType,
    Macro => ImplItemMacro,
}

impl From<ImplItemConst> for ImplItem {
    fn from(v: ImplItemConst) -> Self {
        Self::Const(Box::new(v))
    }
}

impl Parse for ImplItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<ImplItemConst>() {
            return Ok(Self::Const(Box::new(stream.parse()?)));
        }
        if stream.peek::<ImplItemType>() {
            return Ok(Self::Type(stream.parse()?));
        }
        if stream.peek::<ImplItemFn>() {
            return Ok(Self::Fn(stream.parse()?));
        }
        Ok(Self::Macro(stream.parse()?))
    }
}

impl ToTokens for ImplItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Fn(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
            Self::Type(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
        }
    }
}
