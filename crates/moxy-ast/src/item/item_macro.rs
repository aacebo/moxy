use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, MacroCall};

/// A macro invocation used as an item (`name!(...);`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacro {
    pub attrs: Attributes,
    pub call: MacroCall,
    pub semi_punct: Option<Token![;]>,
}

impl Parse for ItemMacro {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| cursor.peek::<MacroCall>())
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let call = parser.parse()?;
        let semi_punct = parser.parse()?;
        Ok(Self { attrs, call, semi_punct })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = cursor.skip::<MacroCall>()?;
        cursor.skip::<Option<Token![;]>>()
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
        self.semi_punct.to_tokens(t);
    }
}

impl ItemMacro {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
