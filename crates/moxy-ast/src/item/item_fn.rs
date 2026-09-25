use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Signature, StmtBlock, Token, Visibility};

/// A free function item (`fn name(...) -> T { ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemFn {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub sig: Signature,
    pub body: Option<StmtBlock>,
    pub semi_punct: Option<Token![;]>,
}

impl Parse for ItemFn {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        Signature::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let sig = <_ as Parse>::parse(parser)?;
        let (body, semi_punct) = if StmtBlock::peek(parser.cursor()) {
            (Some(<_ as Parse>::parse(parser)?), None)
        } else {
            (None, Some(<_ as Parse>::parse(parser)?))
        };

        Ok(Self {
            attrs,
            vis,
            sig,
            body,
            semi_punct,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Signature::skip(cursor)?;

        if StmtBlock::peek(cursor) {
            StmtBlock::skip(cursor)
        } else {
            <Token![;]>::skip(cursor)
        }
    }
}

impl Spanner for ItemFn {
    fn span(&self) -> Span {
        self.attrs.span().join(
            self.body
                .as_ref()
                .map(Spanner::span)
                .or_else(|| self.semi_punct.map(|v| v.span()))
                .unwrap_or_else(|| self.sig.span()),
        )
    }
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}

impl ItemFn {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
