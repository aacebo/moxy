use moxy_token::{Group, Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::path::PathArguments;
use crate::ty::{TypeMacro, TypePath};
use crate::{
    Cursor, Expr, Ident, Lifetime, MacroCall, Parse, ParseError, Parser, Path, PathSegment, Punctuated, Token, Type, TypeBound,
};

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
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
    fn peek(cursor: Cursor<'_>) -> bool {
        if Lifetime::peek(cursor) {
            return true;
        }

        let is_const = match cursor.curr() {
            Some(TokenTree::Literal(_)) => true,
            Some(TokenTree::Group(group)) => group.delim.is_brace(),
            Some(TokenTree::Punct(Punct::Minus(_) | Punct::Not(_))) => true,
            _ => false,
        };

        if is_const {
            return true;
        }

        Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let token = match parser.cursor().curr() {
            None => return Err(ParseError::new(parser.span(), "eof")),
            Some(v) => v.clone(),
        };

        // Lifetime: starts with `'`.
        if token.is_punct_quote() {
            return Ok(Self::Lifetime(<_ as Parse>::parse(parser)?));
        }

        // Literal or block expression const argument.
        let is_const = token.is_literal()
            || token.as_group().map(|g| g.delim.is_brace()).unwrap_or(false)
            || token.is_punct_minus()
            || token.is_punct_not();

        if is_const {
            return Ok(Self::Const(<_ as Parse>::parse(parser)?));
        }

        if token.is_ident() {
            let ident = <_ as Parse>::parse(parser)?;
            let generics: Option<AngleArguments> = <_ as Parse>::parse(parser)?;

            if <Token![:]>::peek(parser.cursor()) {
                let colon_punct = <_ as Parse>::parse(parser)?;
                let bounds = Punctuated::parse_separated_nonempty(parser)?;

                return Ok(ConstraintArgument {
                    ident,
                    generics,
                    colon_punct,
                    bounds,
                }
                .into_generic_argument());
            }

            if <Token![=]>::peek(parser.cursor()) {
                let eq_punct = <_ as Parse>::parse(parser)?;
                let is_const = match parser.cursor().curr() {
                    Some(TokenTree::Literal(_)) => true,
                    Some(TokenTree::Group(g)) if g.delim.is_brace() => true,
                    Some(TokenTree::Punct(Punct::Minus(_))) => true,
                    Some(TokenTree::Punct(Punct::Not(_))) => true,
                    _ => false,
                };

                if is_const {
                    let expr = <_ as Parse>::parse(parser)?;

                    return Ok(AssocConstArgument {
                        ident,
                        generics,
                        eq_punct,
                        expr,
                    }
                    .into_generic_argument());
                }

                let ty = <_ as Parse>::parse(parser)?;

                return Ok(AssocTypeArgument {
                    ident,
                    generics,
                    eq_punct,
                    ty,
                }
                .into_generic_argument());
            }

            let args = generics.map_or(PathArguments::None, PathArguments::AngleBracketed);
            let path = Path::parse_rest(parser, PathSegment { ident, args })?;
            let ty = if <Token![!]>::peek(parser.cursor()) {
                Type::Macro(TypeMacro {
                    mac: MacroCall {
                        path,
                        bang: <_ as Parse>::parse(parser)?,
                        body: <_ as Parse>::parse(parser)?,
                    },
                })
            } else {
                Type::Path(TypePath { qself: None, path })
            };

            return Ok(Self::Type(ty));
        }

        Ok(Self::Type(<_ as Parse>::parse(parser)?))
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if Lifetime::peek(cursor) {
            return Lifetime::skip(cursor);
        }

        let is_const = match cursor.curr() {
            Some(TokenTree::Literal(_)) => true,
            Some(TokenTree::Group(group)) => group.delim.is_brace(),
            Some(TokenTree::Punct(Punct::Minus(_) | Punct::Not(_))) => true,
            _ => false,
        };

        if is_const {
            return Expr::skip(cursor);
        }

        if Ident::peek(cursor) {
            cursor = Ident::skip(cursor)?;
            cursor = Option::<AngleArguments>::skip(cursor)?;

            if <Token![:]>::peek(cursor) {
                cursor = <Token![:]>::skip(cursor)?;
                cursor = TypeBound::skip(cursor)?;

                while <Token![+]>::peek(cursor) {
                    cursor = <Token![+]>::skip(cursor)?;
                    cursor = TypeBound::skip(cursor)?;
                }

                return Some(cursor);
            }

            if <Token![=]>::peek(cursor) {
                cursor = <Token![=]>::skip(cursor)?;

                let is_const = match cursor.curr() {
                    Some(TokenTree::Literal(_)) => true,
                    Some(TokenTree::Group(group)) => group.delim.is_brace(),
                    Some(TokenTree::Punct(Punct::Minus(_) | Punct::Not(_))) => true,
                    _ => false,
                };

                return if is_const { Expr::skip(cursor) } else { Type::skip(cursor) };
            }

            cursor = Path::skip_rest(cursor)?;

            if <Token![!]>::peek(cursor) {
                cursor = <Token![!]>::skip(cursor)?;
                cursor = Group::skip(cursor)?;
            }

            return Some(cursor);
        }

        Type::skip(cursor)
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
