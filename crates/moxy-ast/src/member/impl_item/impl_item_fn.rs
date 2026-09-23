use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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
    pub body: StmtBlock,
}

impl Parse for ImplItemFn {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = cursor.skip::<Option<Token![default]>>().unwrap_or(cursor);
        cursor.peek::<Signature>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            vis: parser.parse()?,
            defaultness: parser.parse()?,
            sig: parser.parse()?,
            body: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = cursor.skip::<Option<Token![default]>>()?;
        cursor = cursor.skip::<Signature>()?;
        cursor.skip::<StmtBlock>()
    }
}

impl Spanner for ImplItemFn {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ImplItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
    }
}
