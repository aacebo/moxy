use moxy_token::keyword::Static;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq, Semi};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Expr, Ident, Mutability, Type, Visibility};

/// A static item (`static [mut] NAME: Type = expr;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStatic {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_keyword: Static,
    pub mutability: Mutability,
    pub ident: Ident,
    pub colon_punct: Colon,
    pub ty: Type,
    pub eq_punct: Eq,
    pub expr: Expr,
    pub semi_punct: Semi,
}

impl Parse for ItemStatic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let static_keyword = stream.parse::<Static>()?;
        let mutability = stream.parse::<Mutability>()?;
        let ident = stream.parse::<Ident>()?;
        let colon_punct = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;
        let eq_punct = stream.parse::<Eq>()?;
        let expr = stream.parse::<Expr>()?;
        let semi_punct = stream.parse::<Semi>()?;
        Ok(ItemStatic {
            attrs,
            vis,
            static_keyword,
            mutability,
            ident,
            colon_punct,
            ty,
            eq_punct,
            expr,
            semi_punct,
        })
    }
}

impl Spanner for ItemStatic {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if !matches!(self.vis, Visibility::Inherited) {
            self.vis.span()
        } else {
            self.static_keyword.span()
        };
        start.join(self.semi_punct.span())
    }
}

impl ToTokens for ItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.static_keyword.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.ty.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.expr.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}

impl ItemStatic {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
