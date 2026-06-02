use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::token::ToTokens;
use moxy_token::token::keyword::Use;
use moxy_token::token::punct::{Comma, Gt, Lt};
use moxy_token::{Parse, Span, TokenStream};

use crate::{Lifetime, Punctuated};

#[doc = "A `use<'a, T>` bound (precise capturing)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseBound {
    pub span: Span,
    pub lifetimes: Punctuated<Lifetime, Comma>,
}

impl Parse for UseBound {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Use>()?;
        let _ = stream.parse::<Lt>()?;
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
            lifetimes,
        })
    }
}

impl ToTokens for UseBound {
    fn to_tokens(&self, t: &mut TokenStream) {
        Use::default().to_tokens(t);
        Lt::default().to_tokens(t);
        self.lifetimes.to_tokens(t);
        Gt::default().to_tokens(t);
    }
}
