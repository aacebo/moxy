use super::UseTree;
use crate::ast::Ident;
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::As;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A renamed use leaf (`foo as bar`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseRename {
    pub span: Span,
    pub ident: Ident,
    pub rename: Ident,
}

impl Parse for UseRename {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match UseTree::parse(stream)? {
            UseTree::Rename(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected use rename").into()),
        }
    }
}

impl ToTokens for UseRename {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        As::default().to_tokens(t);
        self.rename.to_tokens(t);
    }
}
