use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

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
    Unnamed(u32, Span),
}

impl Member {
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    pub fn is_unnamed(&self) -> bool {
        matches!(self, Self::Unnamed(..))
    }

    pub fn as_named(&self) -> Option<&Ident> {
        if let Self::Named(v) = self { Some(v) } else { None }
    }
}

impl Spanner for Member {
    fn span(&self) -> Span {
        match self {
            Member::Named(id) => id.span(),
            Member::Unnamed(_, span) => *span,
        }
    }
}

impl Parse for Member {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match stream.curr() {
            Some(TokenTree::Literal(lit)) => {
                let span = lit.span();
                let index = lit
                    .repr()
                    .parse::<u32>()
                    .map_err(|_| ParseError::from(LexError::new(at).message("expected tuple index")))?;
                stream.advance();
                Ok(Member::Unnamed(index, span))
            }
            _ => Ok(Member::Named(stream.parse()?)),
        }
    }
}

impl ToTokens for Member {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Member::Named(ident) => ident.to_tokens(tokens),
            Member::Unnamed(index, span) => {
                moxy_token::Literal::from_repr(&index.to_string(), *span).to_tokens(tokens);
            }
        }
    }
}
