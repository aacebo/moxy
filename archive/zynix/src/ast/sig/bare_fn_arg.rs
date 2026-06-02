use crate::ast::{Attribute, Ident, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::{Parse, Span, TokenStream};

#[doc = "An argument of a bare function pointer type."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BareFnArg {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
    pub ty: Type,
}

impl Parse for BareFnArg {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;

        let name = {
            let mut fork = stream.fork();
            if let Ok(id) = fork.parse::<Ident>() {
                if fork.peek::<crate::token::punct::Colon>().is_some() {
                    stream.seek(&fork);
                    let _ = stream.parse::<crate::token::punct::Colon>()?;
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
            name,
            ty,
        })
    }
}

impl ToTokens for BareFnArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some(n) = &self.name {
            n.to_tokens(t);
            crate::token::punct::Colon::default().to_tokens(t);
        }

        self.ty.to_tokens(t);
    }
}
