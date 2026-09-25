mod trait_item_const;
mod trait_item_fn;
mod trait_item_macro;
mod trait_item_type;

pub use trait_item_const::*;
pub use trait_item_fn::*;
pub use trait_item_macro::*;
pub use trait_item_type::*;

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An item inside a `trait` definition.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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

impl From<TraitItemFn> for TraitItem {
    fn from(value: TraitItemFn) -> Self {
        Self::Fn(value)
    }
}

impl From<TraitItemType> for TraitItem {
    fn from(value: TraitItemType) -> Self {
        Self::Type(value)
    }
}

impl From<TraitItemMacro> for TraitItem {
    fn from(value: TraitItemMacro) -> Self {
        Self::Macro(value)
    }
}

impl From<TraitItemConst> for TraitItem {
    fn from(v: TraitItemConst) -> Self {
        Self::Const(Box::new(v))
    }
}

impl Parse for TraitItem {
    fn peek(cursor: Cursor<'_>) -> bool {
        TraitItemConst::peek(cursor) || TraitItemType::peek(cursor) || TraitItemFn::peek(cursor) || TraitItemMacro::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if TraitItemConst::peek(parser.cursor()) {
            return Ok(Self::Const(Box::new(parser.parse()?)));
        }

        if TraitItemType::peek(parser.cursor()) {
            return Ok(Self::Type(parser.parse()?));
        }

        if TraitItemFn::peek(parser.cursor()) {
            return Ok(Self::Fn(parser.parse()?));
        }

        Ok(Self::Macro(parser.parse()?))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if TraitItemConst::peek(cursor) {
            TraitItemConst::skip(cursor)
        } else if TraitItemType::peek(cursor) {
            TraitItemType::skip(cursor)
        } else if TraitItemFn::peek(cursor) {
            TraitItemFn::skip(cursor)
        } else {
            TraitItemMacro::skip(cursor)
        }
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
