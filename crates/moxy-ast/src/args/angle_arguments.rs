use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Gt, Lt};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::GenericArgument;
use crate::Punctuated;

/// A `<...>` argument list.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AngleArguments {
    pub lt_punct: Lt,
    pub args: Punctuated<GenericArgument, Comma>,
    pub gt_punct: Gt,
}

impl Parse for AngleArguments {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            lt_punct: stream.parse()?,
            args: Punctuated::parse_separated_nonempty(stream)?,
            gt_punct: stream.parse()?,
        })
    }
}

impl Spanner for AngleArguments {
    fn span(&self) -> Span {
        self.lt_punct.span().join(self.gt_punct.span())
    }
}

impl ToTokens for AngleArguments {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lt_punct.to_tokens(t);
        self.args.to_tokens(t);
        self.gt_punct.to_tokens(t);
    }
}
