use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::AngleArguments;
use crate::{GenericArgument, Ident, Punctuated, TypeBound};

/// An associated type bound constraint (`Item: Bound`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstraintArgument {
    pub ident: Ident,
    pub generics: Option<AngleArguments>,
    pub colon_punct: Token![:],
    pub bounds: Punctuated<TypeBound, Token![+]>,
}

impl ConstraintArgument {
    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::Constraint(self.clone())
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::Constraint(self)
    }
}

impl Parse for ConstraintArgument {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            ident: stream.parse()?,
            generics: stream.parse_if(),
            colon_punct: stream.parse()?,
            bounds: Punctuated::parse_separated_nonempty(stream)?,
        })
    }
}

impl Spanner for ConstraintArgument {
    fn span(&self) -> Span {
        let end = self
            .bounds
            .last()
            .map(|b| b.span())
            .unwrap_or_else(|| self.colon_punct.span());
        self.ident.span().join(end)
    }
}

impl ToTokens for ConstraintArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
