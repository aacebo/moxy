use crate::ast::{Attribute, Ident};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::MacroRules;
use crate::token::punct::Not;
use crate::token::{Delim, Group, LexError, ToTokens, TokenTree};
use crate::{Parse, Span, TokenStream};

#[doc = "A `macro_rules!` definition item."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacroRules {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub rules: TokenStream,
}

impl Parse for ItemMacroRules {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let _ = stream.parse::<MacroRules>()?;
        let _ = stream.parse::<Not>()?;
        let ident = stream.parse::<Ident>()?;

        let rules = match stream.curr() {
            Some(TokenTree::Group(g)) => {
                let s = g.stream();
                stream.advance();
                s
            }
            _ => {
                return Err(LexError::new(stream.span()).message("expected macro body").into());
            }
        };

        Ok(ItemMacroRules {
            span: Span::default(),
            attrs,
            ident,
            rules,
        })
    }
}

impl ToTokens for ItemMacroRules {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        MacroRules::default().to_tokens(t);
        Not::default().to_tokens(t);
        self.ident.to_tokens(t);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, self.rules.clone())));
    }
}
