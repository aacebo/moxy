use moxy_token::keyword::Type as KwType;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Eq, Semi};
use moxy_token::{LexError, Parse, Span, ToTokens, TokenStream};

use super::ImplItem;
use crate::{Attribute, Defaultness, Generics, Ident, Type, Visibility};

#[doc = "An associated type definition inside an `impl` block (`type Name = Type;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub type_keyword: KwType,
    pub ident: Ident,
    pub generics: Generics,
    pub eq: Eq,
    pub ty: Type,
    pub semi: Option<Semi>,
}

impl Parse for ImplItemType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<Defaultness>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("type") {
            return Err(LexError::new(at).message("expected impl type").into());
        }

        let type_keyword = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let eq = stream.parse::<Eq>()?;
        let ty = stream.parse::<Type>()?;
        let semi = stream.parse_if::<Semi>();
        Ok(ImplItemType {
            span: Span::default(),
            attrs,
            vis,
            defaultness,
            type_keyword,
            ident,
            generics,
            eq,
            ty,
            semi,
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
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.eq.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
