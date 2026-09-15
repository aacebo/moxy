use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A foreign function declaration inside an `extern` block.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemFn {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub sig: Signature,
    pub semi: Option<Token![;]>,
}

impl Parse for ForeignItemFn {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Signature>() || (cursor.peek::<Token![pub]>() && cursor.offset(1).peek::<Signature>())
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            vis: parser.parse()?,
            sig: parser.parse()?,
            semi: parser.parse()?,
        })
    }
}

impl Spanner for ForeignItemFn {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.sig.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ForeignItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.sig.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
