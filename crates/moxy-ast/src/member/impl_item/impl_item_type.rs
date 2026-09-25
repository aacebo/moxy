use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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
    pub eq: Token![=],
    pub ty: Type,
    pub semi: Option<Token![;]>,
}

impl Parse for ImplItemType {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![default]>::skip(cursor).unwrap_or(cursor);
        <Token![type]>::peek(cursor) && Ident::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            vis: <_ as Parse>::parse(parser)?,
            defaultness: <_ as Parse>::parse(parser)?,
            type_keyword: <_ as Parse>::parse(parser)?,
            ident: <_ as Parse>::parse(parser)?,
            generics: <_ as Parse>::parse(parser)?,
            eq: <_ as Parse>::parse(parser)?,
            ty: <_ as Parse>::parse(parser)?,
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
        cursor = <Token![=]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;
        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for ImplItemType {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.ty.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ImplItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.eq.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
