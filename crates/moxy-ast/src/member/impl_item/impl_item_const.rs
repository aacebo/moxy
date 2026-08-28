use moxy_token::keyword::Const;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq, Semi};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Defaultness, Expr, Generics, Ident, Type, Visibility};

/// A constant item inside an `impl` block (`const NAME: Type = expr;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemConst {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub const_keyword: Const,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Colon,
    pub ty: Type,
    pub eq: Eq,
    pub expr: Expr,
    pub semi: Option<Semi>,
}

impl Parse for ImplItemConst {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<Defaultness>()?;
        let const_keyword = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let colon = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;
        let eq = stream.parse::<Eq>()?;
        let expr = stream.parse::<Expr>()?;
        let semi = stream.parse_if::<Semi>();

        Ok(Self {
            attrs,
            vis,
            defaultness,
            const_keyword,
            ident,
            generics,
            colon,
            ty,
            eq,
            expr,
            semi,
        })
    }
}

impl Spanner for ImplItemConst {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.expr.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ImplItemConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
        self.eq.to_tokens(t);
        self.expr.to_tokens(t);
        self.semi.to_tokens(t);
    }
}

impl ImplItemConst {
    pub fn into_impl_item(self) -> super::ImplItem {
        super::ImplItem::from(self)
    }
}
