use moxy_token::keyword::Mod;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Delim, Group, Parse, Span, ToTokens, TokenStream, TokenTree};

use super::Item;
use crate::{Attribute, Ident, Unsafety, Visibility};

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
    pub semi_punct: Semi,
    pub content: Option<Vec<Item>>,
}

impl Parse for ItemMod {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let unsafety = Unsafety::Safe;
        let mod_keyword = stream.parse::<Mod>()?;
        let ident = stream.parse::<Ident>()?;

        let (semi_punct, content) = if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let group = stream.parse_group(Delim::Brace)?;
            let mut inner = group.parse();
            let items = inner.parse::<Vec<Item>>()?;
            (Semi::default(), Some(items))
        } else {
            let semi_punct = stream.parse::<Semi>()?;
            (semi_punct, None)
        };

        Ok(ItemMod {
            span: Span::default(),
            attrs,
            vis,
            unsafety,
            mod_keyword,
            ident,
            semi_punct,
            content,
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
            Some(items) => {
                let mut inner = TokenStream::new();
                for it in items {
                    it.to_tokens(&mut inner);
                }
                t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
            }
            None => self.semi_punct.to_tokens(t),
        }
    }
}
