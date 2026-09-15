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
        cursor.peek::<MacroCall>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            mac: parser.parse()?,
            semi: parser.parse()?,
        })
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
