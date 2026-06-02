use super::TraitItem;
use crate::ast::{Attribute, Signature, StmtBlock};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Semi;
use crate::token::{Delim, LexError, ToTokens, TokenTree};
use crate::{Parse, Span, TokenStream};

#[doc = "A method declaration or default implementation inside a trait definition."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub sig: Signature,
    pub default_body: Option<StmtBlock>,
}

impl Parse for TraitItemFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;

        if !crate::ast::sig::Signature::is_start(stream) {
            return Err(LexError::new(at).message("expected trait fn").into());
        }

        let sig = stream.parse::<Signature>()?;

        let default_body = if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            Some(stream.parse::<StmtBlock>()?)
        } else {
            let _ = stream.parse::<Semi>();
            None
        };

        Ok(TraitItemFn {
            span: Span::default(),
            attrs,
            sig,
            default_body,
        })
    }
}

impl ToTokens for TraitItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.sig.to_tokens(t);
        match &self.default_body {
            Some(b) => b.to_tokens(t),
            None => Semi::default().to_tokens(t),
        }
    }
}
