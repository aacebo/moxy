use super::ForeignItem;
use crate::ast::{Attribute, Generics, Ident, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Type as KwType;
use crate::token::punct::Semi;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A foreign opaque type declaration inside an `extern` block (`type Name;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
}

impl Parse for ForeignItemType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("type") {
            return Err(LexError::new(at).message("expected foreign type").into());
        }

        let _ = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let _ = stream.parse::<Semi>();
        Ok(ForeignItemType {
            span: Span::default(),
            attrs,
            vis,
            ident,
            generics,
        })
    }
}

impl ToTokens for ForeignItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        KwType::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
