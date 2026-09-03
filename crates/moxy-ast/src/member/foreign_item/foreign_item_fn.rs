use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Signature, Visibility};

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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at = parser.span();
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;

        if !crate::sig::Signature::is_start(parser) {
            return Err(LexError::new(at).message("expected foreign fn").into());
        }

        let sig = parser.parse::<Signature>()?;
        let semi = parser.parse_if::<Token![;]>();
        Ok(Self { attrs, vis, sig, semi })
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

impl ForeignItemFn {
    pub fn into_foreign_item(self) -> super::ForeignItem {
        super::ForeignItem::from(self)
    }
}
