use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Lit, Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::Ident;

pub mod foreign_item;
pub mod impl_item;
pub mod trait_item;

pub use foreign_item::*;
pub use impl_item::*;
pub use trait_item::*;

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
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            Some(TokenTree::Literal(_)) => {
                let at = stream.span();
                let lit = stream.parse::<Lit>()?;

                if let Some(i) = lit.as_uint()
                    && i.repr().chars().all(char::is_alphabetic)
                {
                    return Err(LexError::new(at).message("expected tuple index").into());
                }

                if !lit.is_int() {
                    return Err(LexError::new(at).message("expected tuple index").into());
                }

                Ok(Self::Unnamed(lit))
            }
            _ => Ok(Self::Named(stream.parse()?)),
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
