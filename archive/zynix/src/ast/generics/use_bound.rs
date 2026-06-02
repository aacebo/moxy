use crate::ast::{Lifetime, Punctuated};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Use;
use crate::token::punct::{Comma, Gt, Lt};
use crate::{Parse, Span, TokenStream};

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
