use crate::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Delim, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::{List, TypeBound};

/// A trait object type (e.g. `dyn Iterator<Item = u8>`, `dyn Fn() + 'a`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTraitObject {
    pub dyn_token: Option<Token![dyn]>,
    pub bounds: List<TypeBound, Token![+]>,
}

impl Parse for TypeTraitObject {
    fn peek(cursor: Cursor<'_>) -> bool {
        if <Token![dyn]>::peek(cursor) {
            return true;
        }

        // Parentheses that a `+` follows hold the first bound, as `Type` reads
        // them
        if crate::Type::peek_paren_bound_object(cursor) {
            return true;
        }

        // A function pointer type is no bound, though `fn` reads as a path
        // segment, and other parentheses hold a type
        if <Token![fn]>::peek(cursor)
            || <Token![unsafe]>::peek(cursor)
            || <Token![extern]>::peek(cursor)
            || cursor.is_delimited(Delim::Paren)
        {
            return false;
        }

        TypeBound::skip(cursor).is_some_and(|cursor| <Token![+]>::peek(cursor))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let dyn_token: Option<Token![dyn]> = <_ as Parse>::parse(parser)?;
        let bounds = match dyn_token {
            Some(_) => crate::TypeBound::parse_bounds_of_keyword(parser)?,
            None => crate::TypeBound::parse_bounds(parser)?,
        };
        Ok(Self { dyn_token, bounds })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        match <Token![dyn]>::skip(cursor) {
            Some(cursor) => TypeBound::skip_bounds_of_keyword(cursor),
            None => TypeBound::skip_bounds(cursor),
        }
    }
}

impl Spanner for TypeTraitObject {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for TypeTraitObject {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.dyn_token.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
