mod attr_args;
mod attr_style;

pub use attr_args::*;
pub use attr_style::*;

use crate::ast::Path;
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::{Not, Pound};
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A Rust attribute (`#[...]` or `#![...]`) applied to an item, expression, or statement."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Attribute {
    pub span: Span,
    pub style: AttrStyle,
    pub path: Path,
    pub args: AttrArgs,
}

impl Parse for Attribute {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Pound>()?;

        let style = if stream.peek::<Not>().is_some() {
            let _ = stream.parse::<Not>()?;
            AttrStyle::Inner
        } else {
            AttrStyle::Outer
        };

        let inner = stream.parse_group(Delim::Bracket)?;
        let mut inner = inner.parse();
        let path = inner.parse::<Path>()?;
        let args = inner.parse::<AttrArgs>()?;

        Ok(Self {
            span: Span::default(),
            style,
            path,
            args,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Pound::default().to_tokens(tokens);
        if self.style == AttrStyle::Inner {
            Not::default().to_tokens(tokens);
        }

        let mut inner = TokenStream::new();
        self.path.to_tokens(&mut inner);
        self.args.to_tokens(&mut inner);
        tokens.extend_one(TokenTree::Group(Group::new(Delim::Bracket, inner)));
    }
}
