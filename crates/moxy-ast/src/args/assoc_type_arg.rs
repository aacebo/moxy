use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::token::ToTokens;
use moxy_token::token::punct::{Eq, Lt};
use moxy_token::{Parse, Span, TokenStream};

use super::AngleArgs;
use crate::{Ident, Type};

#[doc = "An associated type binding (`Item = T`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocTypeArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub ty: Type,
}

impl Parse for AssocTypeArg {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let mut fork = stream.fork();
        let ident = fork.parse::<Ident>()?;

        let generics = if fork.peek::<Lt>().is_some() {
            Some(fork.parse::<AngleArgs>()?)
        } else {
            None
        };

        let _ = fork.parse::<Eq>()?;
        // Try to parse a type; if it fails this is not an assoc-type binding.
        let ty = fork.parse::<Type>()?;
        stream.seek(&fork);
        Ok(Self {
            span: Span::default(),
            ident,
            generics,
            ty,
        })
    }
}

impl ToTokens for AssocTypeArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        Eq::default().to_tokens(t);
        self.ty.to_tokens(t);
    }
}
