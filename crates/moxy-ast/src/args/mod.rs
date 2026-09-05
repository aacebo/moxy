use crate::{Parse, ParseError, Parser, Punctuated, Token};
use moxy_token::{Ident, Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

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

        // Literal or block expression const argument.
        let is_const = token.is_literal() || token.as_group().map(|g| g.delim().is_brace()).unwrap_or(false);

        if is_const {
            return Ok(Self::Const(parser.parse()?));
        }

        if token.is_ident() {
            let fork = parser.fork();
            let ident = fork.parse::<Ident>()?;
            let generics = fork.parse_if::<AngleArguments>();

            if let Ok(colon_punct) = fork.parse::<Token![:]>() {
                let bounds = Punctuated::parse_separated_nonempty(&fork)?;
                parser.seek(&fork);
                return Ok(ConstraintArgument {
                    ident,
                    generics,
                    colon_punct,
                    bounds,
                }
                .into_generic_argument());
            }

            if let Ok(eq_punct) = fork.parse::<Token![=]>() {
                let is_const = match fork.curr() {
                    Some(TokenTree::Literal(_)) => true,
                    Some(TokenTree::Group(g)) if g.delim().is_brace() => true,
                    Some(TokenTree::Punct(Punct::Minus(_))) => true,
                    Some(TokenTree::Punct(Punct::Not(_))) => true,
                    _ => false,
                };

                if is_const {
                    let expr = fork.parse()?;
                    parser.seek(&fork);
                    return Ok(AssocConstArgument {
                        ident,
                        generics,
                        eq_punct,
                        expr,
                    }
                    .into_generic_argument());
                }

                let ty = fork.parse()?;
                parser.seek(&fork);
                return Ok(AssocTypeArgument {
                    ident,
                    generics,
                    eq_punct,
                    ty,
                }
                .into_generic_argument());
            }
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
