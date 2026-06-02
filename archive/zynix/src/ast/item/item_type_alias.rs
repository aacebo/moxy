use crate::ast::{Attribute, Generics, Ident, Type, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Type as KwType;
use crate::token::punct::{Eq, Semi};
use crate::{Parse, Span, TokenStream};

#[doc = "A type alias item (`type Name<T> = Type;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTypeAlias {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
}

impl Parse for ItemTypeAlias {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let _ = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let _ = stream.parse::<Eq>()?;
        let ty = stream.parse::<Type>()?;
        let _ = stream.parse::<Semi>();
        Ok(ItemTypeAlias {
            span: Span::default(),
            attrs,
            vis,
            ident,
            generics,
            ty,
        })
    }
}

impl ToTokens for ItemTypeAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        KwType::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Eq::default().to_tokens(t);
        self.ty.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
