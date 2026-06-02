use super::UseTree;
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Star;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A glob import (`*`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGlob {
    pub span: Span,
}

impl Parse for UseGlob {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match UseTree::parse(stream)? {
            UseTree::Glob(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected `*`").into()),
        }
    }
}

impl ToTokens for UseGlob {
    fn to_tokens(&self, t: &mut TokenStream) {
        Star::default().to_tokens(t);
    }
}
