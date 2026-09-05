use crate::{Parse, ParseError, Parser};
use crate::{Peek, Token};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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

impl Peek for AssocTypeArgument {
    fn peek(parser: &Parser) -> bool {
        if !parser.parse::<Ident>().is_ok() {
            return false;
        }

        if !parser.parse::<Option<AngleArguments>>().is_ok() {
            return false;
        }

        if !parser.parse::<Token![=]>().is_ok() {
            return false;
        }

        true
    }
}

impl Parse for AssocTypeArgument {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            ident: parser.parse()?,
            generics: parser.parse_if(),
            eq_punct: parser.parse()?,
            ty: parser.parse()?,
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
