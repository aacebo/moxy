use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::AngleArguments;
use crate::{Expr, GenericArgument, Ident};

/// An associated const binding (`N = 8`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocConstArgument {
    pub ident: Ident,
    pub generics: Option<AngleArguments>,
    pub eq_punct: Token![=],
    pub expr: Expr,
}

impl AssocConstArgument {
    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::AssocConst(self.clone())
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocConst(self)
    }
}

impl Parse for AssocConstArgument {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            ident: stream.parse()?,
            generics: stream.parse_if(),
            eq_punct: stream.parse()?,
            expr: stream.parse()?,
        })
    }
}

impl Spanner for AssocConstArgument {
    fn span(&self) -> Span {
        self.ident.span().join(self.expr.span())
    }
}

impl ToTokens for AssocConstArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        self.eq_punct.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
