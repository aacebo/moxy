use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

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
    pub colon: Option<Token![:]>,
    pub bounds: List<TypeBound, Token![+]>,
    /// The where clause before `=`, or before `;` without a type.
    pub where_clause: Option<WhereClause>,
    pub eq_punct: Option<Token![=]>,
    pub ty: Option<Type>,
    /// A where clause after the type, where Rust prefers it.
    pub where_clause_after: Option<WhereClause>,
    pub semi_punct: Token![;],
}

impl Attributed for ItemTypeAlias {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
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
        let colon: Option<Token![:]> = <_ as Parse>::parse(parser)?;
        let bounds = if colon.is_some() {
            List::parse_while_present(parser)?
        } else {
            List::new()
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

        let where_clause_after = if ty.is_some() { <_ as Parse>::parse(parser)? } else { None };

        let semi_punct = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            vis,
            type_keyword,
            ident,
            generics,
            colon,
            bounds,
            where_clause,
            eq_punct,
            ty,
            where_clause_after,
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
            cursor = List::<TypeBound, Token![+]>::skip_while_present(cursor)?;
        }

        cursor = Option::<WhereClause>::skip(cursor)?;

        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
            cursor = Option::<WhereClause>::skip(cursor)?;
        }

        <Token![;]>::skip(cursor)
    }
}

impl Spanner for ItemTypeAlias {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ItemTypeAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);

        self.colon.to_tokens(t);
        self.bounds.to_tokens(t);

        self.where_clause.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.ty.to_tokens(t);
        self.where_clause_after.to_tokens(t);

        self.semi_punct.to_tokens(t);
    }
}

impl ItemTypeAlias {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
