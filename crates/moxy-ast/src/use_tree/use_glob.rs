use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A glob import (`*`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseGlob {
    /// The leading `::` of a path, as in `use ::*;`.
    pub prefix: Option<Token![::]>,
    pub star: Token![*],
}

impl Parse for UseGlob {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![*]>::peek(super::skip_prefix(cursor))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            prefix: <_ as Parse>::parse(parser)?,
            star: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        <Token![*]>::skip(super::skip_prefix(cursor))
    }
}

impl Spanner for UseGlob {
    fn span(&self) -> Span {
        match self.prefix {
            Some(prefix) => prefix.span().join(self.star.span()),
            None => self.star.span(),
        }
    }
}

impl ToTokens for UseGlob {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.prefix.to_tokens(t);
        self.star.to_tokens(t);
    }
}
