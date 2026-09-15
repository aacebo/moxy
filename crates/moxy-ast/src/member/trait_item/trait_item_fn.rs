use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A method declaration or default implementation inside a trait definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemFn {
    pub attrs: Attributes,
    pub sig: Signature,
    pub body: Option<StmtBlock>,
    pub semi: Option<Token![;]>,
}

impl Parse for TraitItemFn {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Signature>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let sig = parser.parse()?;
        let (body, semi) = if parser.is_delimited(Delim::Brace) {
            (Some(parser.parse()?), None)
        } else {
            (None, Some(parser.parse()?))
        };

        Ok(Self { attrs, sig, body, semi })
    }
}

impl Spanner for TraitItemFn {
    fn span(&self) -> Span {
        let end = self
            .body
            .as_ref()
            .map(|b| b.span())
            .or_else(|| self.semi.as_ref().map(|s| s.span()))
            .unwrap_or_else(|| self.sig.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for TraitItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
