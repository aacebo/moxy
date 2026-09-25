use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Group, LexError, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::{Attributes, Ident};

/// A `macro_rules!` definition item.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacroRules {
    pub attrs: Attributes,
    pub macro_rules_keyword: Token![macro_rules],
    pub not_punct: Token![!],
    pub ident: Ident,
    pub body: Group,
}

impl Parse for ItemMacroRules {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| <Token![macro_rules]>::peek(cursor))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let macro_rules_keyword = parser.parse()?;
        let not_punct = parser.parse()?;
        let ident = parser.parse()?;
        let body = match parser.curr() {
            Some(TokenTree::Group(g)) => {
                let g = g.clone();
                parser.advance();
                g
            }

            _ => {
                return Err(LexError::new(parser.span()).message("expected macro body").into());
            }
        };

        Ok(Self {
            attrs,
            macro_rules_keyword,
            not_punct,
            ident,
            body,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = <Token![macro_rules]>::skip(cursor)?;
        cursor = <Token![!]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        Group::skip(cursor)
    }
}

impl Spanner for ItemMacroRules {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ItemMacroRules {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
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
