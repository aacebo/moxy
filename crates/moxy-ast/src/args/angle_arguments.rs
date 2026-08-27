use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, Token, TokenStream};

use super::GenericArgument;
use crate::Punctuated;

/// A `<...>` argument list.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AngleArguments {
    pub colon2: Option<Token![::]>,
    pub lt_punct: Token![<],
    pub args: Punctuated<GenericArgument, Token![,]>,
    pub gt_punct: Token![>],
}

impl Parse for AngleArguments {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            colon2: stream.parse_if(),
            lt_punct: stream.parse()?,
            args: Punctuated::parse_separated_nonempty(stream)?,
            gt_punct: stream.parse()?,
        })
    }
}

impl Spanner for AngleArguments {
    fn span(&self) -> Span {
        self.colon2
            .map(|c| c.span())
            .unwrap_or(self.lt_punct.span())
            .join(self.gt_punct.span())
    }
}

impl ToTokens for AngleArguments {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.colon2.to_tokens(t);
        self.lt_punct.to_tokens(t);
        self.args.to_tokens(t);
        self.gt_punct.to_tokens(t);
    }
}
