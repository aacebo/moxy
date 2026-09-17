mod foreign_item;
mod impl_item;
mod trait_item;

pub use foreign_item::*;
pub use impl_item::*;
pub use trait_item::*;

use moxy_token::{Lit, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct/tuple field accessor — a named field (`.field`) or a tuple index (`.0`).
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Parse for Member {
    fn peek(cursor: Cursor<'_>) -> bool {
        if cursor.peek::<Ident>() {
            return true;
        }

        let Some(moxy_token::TokenTree::Literal(Lit::Int(value))) = cursor.curr() else {
            return false;
        };

        value.repr().chars().all(char::is_numeric) && value.value() <= 4294967295
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Lit>() {
            let lit: Lit = parser.parse()?;

            if let Some(i) = lit.as_int() {
                if !i.repr().chars().all(char::is_numeric) {
                    parser.error("expected tuple index").into()
                } else if i.value() > 4294967295 {
                    parser.error("tuple index exceeds max size 4294967295").into()
                } else {
                    Ok(Self::Unnamed(lit))
                }
            } else {
                parser.error("expected tuple index").into()
            }
        } else {
            Ok(Self::Named(parser.parse()?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if !Self::peek(cursor) {
            return None;
        }

        if cursor.peek::<Lit>() {
            cursor.skip::<Lit>()
        } else {
            cursor.skip::<Ident>()
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
