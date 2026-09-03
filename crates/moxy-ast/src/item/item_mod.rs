use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream, TokenTree};

use super::Item;
use crate::{Attributes, Delimited, Ident, Unsafety, Visibility};

/// A module item (`mod foo;` or `mod foo { ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMod {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub mod_keyword: Token![mod],
    pub ident: Ident,
    pub content: Option<Delimited<Vec<Item>>>,
    pub semi_punct: Option<Token![;]>,
}

impl Parse for ItemMod {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let unsafety = parser.parse_if::<Unsafety>().unwrap_or(Unsafety::Safe);
        let mod_keyword = parser.parse::<Token![mod]>()?;
        let ident = parser.parse::<Ident>()?;
        let (content, semi_punct) = if matches!(parser.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let brace = Delimited::<Vec<Item>>::parse_brace(parser)?;
            (Some(brace), None)
        } else {
            let semi_punct = parser.parse::<Token![;]>()?;
            (None, Some(semi_punct))
        };

        Ok(Self {
            attrs,
            vis,
            unsafety,
            mod_keyword,
            ident,
            content,
            semi_punct,
        })
    }
}

impl Spanner for ItemMod {
    fn span(&self) -> Span {
        let end = if let Some(c) = &self.content {
            c.span()
        } else if let Some(s) = &self.semi_punct {
            s.span()
        } else {
            self.mod_keyword.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for ItemMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.mod_keyword.to_tokens(t);
        self.ident.to_tokens(t);

        match &self.content {
            Some(brace) => brace.to_tokens(t),
            None => self.semi_punct.to_tokens(t),
        }
    }
}

impl ItemMod {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
