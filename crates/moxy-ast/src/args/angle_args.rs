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
    pub args: Punctuated<GenericArgument, Comma>,
}

impl Parse for AngleArgs {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Lt>()?;
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
            args,
        })
    }
}

impl ToTokens for AngleArgs {
    fn to_tokens(&self, t: &mut TokenStream) {
        Lt::default().to_tokens(t);
        self.args.to_tokens(t);
        Gt::default().to_tokens(t);
    }
}
