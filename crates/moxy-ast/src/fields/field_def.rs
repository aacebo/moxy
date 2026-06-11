use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Colon;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Ident, Mutability, Type, Visibility};

/// A struct/enum field definition (`pub name: Type` or `pub Type`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldDef {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Option<Ident>,
    pub colon_punct: Option<Colon>,
    pub ty: Type,
}

impl Parse for FieldDef {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let mutability = stream.parse::<Mutability>()?;

        let (ident, colon_punct) = {
            let mut fork = stream.fork();
            if let Ok(id) = fork.parse::<Ident>() {
                if fork.peek::<Colon>() {
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
            attrs,
            vis,
            mutability,
            ident,
            colon_punct,
            ty,
        })
    }
}

impl Spanner for FieldDef {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if !matches!(self.vis, Visibility::Inherited) {
            self.vis.span()
        } else if !matches!(self.mutability, Mutability::Immutable) {
            self.mutability.span()
        } else if let Some(id) = &self.ident {
            id.span()
        } else {
            self.ty.span()
        };
        start.join(self.ty.span())
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
