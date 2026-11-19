use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// An associated type definition inside an `impl` block (`type Name = Type;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemType {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub defaultness: Option<Token![default]>,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    /// The `=` and type, or none before `;`, which rustc's parser accepts and
    /// the compiler rejects later.
    pub eq: Option<Token![=]>,
    pub ty: Option<Type>,
    /// A where clause after the type; one before `=` is in `generics`.
    pub where_clause_after: Option<WhereClause>,
    pub semi: Option<Token![;]>,
}

impl Attributed for ImplItemType {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ImplItemType {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![default]>::skip(cursor).unwrap_or(cursor);
        <Token![type]>::peek(cursor) && Ident::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let defaultness = <_ as Parse>::parse(parser)?;
        let type_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let generics: Generics = <_ as Parse>::parse(parser)?;
        let eq: Option<Token![=]> = <_ as Parse>::parse(parser)?;
        let ty = if eq.is_some() {
            Some(<_ as Parse>::parse(parser)?)
        } else {
            None
        };

        // A where clause can also follow the type, where Rust prefers it
        let where_clause_after = if ty.is_some() { <_ as Parse>::parse(parser)? } else { None };

        Ok(Self {
            attrs,
            vis,
            defaultness,
            type_keyword,
            ident,
            generics,
            eq,
            ty,
            where_clause_after,
            semi: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Option::<Token![default]>::skip(cursor)?;
        cursor = <Token![type]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
            cursor = Option::<WhereClause>::skip(cursor)?;
        }

        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for ImplItemType {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ImplItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.lt.to_tokens(t);
        self.generics.params.to_tokens(t);
        self.generics.gt.to_tokens(t);
        self.generics.where_clause.to_tokens(t);
        self.eq.to_tokens(t);
        self.ty.to_tokens(t);
        self.where_clause_after.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
