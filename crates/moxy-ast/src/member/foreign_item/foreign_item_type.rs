use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

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
    pub semi: Option<Token![;]>,
}

impl Parse for ForeignItemType {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![type]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            vis: <_ as Parse>::parse(parser)?,
            type_keyword: <_ as Parse>::parse(parser)?,
            ident: <_ as Parse>::parse(parser)?,
            generics: <_ as Parse>::parse(parser)?,
            semi: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![type]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for ForeignItemType {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.ident.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ForeignItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.semi.to_tokens(t);
    }
}

impl ForeignItemType {
    pub fn into_foreign_item(self) -> super::ForeignItem {
        super::ForeignItem::from(self)
    }
}
