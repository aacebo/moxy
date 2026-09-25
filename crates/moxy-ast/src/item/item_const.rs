use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Generics, Type, Visibility};

/// A constant item (`const NAME: Type = expr;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemConst {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub const_keyword: Token![const],
    pub ident: Ident,
    pub generics: Generics,
    pub colon_punct: Token![:],
    pub ty: Type,
    pub eq_punct: Option<Token![=]>,
    pub expr: Option<Expr>,
    pub semi_punct: Token![;],
}

impl Parse for ItemConst {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![const]>::peek(cursor) && Ident::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let const_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let generics = <_ as Parse>::parse(parser)?;
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

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![const]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;

        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

        <Token![;]>::skip(cursor)
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
