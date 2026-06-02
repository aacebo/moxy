use crate::ast::{Attribute, Fields, Generics, Ident, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Struct;
use crate::token::punct::Semi;
use crate::{Parse, Span, TokenStream};

#[doc = "A struct item (`struct Name<T> { ... }` or `struct Name(T);`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Fields,
}

impl Parse for ItemStruct {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let _ = stream.parse::<Struct>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;

        if stream.peek::<crate::token::keyword::Where>().is_some() {
            generics.where_clause = Some(stream.parse()?);
        }

        let fields = stream.parse::<Fields>()?;
        let _ = stream.parse::<Semi>();
        Ok(ItemStruct {
            span: Span::default(),
            attrs,
            vis,
            ident,
            generics,
            fields,
        })
    }
}

impl ToTokens for ItemStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        Struct::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.fields.to_tokens(t);

        if !matches!(self.fields, Fields::Named(_)) {
            Semi::default().to_tokens(t);
        }
    }
}
