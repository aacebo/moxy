use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Generics, Ident, Type, Visibility};

/// A constant item (`const NAME: Type = expr;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemConst {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub const_keyword: Token![const],
    pub ident: Ident,
    pub generics: Generics,
    pub colon_punct: Token![:],
    pub ty: Type,
    pub eq_punct: Token![=],
    pub expr: Expr,
    pub semi_punct: Token![;],
}

impl Parse for ItemConst {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let const_keyword = parser.parse::<Token![const]>()?;
        let ident = parser.parse::<Ident>()?;
        let generics = parser.parse::<Generics>()?;
        let colon_punct = parser.parse::<Token![:]>()?;
        let ty = parser.parse::<Type>()?;
        let eq_punct = parser.parse::<Token![=]>()?;
        let expr = parser.parse::<Expr>()?;
        let semi_punct = parser.parse::<Token![;]>()?;

        Ok(Self {
            attrs,
            vis,
            const_keyword,
            ident,
            generics,
            colon_punct,
            ty,
            eq_punct,
            expr,
            semi_punct,
        })
    }
}

impl Spanner for ItemConst {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi_punct.span())
    }
}

impl ToTokens for ItemConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.ty.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.expr.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}

impl ItemConst {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
