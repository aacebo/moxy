use crate::ast::{Lifetime, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::{ToTokens, Token, TokenTree};
use crate::{Parse, TokenStream};

mod angle_args;
mod assoc_const_arg;
mod assoc_type_arg;
mod constraint_arg;

pub use angle_args::*;
pub use assoc_const_arg::*;
pub use assoc_type_arg::*;
pub use constraint_arg::*;

#[doc = "A single generic argument inside `<...>`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Const(crate::ast::Expr),
    AssocType(AssocTypeArg),
    AssocConst(AssocConstArg),
    Constraint(ConstraintArg),
}

impl Parse for GenericArgument {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // Lifetime: starts with `'`.
        if matches!(
            stream.curr(),
            Some(TokenTree::Token(Token::Punct(crate::token::Punctuation::Quote(_))))
        ) {
            return Ok(GenericArgument::Lifetime(stream.parse()?));
        }

        // Constraint `ident [generics] : bounds` — must come before AssocType/AssocConst
        // because `:` is unambiguous.
        if stream.peek::<ConstraintArg>().is_some() {
            return Ok(GenericArgument::Constraint(stream.parse()?));
        }

        // Associated type binding `ident [generics] = Type`.
        if stream.peek::<AssocTypeArg>().is_some() {
            return Ok(GenericArgument::AssocType(stream.parse()?));
        }

        // Associated const binding `ident [generics] = expr`.
        if stream.peek::<AssocConstArg>().is_some() {
            return Ok(GenericArgument::AssocConst(stream.parse()?));
        }

        // Literal or block expression const argument.
        let is_const = matches!(stream.curr(), Some(TokenTree::Token(Token::Literal(_))))
            || matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == crate::token::Delim::Brace);

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
