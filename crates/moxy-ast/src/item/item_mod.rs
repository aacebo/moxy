use moxy_token::keyword::Mod;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Delim, Parse, Span, ToTokens, TokenStream, TokenTree};

use super::Item;
use crate::{Attribute, Delimited, Ident, Unsafety, Visibility};

#[doc = "A module item (`mod foo;` or `mod foo { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMod {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub mod_keyword: Mod,
    pub ident: Ident,
    pub content: Option<Delimited<Vec<Item>>>,
    pub semi_punct: Option<Semi>,
}

impl Parse for ItemMod {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let unsafety = Unsafety::Safe;
        let mod_keyword = stream.parse::<Mod>()?;
        let ident = stream.parse::<Ident>()?;

        let (content, semi_punct) = if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let brace = Delimited::<Vec<Item>>::parse_brace(stream)?;
            (Some(brace), None)
        } else {
            let semi_punct = stream.parse::<Semi>()?;
            (None, Some(semi_punct))
        };

        Ok(ItemMod {
            span: Span::default(),
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

impl ToTokens for ItemMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.mod_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        match &self.content {
            Some(brace) => brace.to_tokens(t),
            None => self.semi_punct.to_tokens(t),
        }
    }
}
