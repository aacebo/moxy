use super::UseTree;
use crate::ast::Ident;
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::PathSep;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A use path segment (`foo::<rest>`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UsePath {
    pub span: Span,
    pub ident: Ident,
    pub tree: Box<UseTree>,
}

impl Parse for UsePath {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match UseTree::parse(stream)? {
            UseTree::Path(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected use path").into()),
        }
    }
}

impl ToTokens for UsePath {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        PathSep::default().to_tokens(t);
        self.tree.to_tokens(t);
    }
}
