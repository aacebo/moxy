use moxy_token::keyword::Union;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, FieldsNamed, Generics, Ident, Visibility};

/// A union item (`union Name<T> { field: Type, ... }`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUnion {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub union_keyword: Union,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: FieldsNamed,
}

impl Parse for ItemUnion {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let union_keyword = stream.parse::<Union>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let fields = stream.parse::<FieldsNamed>()?;
        Ok(ItemUnion {
            attrs,
            vis,
            union_keyword,
            ident,
            generics,
            fields,
        })
    }
}

impl Spanner for ItemUnion {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if !matches!(self.vis, Visibility::Inherited) {
            self.vis.span()
        } else {
            self.union_keyword.span()
        };
        start.join(self.fields.span())
    }
}

impl ToTokens for ItemUnion {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.union_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.fields.to_tokens(t);
    }
}
