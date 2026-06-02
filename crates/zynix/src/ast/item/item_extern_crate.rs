use crate::ast::{Attribute, Ident, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::{As, Crate, Extern};
use crate::token::punct::Semi;
use crate::{Parse, Span, TokenStream};

#[doc = "An `extern crate` item (`extern crate foo;` or `extern crate foo as bar;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemExternCrate {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub rename: Option<Ident>,
}

impl Parse for ItemExternCrate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let _ = stream.parse::<Extern>()?;
        let _ = stream.parse::<Crate>()?;
        let ident = stream.parse::<Ident>()?;

        let rename = if stream.peek::<As>().is_some() {
            let _ = stream.parse::<As>()?;
            Some(stream.parse::<Ident>()?)
        } else {
            None
        };

        let _ = stream.parse::<Semi>();
        Ok(ItemExternCrate {
            span: Span::default(),
            attrs,
            vis,
            ident,
            rename,
        })
    }
}

impl ToTokens for ItemExternCrate {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        Extern::default().to_tokens(t);
        Crate::default().to_tokens(t);
        self.ident.to_tokens(t);

        if let Some(r) = &self.rename {
            As::default().to_tokens(t);
            r.to_tokens(t);
        }

        Semi::default().to_tokens(t);
    }
}
