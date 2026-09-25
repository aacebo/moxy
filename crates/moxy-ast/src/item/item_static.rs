use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Ident, Type, Visibility};

/// A static item (`static [mut] NAME: Type = expr;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStatic {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub static_keyword: Token![static],
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub colon_punct: Token![:],
    pub ty: Type,
    pub eq_punct: Token![=],
    pub expr: Expr,
    pub semi_punct: Token![;],
}

impl Parse for ItemStatic {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![static]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let vis = parser.parse()?;
        let static_keyword = parser.parse()?;
        let mutability = parser.parse()?;
        let ident = parser.parse()?;
        let colon_punct = parser.parse()?;
        let ty = parser.parse()?;
        let eq_punct = parser.parse()?;
        let expr = parser.parse()?;
        let semi_punct = parser.parse()?;

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

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![static]>::skip(cursor)?;
        cursor = Option::<Token![mut]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;
        cursor = <Token![=]>::skip(cursor)?;
        cursor = Expr::skip(cursor)?;
        <Token![;]>::skip(cursor)
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
