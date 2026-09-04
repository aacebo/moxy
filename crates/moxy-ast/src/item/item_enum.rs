use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Delimited, Expr, Fields, Generics, Ident, Punctuated, Visibility};

/// An enum item (`enum Name<T> { Variant, ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemEnum {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub enum_keyword: Token![enum],
    pub ident: Ident,
    pub generics: Generics,
    pub variants: Delimited<Punctuated<Variant, Token![,]>>,
}

impl Parse for ItemEnum {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let enum_keyword = parser.parse::<Token![enum]>()?;
        let ident = parser.parse::<Ident>()?;
        let generics = parser.parse::<Generics>()?;
        let variants = Delimited::parse_brace_with(parser, Punctuated::parse_terminated)?;

        Ok(Self {
            attrs,
            vis,
            enum_keyword,
            ident,
            generics,
            variants,
        })
    }
}

impl Spanner for ItemEnum {
    fn span(&self) -> Span {
        self.attrs.span().join(self.variants.span())
    }
}

impl Spanner for Variant {
    fn span(&self) -> Span {
        let end = if let Some(d) = &self.discriminant {
            d.span()
        } else {
            self.fields.span()
        };
        self.attrs.span().join(end)
    }
}

impl ToTokens for ItemEnum {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.enum_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.variants.to_tokens(t);
    }
}

/// An enum variant (`Name`, `Name(T)`, `Name { x: T }`, `Name = 1`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variant {
    pub attrs: Attributes,
    pub ident: Ident,
    pub fields: Fields,
    pub eq_punct: Option<Token![=]>,
    pub discriminant: Option<Expr>,
}

impl Parse for Variant {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let ident = parser.parse::<Ident>()?;
        let fields = parser.parse::<Fields>()?;
        let (eq_punct, discriminant) = if parser.peek::<Token![=]>() {
            let eq_punct = parser.parse::<Token![=]>()?;
            let discriminant = parser.parse::<Expr>()?;
            (Some(eq_punct), Some(discriminant))
        } else {
            (None, None)
        };

        Ok(Self {
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
        self.attrs.to_tokens(t);
        self.ident.to_tokens(t);
        self.fields.to_tokens(t);

        if let (Some(eq_punct), Some(d)) = (&self.eq_punct, &self.discriminant) {
            eq_punct.to_tokens(t);
            d.to_tokens(t);
        }
    }
}

impl ItemEnum {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
