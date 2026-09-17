use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A macro invocation inside a trait definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
    pub semi: Token![;],
}

impl Parse for TraitItemMacro {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| cursor.peek::<MacroCall>())
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            mac: parser.parse()?,
            semi: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = cursor.skip::<MacroCall>()?;
        cursor.skip::<Token![;]>()
    }
}

impl Spanner for TraitItemMacro {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi.span())
    }
}

impl ToTokens for TraitItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
