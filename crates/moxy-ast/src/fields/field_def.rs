use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Colon;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Ident, Mutability, Type, Visibility};

#[doc = "A struct/enum field definition (`pub name: Type` or `pub Type`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldDef {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Option<Ident>,
    pub colon_punct: Option<Colon>,
    pub ty: Type,
}

impl Parse for FieldDef {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let mutability = stream.parse::<Mutability>()?;

        let (ident, colon_punct) = {
            let mut fork = stream.fork();
            if let Ok(id) = fork.parse::<Ident>() {
                if fork.peek::<Colon>().is_some() {
                    stream.seek(&fork);
                    let colon = stream.parse::<Colon>()?;
                    (Some(id), Some(colon))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            }
        };

        let ty = stream.parse::<Type>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            vis,
            mutability,
            ident,
            colon_punct,
            ty,
        })
    }
}

impl ToTokens for FieldDef {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.mutability.to_tokens(t);

        if let Some(id) = &self.ident {
            id.to_tokens(t);
            if let Some(colon_punct) = &self.colon_punct {
                colon_punct.to_tokens(t);
            }
        }

        self.ty.to_tokens(t);
    }
}
