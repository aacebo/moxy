use super::ForeignItem;
use crate::ast::{Attribute, Signature, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Semi;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A foreign function declaration inside an `extern` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
}

impl Parse for ForeignItemFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;

        if !crate::ast::sig::Signature::is_start(stream) {
            return Err(LexError::new(at).message("expected foreign fn").into());
        }

        let sig = stream.parse::<Signature>()?;
        let _ = stream.parse::<Semi>();
        Ok(ForeignItemFn {
            span: Span::default(),
            attrs,
            vis,
            sig,
        })
    }
}

impl ToTokens for ForeignItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.sig.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
