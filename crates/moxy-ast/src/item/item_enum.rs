use moxy_token::keyword::Enum;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Eq};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Delimited, Expr, Fields, Generics, Ident, Punctuated, Visibility};

#[doc = "An enum item (`enum Name<T> { Variant, ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemEnum {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub enum_keyword: Enum,
    pub ident: Ident,
    pub generics: Generics,
    pub variants: Delimited<Punctuated<Variant, Comma>>,
}

impl Parse for ItemEnum {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let enum_keyword = stream.parse::<Enum>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;

        if stream.peek::<moxy_token::keyword::Where>().is_some() {
            generics.where_clause = Some(stream.parse()?);
        }

        let variants = Delimited::parse_brace_with(stream, Punctuated::parse_terminated)?;
        Ok(ItemEnum {
            span: Span::default(),
            attrs,
            vis,
            enum_keyword,
            ident,
            generics,
            variants,
        })
    }
}

impl ToTokens for ItemEnum {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.enum_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.variants.to_tokens(t);
    }
}

#[doc = "An enum variant (`Name`, `Name(T)`, `Name { x: T }`, `Name = 1`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variant {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub fields: Fields,
    pub eq_punct: Option<Eq>,
    pub discriminant: Option<Expr>,
}

impl Parse for Variant {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let ident = stream.parse::<Ident>()?;
        let fields = stream.parse::<Fields>()?;

        let (eq_punct, discriminant) = if stream.peek::<Eq>().is_some() {
            let eq_punct = stream.parse::<Eq>()?;
            let discriminant = stream.parse::<Expr>()?;
            (Some(eq_punct), Some(discriminant))
        } else {
            (None, None)
        };

        Ok(Self {
            span: Span::default(),
            attrs,
            ident,
            fields,
            eq_punct,
            discriminant,
        })
    }
}

impl ToTokens for Variant {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.ident.to_tokens(t);
        self.fields.to_tokens(t);

        if let (Some(eq_punct), Some(d)) = (&self.eq_punct, &self.discriminant) {
            eq_punct.to_tokens(t);
            d.to_tokens(t);
        }
    }
}
