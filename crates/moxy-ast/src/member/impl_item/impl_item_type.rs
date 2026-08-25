use moxy_token::keyword::Type as KwType;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Eq, Semi};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Defaultness, Generics, Ident, Type, Visibility};

/// An associated type definition inside an `impl` block (`type Name = Type;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemType {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub type_keyword: KwType,
    pub ident: Ident,
    pub generics: Generics,
    pub eq: Eq,
    pub ty: Type,
    pub semi: Option<Semi>,
}

impl Parse for ImplItemType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<Defaultness>()?;

        if stream.curr().and_then(|t| t.text()) != Some("type") {
            return Err(LexError::new(at).message("expected impl type").into());
        }

        let type_keyword = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let eq = stream.parse::<Eq>()?;
        let ty = stream.parse::<Type>()?;
        let semi = stream.parse_if::<Semi>();

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
