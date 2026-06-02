use super::AngleArgs;
use crate::ast::{Expr, Ident};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::{Eq, Lt};
use crate::{Parse, Span, TokenStream};

#[doc = "An associated const binding (`N = 8`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocConstArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub expr: Expr,
}

impl Parse for AssocConstArg {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let mut fork = stream.fork();
        let ident = fork.parse::<Ident>()?;

        let generics = if fork.peek::<Lt>().is_some() {
            Some(fork.parse::<AngleArgs>()?)
        } else {
            None
        };

        let _ = fork.parse::<Eq>()?;
        let expr = fork.parse::<Expr>()?;
        stream.seek(&fork);
        Ok(Self {
            span: Span::default(),
            ident,
            generics,
            expr,
        })
    }
}

impl ToTokens for AssocConstArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        Eq::default().to_tokens(t);
        self.expr.to_tokens(t);
    }
}
