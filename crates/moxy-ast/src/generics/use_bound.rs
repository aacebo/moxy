use moxy_token::keyword::Use;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Gt, Lt};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Lifetime, Punctuated};

/// A `use<'a, T>` bound (precise capturing).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseBound {
    pub use_keyword: Use,
    pub lt_punct: Lt,
    pub lifetimes: Punctuated<Lifetime, Comma>,
    pub gt_punct: Gt,
}

impl Parse for UseBound {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            use_keyword: stream.parse()?,
            lt_punct: stream.parse()?,
            lifetimes: Punctuated::parse_separated_nonempty(stream)?,
            gt_punct: stream.parse()?,
        })
    }
}

impl Spanner for UseBound {
    fn span(&self) -> Span {
        self.use_keyword.span().join(self.gt_punct.span())
    }
}

impl ToTokens for UseBound {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.use_keyword.to_tokens(t);
        self.lt_punct.to_tokens(t);
        self.lifetimes.to_tokens(t);
        self.gt_punct.to_tokens(t);
    }
}
