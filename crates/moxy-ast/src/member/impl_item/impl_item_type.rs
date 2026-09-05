use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Defaultness, Generics, Ident, Type, Visibility};

/// An associated type definition inside an `impl` block (`type Name = Type;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemType {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub eq: Token![=],
    pub ty: Type,
    pub semi: Option<Token![;]>,
}

impl Parse for ImplItemType {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let defaultness = parser.parse::<Defaultness>()?;
        let type_keyword = parser.parse::<Token![type]>()?;
        let ident = parser.parse::<Ident>()?;
        let generics = parser.parse::<Generics>()?;
        let eq = parser.parse::<Token![=]>()?;
        let ty = parser.parse::<Type>()?;
        let semi = parser.parse_if::<Token![;]>();

        Ok(Self {
            attrs,
            vis,
            defaultness,
            type_keyword,
            ident,
            generics,
            eq,
            ty,
            semi,
        })
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

impl ImplItemType {
    pub fn into_impl_item(self) -> super::ImplItem {
        super::ImplItem::from(self)
    }
}
