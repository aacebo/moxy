use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, MacroCall};

/// A macro invocation inside an `impl` block.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
    pub semi: Option<Semi>,
}

impl Parse for ImplItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let (mac, semi) = crate::MacroCall::parse_semi(stream)?;
        Ok(ImplItemMacro { attrs, mac, semi })
    }
}

impl Spanner for ImplItemMacro {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.mac.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ImplItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}

impl ImplItemMacro {
    pub fn into_impl_item(self) -> super::ImplItem {
        super::ImplItem::from(self)
    }
}
