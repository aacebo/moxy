mod meta_argument;
mod meta_layout;
mod meta_value;

pub use meta_argument::*;
pub use meta_layout::*;
pub use meta_value::*;
use moxy_token::parser::{Parse, ParseError, ParseStream};
use moxy_token::{Comma, Delim, Eq, EqEq, FatArrow, Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Lit, Path, Punctuated};

/// A structured attribute meta item (`name`, `name(...)`, `name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Meta {
    pub path: Path,
    pub content: MetaLayout,
}

impl std::ops::Deref for Meta {
    type Target = MetaLayout;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl std::ops::DerefMut for Meta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl Parse for Meta {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            path: stream.parse()?,
            content: stream.parse()?,
        })
    }
}

impl Spanner for Meta {
    fn span(&self) -> Span {
        match self.content.span() {
            None => self.path.span(),
            Some(span) => self.path.span().join(span),
        }
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.content.to_tokens(t);
    }
}
