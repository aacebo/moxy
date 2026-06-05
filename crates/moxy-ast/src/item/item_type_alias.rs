use moxy_token::keyword::Type as KwType;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Eq, Semi};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Generics, Ident, Type, Visibility};

#[doc = "A type alias item (`type Name<T> = Type;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTypeAlias {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub type_keyword: KwType,
    pub ident: Ident,
    pub generics: Generics,
    pub eq_punct: Eq,
    pub ty: Type,
    pub semi_punct: Semi,
}

impl Parse for ItemTypeAlias {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let type_keyword = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let eq_punct = stream.parse::<Eq>()?;
        let ty = stream.parse::<Type>()?;
        let semi_punct = stream.parse::<Semi>()?;
        Ok(ItemTypeAlias {
            span: Span::default(),
            attrs,
            vis,
            type_keyword,
            ident,
            generics,
            eq_punct,
            ty,
            semi_punct,
        })
    }
}

impl ToTokens for ItemTypeAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}
