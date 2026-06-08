use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Eq, Lt};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::AngleArgs;
use crate::{Expr, GenericArgument, Ident};

/// An associated const binding (`N = 8`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocConstArg {
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub eq_punct: Eq,
    pub expr: Expr,
}

impl AssocConstArg {
    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::AssocConst(self.clone())
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocConst(self)
    }
}

impl Parse for AssocConstArg {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            ident: stream.parse()?,
            generics: stream.parse_if(),
            eq_punct: stream.parse()?,
            expr: stream.parse()?,
        })
    }
}

impl Spanner for AssocConstArg {
    fn span(&self) -> Span {
        self.ident.span().join(self.expr.span())
    }
}

impl ToTokens for AssocConstArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        self.eq_punct.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
