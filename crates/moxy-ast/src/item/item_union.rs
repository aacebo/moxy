use moxy_token::keyword::Union;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, FieldsNamed, Generics, Ident, Visibility};

#[doc = "A union item (`union Name<T> { field: Type, ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUnion {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: FieldsNamed,
}

impl Parse for ItemUnion {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let _ = stream.parse::<Union>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let fields = stream.parse::<FieldsNamed>()?;
        Ok(ItemUnion {
            span: Span::default(),
            attrs,
            vis,
            ident,
            generics,
            fields,
        })
    }
}

impl ToTokens for ItemUnion {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        Union::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.fields.to_tokens(t);
    }
}
