mod attr_args;
mod attr_style;

pub use attr_args::*;
pub use attr_style::*;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Not, Pound};
use moxy_token::{Bracket, Parse, Span, ToTokens, TokenStream};

use crate::Path;

#[doc = "A Rust attribute (`#[...]` or `#![...]`) applied to an item, expression, or statement."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Attribute {
    pub span: Span,
    pub style: AttrStyle,
    pub bracket: Bracket,
    pub path: Path,
    pub args: AttrArgs,
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

        let (bracket, inner) = stream.parse_bracket()?;
        let mut inner = inner.parse();
        let path = inner.parse::<Path>()?;
        let args = inner.parse::<AttrArgs>()?;

        Ok(Self {
            span: Span::default(),
            style,
            bracket,
            path,
            args,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.style.to_tokens(tokens);

        let mut inner = TokenStream::new();
        self.path.to_tokens(&mut inner);
        self.args.to_tokens(&mut inner);
        self.bracket.surround(tokens, inner);
    }
}
