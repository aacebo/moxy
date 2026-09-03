use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Ident, Visibility};

/// An `extern crate` item (`extern crate foo;` or `extern crate foo as bar;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemExternCrate {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub extern_keyword: Token![extern],
    pub crate_keyword: Token![crate],
    pub ident: Ident,
    pub as_keyword: Option<Token![as]>,
    pub rename: Option<Ident>,
    pub semi_punct: Token![;],
}

impl Parse for ItemExternCrate {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let extern_keyword = parser.parse::<Token![extern]>()?;
        let crate_keyword = parser.parse::<Token![crate]>()?;
        let ident = parser.parse::<Ident>()?;
        let (as_keyword, rename) = if parser.peek::<Token![as]>() {
            let as_keyword = parser.parse::<Token![as]>()?;
            let rename = parser.parse::<Ident>()?;
            (Some(as_keyword), Some(rename))
        } else {
            (None, None)
        };

        let semi_punct = parser.parse::<Token![;]>()?;

        Ok(Self {
            attrs,
            vis,
            extern_keyword,
            crate_keyword,
            ident,
            as_keyword,
            rename,
            semi_punct,
        })
    }
}

impl Spanner for ItemExternCrate {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi_punct.span())
    }
}

impl ToTokens for ItemExternCrate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.extern_keyword.to_tokens(t);
        self.crate_keyword.to_tokens(t);
        self.ident.to_tokens(t);

        if let (Some(as_keyword), Some(r)) = (&self.as_keyword, &self.rename) {
            as_keyword.to_tokens(t);
            r.to_tokens(t);
        }

        self.semi_punct.to_tokens(t);
    }
}

impl ItemExternCrate {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
