use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Generics, Ident, Punctuated, Type, TypeBound, Visibility, WhereClause};

/// A type alias item (`type Name<T> = Type;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTypeAlias {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Punctuated<TypeBound, Token![+]>,
    pub where_clause: Option<WhereClause>,
    pub eq_punct: Option<Token![=]>,
    pub ty: Option<Type>,
    pub semi_punct: Token![;],
}

impl Parse for ItemTypeAlias {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![type]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let type_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let mut generics: Generics = <_ as Parse>::parse(parser)?;
        let bounds = if <Token![:]>::peek(parser.cursor()) {
            let _: Token![:] = <_ as Parse>::parse(parser)?;
            TypeBound::parse_bounds(parser)?
        } else {
            Punctuated::new()
        };

        let where_clause = generics
            .where_clause
            .take()
            .or_else(|| <_ as Parse>::parse(parser).ok().flatten());
        let eq_punct: Option<Token![=]> = <_ as Parse>::parse(parser)?;
        let ty = if eq_punct.is_some() {
            Some(<_ as Parse>::parse(parser)?)
        } else {
            None
        };

        let semi_punct = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            vis,
            type_keyword,
            ident,
            generics,
            bounds,
            where_clause,
            eq_punct,
            ty,
            semi_punct,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![type]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;

        if <Token![:]>::peek(cursor) {
            cursor = <Token![:]>::skip(cursor)?;
            cursor = TypeBound::skip(cursor)?;
            while <Token![+]>::peek(cursor) {
                cursor = <Token![+]>::skip(cursor)?;
                cursor = TypeBound::skip(cursor)?;
            }
        }

        cursor = Option::<WhereClause>::skip(cursor)?;

        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
        }

        <Token![;]>::skip(cursor)
    }
}

impl Spanner for ItemTypeAlias {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi_punct.span())
    }
}

impl ToTokens for ItemTypeAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);

        if !self.bounds.is_empty() {
            <Token![:]>::default().to_tokens(t);
            self.bounds.to_tokens(t);
        }

        self.where_clause.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}

impl ItemTypeAlias {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
