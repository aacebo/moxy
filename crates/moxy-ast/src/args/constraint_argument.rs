use crate::{Parse, ParseError, Parser};
use crate::{Peek, Token};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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

impl Peek for ConstraintArgument {
    fn peek(parser: &Parser) -> bool {
        if !parser.parse::<Ident>().is_ok() {
            return false;
        }

        if !parser.parse::<Option<AngleArguments>>().is_ok() {
            return false;
        }

        if !parser.parse::<Token![:]>().is_ok() {
            return false;
        }

        true
    }
}

impl Parse for ConstraintArgument {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            ident: parser.parse()?,
            generics: parser.parse_if(),
            colon_punct: parser.parse()?,
            bounds: Punctuated::parse_separated_nonempty(parser)?,
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
