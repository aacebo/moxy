use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A foreign opaque type declaration inside an `extern` block (`type Name;`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
        cursor.peek::<Token![type]>() || (cursor.peek::<Token![pub]>() && cursor.offset(1).peek::<Token![type]>())
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            vis: parser.parse()?,
            type_keyword: parser.parse()?,
            ident: parser.parse()?,
            generics: parser.parse()?,
            semi: parser.parse()?,
        })
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
