use moxy_token::keyword::Type as KwType;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use super::ForeignItem;
use crate::{Attribute, Generics, Ident, Visibility};

/// A foreign opaque type declaration inside an `extern` block (`type Name;`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemType {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub type_keyword: KwType,
    pub ident: Ident,
    pub generics: Generics,
    pub semi: Option<Semi>,
}

impl Parse for ForeignItemType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("type") {
            return Err(LexError::new(at).message("expected foreign type").into());
        }

        let type_keyword = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let semi = stream.parse_if::<Semi>();
        Ok(ForeignItemType {
            attrs,
            vis,
            type_keyword,
            ident,
            generics,
            semi,
        })
    }
}

impl Spanner for ForeignItemType {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if !matches!(self.vis, Visibility::Inherited) {
            self.vis.span()
        } else {
            self.type_keyword.span()
        };
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.ident.span);
        start.join(end)
    }
}

impl ToTokens for ForeignItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
