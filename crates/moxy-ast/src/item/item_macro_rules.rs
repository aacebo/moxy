use moxy_token::keyword::MacroRules;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Not;
use moxy_token::{Group, LexError, Parse, Span, ToTokens, TokenStream, TokenTree};

use crate::{Attribute, Ident};

#[doc = "A `macro_rules!` definition item."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacroRules {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub macro_rules_keyword: MacroRules,
    pub not_punct: Not,
    pub ident: Ident,
    pub body: Group,
}

impl Parse for ItemMacroRules {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
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
            span: Span::default(),
            attrs,
            macro_rules_keyword,
            not_punct,
            ident,
            body,
        })
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
