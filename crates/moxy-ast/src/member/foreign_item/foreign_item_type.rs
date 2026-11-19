use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A foreign opaque type declaration inside an `extern` block (`type Name;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemType {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    /// Bounds and a default, which rustc's parser accepts and the compiler
    /// rejects later, as in `type T: A;`.
    pub colon: Option<Token![:]>,
    pub bounds: List<TypeBound, Token![+]>,
    pub default: Option<(Token![=], Type)>,
    /// A where clause after the default; one before it is in `generics`.
    pub where_clause_after: Option<WhereClause>,
    pub semi: Option<Token![;]>,
}

impl Attributed for ForeignItemType {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ForeignItemType {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![type]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let type_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let mut generics = <_ as Parse>::parse(parser)?;
        let (colon, bounds, default, where_clause_after) = super::super::parse_type_rest(parser, &mut generics)?;

        Ok(Self {
            attrs,
            vis,
            type_keyword,
            ident,
            generics,
            colon,
            bounds,
            default,
            where_clause_after,
            semi: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![type]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        cursor = super::super::skip_type_rest(cursor)?;
        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for ForeignItemType {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ForeignItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        super::super::type_rest_to_tokens(
            &self.generics,
            &self.colon,
            &self.bounds,
            &self.default,
            &self.where_clause_after,
            t,
        );
        self.semi.to_tokens(t);
    }
}

impl ForeignItemType {
    pub fn into_foreign_item(self) -> super::ForeignItem {
        super::ForeignItem::from(self)
    }
}
