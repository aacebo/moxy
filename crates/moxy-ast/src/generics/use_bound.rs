use moxy_token::keyword::Use;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Gt, Lt};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Lifetime, Punctuated};

#[doc = "A `use<'a, T>` bound (precise capturing)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseBound {
    pub span: Span,
    pub use_keyword: Use,
    pub lt_punct: Lt,
    pub lifetimes: Punctuated<Lifetime, Comma>,
    pub gt_punct: Gt,
}

impl Parse for UseBound {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let use_keyword = stream.parse::<Use>()?;
        let lt_punct = stream.parse::<Lt>()?;
        let mut lifetimes = Punctuated::new();

        while !stream.peek_angle_close() && !stream.is_empty() {
            lifetimes.push_value(stream.parse::<Lifetime>()?);
            if stream.peek::<Comma>().is_some() {
                lifetimes.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        stream.eat_angle_close()?;
        Ok(Self {
            span: Span::default(),
            use_keyword,
            lt_punct,
            lifetimes,
            gt_punct: Gt::default(),
        })
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
