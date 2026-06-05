use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Quote;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::LifetimeName;

#[doc = "A named lifetime (e.g. `'a`, `'static`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Lifetime {
    pub span: Span,
    pub ident: LifetimeName,
}

impl Parse for Lifetime {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let start = stream.span();
        let _ = stream.parse::<Quote>()?;
        let ident = stream.parse::<LifetimeName>()?;
        Ok(Self {
            span: start.join(ident.span),
            ident,
        })
    }
}

impl ToTokens for Lifetime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Quote::default().to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

impl Lifetime {
    pub fn parse_bounds(
        stream: &mut moxy_token::parse::ParseStream,
    ) -> Result<crate::Punctuated<Self, moxy_token::punct::Plus>, moxy_token::parse::ParseError> {
        use moxy_token::punct::{Colon, Plus};
        let mut bounds = crate::Punctuated::new();
        if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;

            loop {
                bounds.push_value(stream.parse::<Lifetime>()?);

                if stream.peek::<Plus>().is_some() {
                    bounds.push_punct(stream.parse::<Plus>()?);
                } else {
                    break;
                }
            }
        }
        Ok(bounds)
    }
}
