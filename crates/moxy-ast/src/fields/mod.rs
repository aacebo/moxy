use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

mod field_def;
mod field_value;
mod fields_named;
mod fields_unnamed;

pub use field_def::*;
pub use field_value::*;
pub use fields_named::*;
pub use fields_unnamed::*;

/// The fields of a struct/enum variant (named, unnamed, or unit).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}

impl From<FieldsNamed> for Fields {
    fn from(v: FieldsNamed) -> Self {
        Fields::Named(v)
    }
}

impl From<FieldsUnnamed> for Fields {
    fn from(v: FieldsUnnamed) -> Self {
        Fields::Unnamed(v)
    }
}

impl Spanner for Fields {
    fn span(&self) -> Span {
        match self {
            Fields::Named(v) => v.span(),
            Fields::Unnamed(v) => v.span(),
            Fields::Unit => Span::call_site(),
        }
    }
}

impl Parse for Fields {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            Some(TokenTree::Group(g)) if g.delim() == Delim::Brace => Ok(Fields::Named(stream.parse()?)),
            Some(TokenTree::Group(g)) if g.delim() == Delim::Paren => Ok(Fields::Unnamed(stream.parse()?)),
            _ => Ok(Fields::Unit),
        }
    }
}

impl ToTokens for Fields {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Fields::Named(v) => v.to_tokens(t),
            Fields::Unnamed(v) => v.to_tokens(t),
            Fields::Unit => {}
        }
    }
}
