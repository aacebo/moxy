use super::ImplItem;
use crate::ast::{Attribute, Defaultness, Generics, Ident, Type, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Type as KwType;
use crate::token::punct::{Eq, Semi};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "An associated type definition inside an `impl` block (`type Name = Type;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
}

impl Parse for ImplItemType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<Defaultness>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("type") {
            return Err(LexError::new(at).message("expected impl type").into());
        }

        let _ = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let _ = stream.parse::<Eq>()?;
        let ty = stream.parse::<Type>()?;
        let _ = stream.parse::<Semi>();
        Ok(ImplItemType {
            span: Span::default(),
            attrs,
            vis,
            defaultness,
            ident,
            generics,
            ty,
        })
    }
}

impl ToTokens for ImplItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        KwType::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Eq::default().to_tokens(t);
        self.ty.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
