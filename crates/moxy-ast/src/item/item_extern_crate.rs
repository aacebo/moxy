use moxy_token::keyword::{As, Crate, Extern};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Ident, Visibility};

/// An `extern crate` item (`extern crate foo;` or `extern crate foo as bar;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemExternCrate {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub extern_keyword: Extern,
    pub crate_keyword: Crate,
    pub ident: Ident,
    pub as_keyword: Option<As>,
    pub rename: Option<Ident>,
    pub semi_punct: Semi,
}

impl Parse for ItemExternCrate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let extern_keyword = stream.parse::<Extern>()?;
        let crate_keyword = stream.parse::<Crate>()?;
        let ident = stream.parse::<Ident>()?;

        let (as_keyword, rename) = if stream.peek::<As>() {
            let as_keyword = stream.parse::<As>()?;
            let rename = stream.parse::<Ident>()?;
            (Some(as_keyword), Some(rename))
        } else {
            (None, None)
        };

        let semi_punct = stream.parse::<Semi>()?;
        Ok(ItemExternCrate {
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
