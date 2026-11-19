use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream, Underscore};

use crate::*;

/// A wild pattern, e.g. `_`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatWild {
    pub attrs: Attributes,
    pub token: Token![_],
}

impl Attributed for PatWild {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Spanner for PatWild {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl Parse for PatWild {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        <Token![_]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            token: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        <Token![_]>::skip(Attributes::skip(cursor)?)
    }
}

impl ToTokens for PatWild {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        // An identifier, as the lexer and the compiler give `_`
        Ident::new(Underscore::TEXT).with_span(self.token.span()).to_tokens(t);
    }
}
