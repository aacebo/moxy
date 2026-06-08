use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Quote, Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

use crate::{Expr, Lifetime, Type};

mod angle_args;
mod assoc_const_arg;
mod assoc_type_arg;
mod constraint_arg;

pub use angle_args::*;
pub use assoc_const_arg::*;
pub use assoc_type_arg::*;
pub use constraint_arg::*;

/// A single generic argument inside `<...>`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Const(Expr),
    AssocType(AssocTypeArg),
    AssocConst(AssocConstArg),
    Constraint(ConstraintArg),
}

impl Spanner for GenericArgument {
    fn span(&self) -> Span {
        match self {
            GenericArgument::Lifetime(v) => v.span(),
            GenericArgument::Type(v) => v.span(),
            GenericArgument::Const(v) => v.span(),
            GenericArgument::AssocType(v) => v.span(),
            GenericArgument::AssocConst(v) => v.span(),
            GenericArgument::Constraint(v) => v.span(),
        }
    }
}

impl Parse for GenericArgument {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let token = match stream.curr() {
            None => return Err(ParseError::new(stream.span(), "eof")),
            Some(v) => v.clone(),
        };

        // Lifetime: starts with `'`.
        if token.is_punct_quote() {
            return Ok(GenericArgument::Lifetime(stream.parse()?));
        }

        // Constraint `ident [generics] : bounds` — must come before AssocType/AssocConst
        // because `:` is unambiguous.
        if let Ok(v) = stream.parse::<ConstraintArg>() {
            return Ok(v.into_generic_argument());
        }

        // Associated type binding `ident [generics] = Type`.
        if let Ok(v) = stream.parse::<AssocTypeArg>() {
            return Ok(v.into_generic_argument());
        }

        // Associated const binding `ident [generics] = expr`.
        if let Ok(v) = stream.parse::<AssocConstArg>() {
            return Ok(v.into_generic_argument());
        }

        // Literal or block expression const argument.
        let is_const = token.is_literal() || token.as_group().map(|g| g.delim().is_brace()).unwrap_or(false);
        // matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == moxy_token::Delim::Brace);

        if is_const {
            return Ok(GenericArgument::Const(stream.parse()?));
        }

        Ok(GenericArgument::Type(stream.parse()?))
    }
}

impl ToTokens for GenericArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            GenericArgument::Lifetime(v) => v.to_tokens(t),
            GenericArgument::Type(v) => v.to_tokens(t),
            GenericArgument::Const(v) => v.to_tokens(t),
            GenericArgument::AssocType(v) => v.to_tokens(t),
            GenericArgument::AssocConst(v) => v.to_tokens(t),
            GenericArgument::Constraint(v) => v.to_tokens(t),
        }
    }
}
