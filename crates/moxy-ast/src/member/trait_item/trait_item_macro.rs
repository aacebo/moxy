use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, MacroCall};

/// A macro invocation inside a trait definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
    pub semi: Semi,
}

impl Parse for TraitItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let (mac, semi) = crate::MacroCall::parse_semi(stream)?;

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
