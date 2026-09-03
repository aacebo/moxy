use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, MacroCall};

/// A macro invocation inside a trait definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
    pub semi: Token![;],
}

impl Parse for TraitItemMacro {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let (mac, semi) = crate::MacroCall::parse_semi(parser)?;

        if semi.is_none() {
            return Err(LexError::new(mac.span()).message("expected ';'").into());
        }

        if let Some(semi) = semi {
            Ok(Self { attrs, mac, semi })
        } else {
            Err(LexError::new(mac.span()).message("expected ';'").into())
        }
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

impl TraitItemMacro {
    pub fn into_trait_item(self) -> super::TraitItem {
        super::TraitItem::from(self)
    }
}
