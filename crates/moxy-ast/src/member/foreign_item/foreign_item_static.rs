use crate::{Parse, ParseError, Parser};
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Ident, Mutability, Type, Visibility};

/// A foreign static declaration inside an `extern` block (`static NAME: Type;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemStatic {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub static_keyword: Token![static],
    pub mutability: Mutability,
    pub ident: Ident,
    pub colon: Token![:],
    pub ty: Type,
    pub semi: Option<Token![;]>,
}

impl Parse for ForeignItemStatic {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at = parser.span();
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;

        if parser.curr().and_then(|t| t.text()) != Some("static") {
            return Err(LexError::new(at).message("expected foreign static").into());
        }

        let static_keyword = parser.parse::<Token![static]>()?;
        let mutability = parser.parse::<Mutability>()?;
        let ident = parser.parse::<Ident>()?;
        let colon = parser.parse::<Token![:]>()?;
        let ty = parser.parse::<Type>()?;
        let semi = parser.parse_if::<Token![;]>();

        Ok(Self {
            attrs,
            vis,
            static_keyword,
            mutability,
            ident,
            colon,
            ty,
            semi,
        })
    }
}

impl Spanner for ForeignItemStatic {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.ty.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ForeignItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.static_keyword.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi.to_tokens(t);
    }
}

impl ForeignItemStatic {
    pub fn into_foreign_item(self) -> super::ForeignItem {
        super::ForeignItem::from(self)
    }
}
