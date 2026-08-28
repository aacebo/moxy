use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::AngleArguments;
use crate::{GenericArgument, Ident, Type};

/// An associated type binding (`Item = T`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocTypeArgument {
    pub ident: Ident,
    pub generics: Option<AngleArguments>,
    pub eq_punct: Token![=],
    pub ty: Type,
}

impl AssocTypeArgument {
    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::AssocType(self.clone())
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocType(self)
    }
}

impl Parse for AssocTypeArgument {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            ident: stream.parse()?,
            generics: stream.parse_if(),
            eq_punct: stream.parse()?,
            ty: stream.parse()?,
        })
    }
}

impl Spanner for AssocTypeArgument {
    fn span(&self) -> Span {
        self.ident.span().join(self.ty.span())
    }
}

impl ToTokens for AssocTypeArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        self.eq_punct.to_tokens(t);
        self.ty.to_tokens(t);
    }
}
