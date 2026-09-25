use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Safety, Type, Visibility};

/// A static item (`static [mut] NAME: Type = expr;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStatic {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub safety: Option<Safety>,
    pub static_keyword: Token![static],
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub colon_punct: Token![:],
    pub ty: Type,
    pub eq_punct: Option<Token![=]>,
    pub expr: Option<Expr>,
    pub semi_punct: Token![;],
}

impl Parse for ItemStatic {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Safety::skip(cursor).unwrap_or(cursor);
        <Token![static]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let safety = <_ as Parse>::parse(parser)?;
        let static_keyword = <_ as Parse>::parse(parser)?;
        let mutability = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let colon_punct = <_ as Parse>::parse(parser)?;
        let ty = <_ as Parse>::parse(parser)?;
        let eq_punct: Option<Token![=]> = <_ as Parse>::parse(parser)?;
        let expr = if eq_punct.is_some() {
            Some(<_ as Parse>::parse(parser)?)
        } else {
            None
        };

        let semi_punct = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            vis,
            safety,
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
        cursor = Option::<Safety>::skip(cursor)?;
        cursor = <Token![static]>::skip(cursor)?;
        cursor = Option::<Token![mut]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;

        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

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
        self.safety.to_tokens(t);
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
