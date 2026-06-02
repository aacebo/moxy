use zynix_macros::{Parse, ToTokens};

use super::LifetimeName;
use crate::Span;
use crate::token::punct::Quote;

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
        stream: &mut crate::parse::ParseStream,
    ) -> Result<crate::ast::Punctuated<Self, crate::token::punct::Plus>, crate::parse::ParseError> {
        use crate::token::punct::{Colon, Plus};
        let mut bounds = crate::ast::Punctuated::new();
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
