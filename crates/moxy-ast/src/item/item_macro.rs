use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, MacroCall};

/// A macro invocation used as an item (`name!(...);`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacro {
    pub attrs: Attributes,
    pub call: MacroCall,
    pub semi_punct: Option<Token![;]>,
}

impl Parse for ItemMacro {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let call = parser.parse::<MacroCall>()?;
        let semi_punct = parser.parse_if::<Token![;]>();
        Ok(Self { attrs, call, semi_punct })
    }
}

impl Spanner for ItemMacro {
    fn span(&self) -> Span {
        let end = self.semi_punct.as_ref().map(|s| s.span()).unwrap_or_else(|| self.call.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.call.to_tokens(t);

        if let Some(semi_punct) = &self.semi_punct {
            semi_punct.to_tokens(t);
        }
    }
}

impl ItemMacro {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
