use super::Item;
use crate::ast::{Attribute, Ident, Unsafety, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Mod;
use crate::token::punct::Semi;
use crate::token::{Delim, Group, ToTokens, TokenTree};
use crate::{Parse, Span, TokenStream};

#[doc = "A module item (`mod foo;` or `mod foo { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMod {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub ident: Ident,
    pub content: Option<Vec<Item>>,
}

impl Parse for ItemMod {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let unsafety = Unsafety::Safe;
        let _ = stream.parse::<Mod>()?;
        let ident = stream.parse::<Ident>()?;

        let content = if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let group = stream.parse_group(Delim::Brace)?;
            let mut inner = group.parse();
            Some(inner.parse_vec::<Item>()?)
        } else {
            let _ = stream.parse::<Semi>();
            None
        };

        Ok(ItemMod {
            span: Span::default(),
            attrs,
            vis,
            unsafety,
            ident,
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
        Mod::default().to_tokens(t);
        self.ident.to_tokens(t);
        match &self.content {
            Some(items) => {
                let mut inner = TokenStream::new();
                for it in items {
                    it.to_tokens(&mut inner);
                }
                t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
            }
            None => Semi::default().to_tokens(t),
        }
    }
}
