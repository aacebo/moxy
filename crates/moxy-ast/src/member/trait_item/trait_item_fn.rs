use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A method declaration or default implementation inside a trait definition.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemFn {
    pub attrs: Attributes,
    pub sig: Signature,
    pub body: Option<StmtBlock>,
    pub semi: Option<Token![;]>,
}

impl Parse for TraitItemFn {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| Signature::peek(cursor))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let sig = <_ as Parse>::parse(parser)?;
        let (body, semi) = if parser.is_delimited(Delim::Brace) {
            (Some(<_ as Parse>::parse(parser)?), None)
        } else {
            (None, Some(<_ as Parse>::parse(parser)?))
        };

        Ok(Self { attrs, sig, body, semi })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Signature::skip(cursor)?;

        if StmtBlock::peek(cursor) {
            StmtBlock::skip(cursor)
        } else {
            <Token![;]>::skip(cursor)
        }
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
