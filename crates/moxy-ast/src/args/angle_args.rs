use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Gt, Lt};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::GenericArgument;
use crate::Punctuated;

#[doc = "A `<...>` argument list."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AngleArgs {
    pub span: Span,
    pub lt_punct: Lt,
    pub args: Punctuated<GenericArgument, Comma>,
    pub gt_punct: Gt,
}

impl Parse for AngleArgs {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lt_punct = stream.parse::<Lt>()?;
        let mut args = Punctuated::new();

        while !stream.peek_angle_close() && !stream.is_empty() {
            args.push_value(stream.parse::<GenericArgument>()?);
            if stream.peek::<Comma>().is_some() {
                args.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        stream.eat_angle_close()?;
        Ok(Self {
            span: Span::default(),
            lt_punct,
            args,
            gt_punct: Gt::default(),
        })
    }
}

impl ToTokens for AngleArgs {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lt_punct.to_tokens(t);
        self.args.to_tokens(t);
        self.gt_punct.to_tokens(t);
    }
}
