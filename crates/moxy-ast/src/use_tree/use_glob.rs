use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Star;
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::UseTree;

#[doc = "A glob import (`*`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGlob {
    pub star: Star,
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

impl Spanner for UseGlob {
    fn span(&self) -> Span {
        self.star.span()
    }
}

impl ToTokens for UseGlob {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.star.to_tokens(t);
    }
}
