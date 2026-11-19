use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An associated type inside a trait definition (`type Name: Bound = Default;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemType {
    pub attrs: Attributes,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Option<Token![:]>,
    pub bounds: List<TypeBound, Token![+]>,
    pub default: Option<(Token![=], Type)>,
    pub semi: Token![;],
}

impl Attributed for TraitItemType {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for TraitItemType {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| <Token![type]>::peek(cursor) && Ident::peek(cursor.offset(1)))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let type_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let mut generics: Generics = <_ as Parse>::parse(parser)?;
        let (colon, bounds) = if <Token![:]>::peek(parser.cursor()) {
            let colon = <_ as Parse>::parse(parser)?;

            if TypeBound::peek(parser.cursor()) {
                (Some(colon), List::parse_nonempty(parser)?)
            } else {
                (Some(colon), List::new())
            }
        } else {
            (None, List::new())
        };

        // A where clause follows the bounds; without bounds, `Generics` has
        // already read it
        if generics.where_clause.is_none() {
            generics.where_clause = <_ as Parse>::parse(parser)?;
        }

        let default = if <Token![=]>::peek(parser.cursor()) {
            let eq = <_ as Parse>::parse(parser)?;
            Some((eq, <_ as Parse>::parse(parser)?))
        } else {
            None
        };

        // The where clause can also follow the default, where Rust prefers it
        if default.is_some() && generics.where_clause.is_none() {
            generics.where_clause = <_ as Parse>::parse(parser)?;
        }

        let semi = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            type_keyword,
            ident,
            generics,
            colon,
            bounds,
            default,
            semi,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = <Token![type]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;

        if <Token![:]>::peek(cursor) {
            cursor = <Token![:]>::skip(cursor)?;

            if TypeBound::peek(cursor) {
                cursor = TypeBound::skip(cursor)?;

                while <Token![+]>::peek(cursor) {
                    cursor = <Token![+]>::skip(cursor)?;
                    cursor = TypeBound::skip(cursor)?;
                }
            }
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

impl Spanner for TraitItemType {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi.span())
    }
}

impl ToTokens for TraitItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.lt.to_tokens(t);
        self.generics.params.to_tokens(t);
        self.generics.gt.to_tokens(t);
        self.colon.to_tokens(t);
        self.bounds.to_tokens(t);

        // With a default, the where clause goes after it, where Rust prefers it
        if let Some((eq, d)) = &self.default {
            eq.to_tokens(t);
            d.to_tokens(t);
        }

        self.generics.where_clause.to_tokens(t);

        self.semi.to_tokens(t);
    }
}
