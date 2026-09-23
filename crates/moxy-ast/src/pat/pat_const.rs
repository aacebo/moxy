use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A const pattern, e.g. `const STATIC: usize = 1`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatConst {
    pub attrs: Attributes,
    pub keyword: Token![const],
    pub block: StmtBlock,
}

impl Spanner for PatConst {
    fn span(&self) -> Span {
        self.attrs.span().join(self.block.span())
    }
}

impl Parse for PatConst {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![const]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            keyword: parser.parse()?,
            block: parser.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Attributes::skip(cursor)?.skip::<Token![const]>()?.skip::<StmtBlock>()
    }
}

impl ToTokens for PatConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.keyword.to_tokens(t);
        self.block.to_tokens(t);
    }
}
