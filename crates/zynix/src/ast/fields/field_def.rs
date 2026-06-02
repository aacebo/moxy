use crate::ast::{Attribute, Ident, Mutability, Type, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::Colon;
use crate::{Parse, Span, TokenStream};

#[doc = "A struct/enum field definition (`pub name: Type` or `pub Type`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldDef {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Option<Ident>,
    pub ty: Type,
}

impl Parse for FieldDef {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let mutability = stream.parse::<Mutability>()?;

        let ident = {
            let mut fork = stream.fork();
            if let Ok(id) = fork.parse::<Ident>() {
                if fork.peek::<Colon>().is_some() {
                    stream.seek(&fork);
                    let _ = stream.parse::<Colon>()?;
                    Some(id)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let ty = stream.parse::<Type>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            vis,
            mutability,
            ident,
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
            Colon::default().to_tokens(t);
        }

        self.ty.to_tokens(t);
    }
}
