use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;
use moxy_token::token::punct::Quote;

use super::LifetimeName;

#[doc = "A named lifetime (e.g. `'a`, `'static`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Lifetime {
    #[parse(skip)]
    pub span: Span,
    #[parse(prefix = Quote)]
    pub ident: LifetimeName,
}

impl Lifetime {
    pub fn parse_bounds(
        stream: &mut moxy_token::parse::ParseStream,
    ) -> Result<crate::Punctuated<Self, moxy_token::token::punct::Plus>, moxy_token::parse::ParseError> {
        use moxy_token::token::punct::{Colon, Plus};
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
