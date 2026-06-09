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
#[derive(Debug, Clone)]
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

        let meta = Delimited::<Meta>::parse_bracket(stream)?;
        Ok(Self { style, meta })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.style.to_tokens(tokens);
        self.meta.to_tokens(tokens);
    }
}
