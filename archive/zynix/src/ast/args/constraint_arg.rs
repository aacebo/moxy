use super::AngleArgs;
use crate::ast::{Ident, Punctuated, TypeBound};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::{Colon, Lt, Plus};
use crate::{Parse, Span, TokenStream};

#[doc = "An associated type bound constraint (`Item: Bound`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstraintArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for ConstraintArg {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let mut fork = stream.fork();
        let ident = fork.parse::<Ident>()?;

        let generics = if fork.peek::<Lt>().is_some() {
            Some(fork.parse::<AngleArgs>()?)
        } else {
            None
        };

        let _ = fork.parse::<Colon>()?;
        let mut bounds = Punctuated::new();

        loop {
            bounds.push_value(fork.parse::<TypeBound>()?);
            if fork.peek::<Plus>().is_some() {
                bounds.push_punct(fork.parse::<Plus>()?);
            } else {
                break;
            }
        }

        stream.seek(&fork);
        Ok(Self {
            span: Span::default(),
            ident,
            generics,
            bounds,
        })
    }
}

impl ToTokens for ConstraintArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        Colon::default().to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
