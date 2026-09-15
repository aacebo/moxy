use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A macro invocation inside an `extern` block.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
    pub semi: Option<Token![;]>,
}

impl Parse for ForeignItemMacro {
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

impl Spanner for ForeignItemMacro {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.mac.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ForeignItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
