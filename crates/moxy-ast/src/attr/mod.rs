pub mod meta;
pub mod query;
mod style;

pub use meta::Meta;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Not, Pound};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};
pub use style::*;

use crate::Delimited;

/// A Rust attribute (`#[...]` or `#![...]`) applied to an item, expression, or statement.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Attribute {
    pub style: AttrStyle,
    pub meta: Delimited<Meta>,
}

impl Spanner for Attribute {
    fn span(&self) -> Span {
        self.style.span().join(self.meta.span())
    }
}

impl Parse for Attribute {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let pound = stream.parse::<Pound>()?;
        let style = if stream.peek::<Not>() {
            let not = stream.parse::<Not>()?;
            AttrStyle::Inner(pound, not)
        } else {
            AttrStyle::Outer(pound)
        };

        let meta = Delimited::parse_bracket(stream)?;
        Ok(Self { style, meta })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.style.to_tokens(tokens);
        self.meta.to_tokens(tokens);
    }
}

/// A list of attributes
///
/// # Example
/// ```ignore
/// #[a]
/// #[a(b)]
/// #[a { .. }]
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct Attributes(Vec<Attribute>);

impl Spanner for Attributes {
    fn span(&self) -> Span {
        match (self.0.first(), self.0.last()) {
            (Some(a), Some(b)) => a.span().join(b.span()),
            (Some(a), None) => a.span(),
            (None, Some(b)) => b.span(),
            _ => Span::default(),
        }
    }
}

impl FromIterator<Attribute> for Attributes {
    fn from_iter<I: IntoIterator<Item = Attribute>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<Vec<Attribute>> for Attributes {
    fn from(v: Vec<Attribute>) -> Self {
        Self(v)
    }
}

impl IntoIterator for Attributes {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = &'a Attribute;
    type IntoIter = std::slice::Iter<'a, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Attributes {
    type Item = &'a mut Attribute;
    type IntoIter = std::slice::IterMut<'a, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for Attributes {
    type Target = Vec<Attribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Attributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.0 {
            attr.to_tokens(tokens);
        }
    }
}

impl Parse for Attributes {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self(stream.parse_while::<Attribute>()))
    }
}
