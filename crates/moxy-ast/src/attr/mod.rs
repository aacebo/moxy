mod attr_args;
mod attr_style;
pub mod meta;

pub use attr_args::*;
pub use attr_style::*;
pub use meta::Meta;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Not, Pound};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Delimited, Path};

#[doc = "A Rust attribute (`#[...]` or `#![...]`) applied to an item, expression, or statement."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Attribute {
    pub span: Span,
    pub style: AttrStyle,
    pub content: Delimited<AttrContent>,
}

impl Parse for Attribute {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let pound = stream.parse::<Pound>()?;

        let style = if stream.peek::<Not>().is_some() {
            let not = stream.parse::<Not>()?;
            AttrStyle::Inner(pound, not)
        } else {
            AttrStyle::Outer(pound)
        };

        let content = Delimited::<AttrContent>::parse_bracket(stream)?;

        Ok(Self {
            span: Span::default(),
            style,
            content,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.style.to_tokens(tokens);
        self.content.to_tokens(tokens);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AttrContent {
    pub path: Path,
    pub args: AttrArgs,
}

impl Parse for AttrContent {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let path = stream.parse::<Path>()?;
        let args = stream.parse::<AttrArgs>()?;
        Ok(Self { path, args })
    }
}

impl ToTokens for AttrContent {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.args.to_tokens(t);
    }
}
