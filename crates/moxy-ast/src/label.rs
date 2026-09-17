use moxy_token::{Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A loop label (`'outer:`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Label {
    pub name: Lifetime,
    pub colon: Token![:],
}

impl Parse for Label {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = cursor.skip::<Lifetime>() else {
            return false;
        };

        cursor.peek::<Token![:]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let name = parser.parse()?;
        let colon = parser.parse()?;
        Ok(Self { name, colon })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<Lifetime>()?.skip::<Token![:]>()
    }
}

impl Spanner for Label {
    fn span(&self) -> Span {
        self.name.span().join(self.colon.span())
    }
}

impl ToTokens for Label {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.colon.to_tokens(tokens);
    }
}

impl Label {
    /// Returns `true` when the parser is positioned at a lifetime (`'a`) directly
    /// followed by `:`, which signals a loop/block label.
    pub fn is_prefix(parser: &Parser) -> bool {
        matches!(parser.curr(), Some(TokenTree::Punct(Punct::Quote(_))))
            && matches!(parser.nth(2), Some(TokenTree::Punct(Punct::Colon(_))))
    }
}
