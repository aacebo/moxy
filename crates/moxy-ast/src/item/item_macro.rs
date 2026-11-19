use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A macro invocation used as an item (`name!(...);`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacro {
    pub attrs: Attributes,
    pub call: MacroCall,
    pub semi_punct: Option<Token![;]>,
}

impl Attributed for ItemMacro {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ItemMacro {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| MacroCall::peek(cursor))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let call: MacroCall = <_ as Parse>::parse(parser)?;

        // As in rustc, a call in braces ends the item, and one in parentheses
        // or brackets ends with `;`
        let semi_punct = if call.body.delim.is_brace() {
            None
        } else {
            Some(<Token![;]>::parse(parser)?)
        };

        Ok(Self { attrs, call, semi_punct })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        if !Self::peek(cursor) {
            return None;
        }

        cursor = Attributes::skip(cursor)?;
        cursor = MacroCall::skip(cursor)?;

        if matches!(cursor.prev(), Some(moxy_token::TokenTree::Group(group)) if group.delim.is_brace()) {
            Some(cursor)
        } else {
            <Token![;]>::skip(cursor)
        }
    }
}

impl Spanner for ItemMacro {
    fn span(&self) -> Span {
        self.to_token_stream().span()
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
