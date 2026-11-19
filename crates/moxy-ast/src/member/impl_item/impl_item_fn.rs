use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A method or associated function inside an `impl` block.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemFn {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub defaultness: Option<Token![default]>,
    pub sig: Signature,
    /// The body, or none before a `;`, which rustc's parser accepts and the
    /// compiler rejects later.
    pub body: Option<StmtBlock>,
    pub semi: Option<Token![;]>,
}

impl Attributed for ImplItemFn {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ImplItemFn {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![default]>::skip(cursor).unwrap_or(cursor);
        Signature::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let defaultness = <_ as Parse>::parse(parser)?;
        let sig = Signature::parse_with(parser, false)?;
        let (body, semi) = if StmtBlock::peek(parser.cursor()) {
            (Some(<_ as Parse>::parse(parser)?), None)
        } else {
            (None, Some(<Token![;]>::parse(parser)?))
        };

        Ok(Self {
            attrs,
            vis,
            defaultness,
            sig,
            body,
            semi,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Option::<Token![default]>::skip(cursor)?;
        cursor = Signature::skip_with(cursor, false)?;

        if StmtBlock::peek(cursor) {
            StmtBlock::skip(cursor)
        } else {
            <Token![;]>::skip(cursor)
        }
    }
}

impl Spanner for ImplItemFn {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ImplItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
