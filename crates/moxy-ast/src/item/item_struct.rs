use moxy_token::keyword::Struct;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Semi;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Fields, Generics, Ident, Visibility};

/// A struct item (`struct Name<T> { ... }` or `struct Name(T);`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStruct {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub struct_keyword: Struct,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Fields,
    pub semi_punct: Semi,
}

impl Parse for ItemStruct {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let _ = stream.parse::<Struct>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let fields = stream.parse::<Fields>()?;
        let _ = stream.parse::<Semi>();

        Ok(Self {
            attrs,
            vis,
            struct_keyword: Struct::default(),
            ident,
            generics,
            fields,
            semi_punct: Semi::default(),
        })
    }
}

impl Spanner for ItemStruct {
    fn span(&self) -> Span {
        self.attrs.span().join(self.fields.span())
    }
}

impl ToTokens for ItemStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.struct_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.fields.to_tokens(t);

        if !matches!(self.fields, Fields::Named(_)) {
            self.semi_punct.to_tokens(t);
        }
    }
}

impl ItemStruct {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
