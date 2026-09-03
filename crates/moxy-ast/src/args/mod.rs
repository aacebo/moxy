use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Expr, Lifetime, Type};

mod angle_arguments;
mod assoc_const_argument;
mod assoc_type_argument;
mod constraint_argument;
mod paren_arguments;

pub use angle_arguments::*;
pub use assoc_const_argument::*;
pub use assoc_type_argument::*;
pub use constraint_argument::*;
pub use paren_arguments::*;

/// A single generic argument inside `<...>`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Const(Expr),
    AssocType(AssocTypeArgument),
    AssocConst(AssocConstArgument),
    Constraint(ConstraintArgument),
}

impl Spanner for GenericArgument {
    fn span(&self) -> Span {
        match self {
            Self::Lifetime(v) => v.span(),
            Self::Type(v) => v.span(),
            Self::Const(v) => v.span(),
            Self::AssocType(v) => v.span(),
            Self::AssocConst(v) => v.span(),
            Self::Constraint(v) => v.span(),
        }
    }
}

impl Parse for GenericArgument {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let token = match parser.curr() {
            None => return Err(ParseError::new(parser.span(), "eof")),
            Some(v) => v.clone(),
        };

        // Lifetime: starts with `'`.
        if token.is_punct_quote() {
            return Ok(Self::Lifetime(parser.parse()?));
        }

        // Constraint `ident [generics] : bounds` — must come before AssocType/AssocConst
        // because `:` is unambiguous.
        if let Some(argument) = parser.parse_if::<ConstraintArgument>() {
            return Ok(argument.into_generic_argument());
        }

        // Associated type binding `ident [generics] = Type`.
        if let Some(argument) = parser.parse_if::<AssocTypeArgument>() {
            return Ok(argument.into_generic_argument());
        }

        // Associated const binding `ident [generics] = expr`.
        if let Some(argument) = parser.parse_if::<AssocConstArgument>() {
            return Ok(argument.into_generic_argument());
        }

        // Literal or block expression const argument.
        let is_const = token.is_literal() || token.as_group().map(|g| g.delim().is_brace()).unwrap_or(false);

        if is_const {
            return Ok(Self::Const(parser.parse()?));
        }

        Ok(Self::Type(parser.parse()?))
    }
}

impl ToTokens for GenericArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Lifetime(v) => v.to_tokens(t),
            Self::Type(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
            Self::AssocType(v) => v.to_tokens(t),
            Self::AssocConst(v) => v.to_tokens(t),
            Self::Constraint(v) => v.to_tokens(t),
        }
    }
}
