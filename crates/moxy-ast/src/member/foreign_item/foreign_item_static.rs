use moxy_token::keyword::Static;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Semi};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Ident, Mutability, Type, Visibility};

/// A foreign static declaration inside an `extern` block (`static NAME: Type;`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemStatic {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_keyword: Static,
    pub mutability: Mutability,
    pub ident: Ident,
    pub colon: Colon,
    pub ty: Type,
    pub semi: Option<Semi>,
}

impl Parse for ForeignItemStatic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;

        if stream.curr().and_then(|t| t.text()) != Some("static") {
            return Err(LexError::new(at).message("expected foreign static").into());
        }

        let static_keyword = stream.parse::<Static>()?;
        let mutability = stream.parse::<Mutability>()?;
        let ident = stream.parse::<Ident>()?;
        let colon = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;
        let semi = stream.parse_if::<Semi>();
        Ok(ForeignItemStatic {
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
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if !matches!(self.vis, Visibility::Inherited) {
            self.vis.span()
        } else {
            self.static_keyword.span()
        };
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.ty.span());
        start.join(end)
    }
}

impl ToTokens for ForeignItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
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
