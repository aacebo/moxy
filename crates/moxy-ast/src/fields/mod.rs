use crate::{Parse, ParseError, Parser};
use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream, TokenTree};

mod field;
mod field_value;
mod fields_named;
mod fields_unnamed;

pub use field::*;
pub use field_value::*;
pub use fields_named::*;
pub use fields_unnamed::*;

/// The fields of a struct/enum variant (named, unnamed, or unit).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}

impl Fields {
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    pub fn is_unnamed(&self) -> bool {
        matches!(self, Self::Unnamed(_))
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    pub fn as_named(&self) -> Option<&FieldsNamed> {
        if let Self::Named(v) = self { Some(v) } else { None }
    }

    pub fn as_unnamed(&self) -> Option<&FieldsUnnamed> {
        if let Self::Unnamed(v) = self { Some(v) } else { None }
    }
}

impl From<FieldsNamed> for Fields {
    fn from(v: FieldsNamed) -> Self {
        Self::Named(v)
    }
}

impl From<FieldsUnnamed> for Fields {
    fn from(v: FieldsUnnamed) -> Self {
        Self::Unnamed(v)
    }
}

impl Spanner for Fields {
    fn span(&self) -> Span {
        match self {
            Self::Named(v) => v.span(),
            Self::Unnamed(v) => v.span(),
            Self::Unit => Span::call_site(),
        }
    }
}

impl Parse for Fields {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Group(g)) if g.delim() == Delim::Brace => Ok(Self::Named(parser.parse()?)),
            Some(TokenTree::Group(g)) if g.delim() == Delim::Paren => Ok(Self::Unnamed(parser.parse()?)),
            _ => Ok(Self::Unit),
        }
    }
}

impl ToTokens for Fields {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Named(v) => v.to_tokens(t),
            Self::Unnamed(v) => v.to_tokens(t),
            Self::Unit => {}
        }
    }
}
