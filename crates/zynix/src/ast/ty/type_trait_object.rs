use crate::ast::{Punctuated, TypeBound};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Dyn;
use crate::token::punct::Plus;
use crate::{Parse, Span, TokenStream};

#[doc = "A trait object type (e.g. `dyn Iterator<Item = u8>`, `dyn Fn() + 'a`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTraitObject {
    pub span: Span,
    pub dyn_token: bool,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for TypeTraitObject {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let dyn_token = if stream.peek::<Dyn>().is_some() {
            let _ = stream.parse::<Dyn>()?;
            true
        } else {
            false
        };

        let bounds = crate::ast::TypeBound::parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            dyn_token,
            bounds,
        })
    }
}

impl ToTokens for TypeTraitObject {
    fn to_tokens(&self, t: &mut TokenStream) {
        if self.dyn_token {
            Dyn::default().to_tokens(t);
        }

        self.bounds.to_tokens(t);
    }
}
