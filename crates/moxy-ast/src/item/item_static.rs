use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Ident, Mutability, Type, Visibility};

/// A static item (`static [mut] NAME: Type = expr;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStatic {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub static_keyword: Token![static],
    pub mutability: Mutability,
    pub ident: Ident,
    pub colon_punct: Token![:],
    pub ty: Type,
    pub eq_punct: Token![=],
    pub expr: Expr,
    pub semi_punct: Token![;],
}

impl Parse for ItemStatic {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let static_keyword = parser.parse::<Token![static]>()?;
        let mutability = parser.parse::<Mutability>()?;
        let ident = parser.parse::<Ident>()?;
        let colon_punct = parser.parse::<Token![:]>()?;
        let ty = parser.parse::<Type>()?;
        let eq_punct = parser.parse::<Token![=]>()?;
        let expr = parser.parse::<Expr>()?;
        let semi_punct = parser.parse::<Token![;]>()?;

        Ok(Self {
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
        self.attrs.span().join(self.semi_punct.span())
    }
}

impl ToTokens for ItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
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
