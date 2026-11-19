mod foreign_item;
mod impl_item;
mod trait_item;

pub use foreign_item::*;
pub use impl_item::*;
pub use trait_item::*;

use moxy_token::{Ident, Lit, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct/tuple field accessor — a named field (`.field`) or a tuple index (`.0`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Member {
    Named(Ident),
    Unnamed(Lit),
}

impl Member {
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    pub fn is_unnamed(&self) -> bool {
        matches!(self, Self::Unnamed(_))
    }

    pub fn as_named(&self) -> Option<&Ident> {
        if let Self::Named(v) = self { Some(v) } else { None }
    }

    pub fn as_unnamed(&self) -> Option<&Lit> {
        if let Self::Unnamed(v) = self { Some(v) } else { None }
    }
}

impl From<Ident> for Member {
    fn from(v: Ident) -> Self {
        Self::Named(v)
    }
}

impl From<u32> for Member {
    fn from(v: u32) -> Self {
        Self::Unnamed(Lit::u32_unsuffixed(v))
    }
}

impl Spanner for Member {
    fn span(&self) -> Span {
        match self {
            Self::Named(id) => id.span(),
            Self::Unnamed(idx) => idx.span(),
        }
    }
}

// As in rustc's parser, any integer literal without a suffix is a tuple
// index, as `0x1` in `a.0x1` and `S { 0x1: a }`; the compiler checks the
// value later. After a field dot a number with an exponent is one too, which
// field access reads itself
fn is_tuple_index(lit: &Lit) -> bool {
    matches!(lit, Lit::Int(int) if int.suffix() == moxy_token::IntSuffix::None)
}

impl Parse for Member {
    fn peek(cursor: Cursor<'_>) -> bool {
        if Ident::peek(cursor) {
            return true;
        }

        matches!(cursor.curr(), Some(moxy_token::TokenTree::Literal(lit)) if is_tuple_index(lit))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if Lit::peek(parser.cursor()) {
            let lit: Lit = <_ as Parse>::parse(parser)?;

            if is_tuple_index(&lit) {
                Ok(Self::Unnamed(lit))
            } else {
                parser.error("expected tuple index").into()
            }
        } else {
            Ok(Self::Named(<_ as Parse>::parse(parser)?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if !Self::peek(cursor) {
            return None;
        }

        if Lit::peek(cursor) {
            Lit::skip(cursor)
        } else {
            Ident::skip(cursor)
        }
    }
}

impl ToTokens for Member {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Named(ident) => ident.to_tokens(tokens),
            Self::Unnamed(idx) => idx.to_tokens(tokens),
        }
    }
}

// The levels a member of an impl, trait, or extern block takes beyond its own,
// in parsing and in skipping: the body of a function and the items inside it
// take that much more stack
pub(crate) const EXTRA_LEVELS: usize = 2;
