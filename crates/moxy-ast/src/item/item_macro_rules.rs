use moxy_token::keyword::MacroRules;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Not;
use moxy_token::{Group, LexError, Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::{Attributes, Ident};

/// A `macro_rules!` definition item.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacroRules {
    pub attrs: Attributes,
    pub macro_rules_keyword: MacroRules,
    pub not_punct: Not,
    pub ident: Ident,
    pub body: Group,
}

impl Parse for ItemMacroRules {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let macro_rules_keyword = stream.parse::<MacroRules>()?;
        let not_punct = stream.parse::<Not>()?;
        let ident = stream.parse::<Ident>()?;

        let body = match stream.curr() {
            Some(TokenTree::Group(g)) => {
                let g = g.clone();
                stream.advance();
                g
            }
            _ => {
                return Err(LexError::new(stream.span()).message("expected macro body").into());
            }
        };

        Ok(ItemMacroRules {
            attrs,
            macro_rules_keyword,
            not_punct,
            ident,
            body,
        })
    }
}

impl Spanner for ItemMacroRules {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.macro_rules_keyword.span()
        };
        start.join(self.body.span().into())
    }
}

impl ToTokens for ItemMacroRules {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.macro_rules_keyword.to_tokens(t);
        self.not_punct.to_tokens(t);
        self.ident.to_tokens(t);
        t.extend_one(TokenTree::Group(self.body.clone()));
    }
}

impl ItemMacroRules {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
