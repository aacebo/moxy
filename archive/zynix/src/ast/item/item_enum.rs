use crate::ast::{Attribute, Expr, Fields, Generics, Ident, Punctuated, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Enum;
use crate::token::punct::{Comma, Eq};
use crate::token::{Delim, Group, ToTokens, TokenStream as TS, TokenTree};
use crate::{Parse, Span, TokenStream};

#[doc = "An enum item (`enum Name<T> { Variant, ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemEnum {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub variants: Punctuated<Variant, Comma>,
}

impl Parse for ItemEnum {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let _ = stream.parse::<Enum>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;

        if stream.peek::<crate::token::keyword::Where>().is_some() {
            generics.where_clause = Some(stream.parse()?);
        }

        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let variants = Punctuated::parse_terminated(&mut inner)?;
        Ok(ItemEnum {
            span: Span::default(),
            attrs,
            vis,
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
        Enum::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        let mut inner = TS::new();
        self.variants.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
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
    pub discriminant: Option<Expr>,
}

impl Parse for Variant {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let ident = stream.parse::<Ident>()?;
        let fields = stream.parse::<Fields>()?;

        let discriminant = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Expr>()?)
        } else {
            None
        };

        Ok(Self {
            span: Span::default(),
            attrs,
            ident,
            fields,
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

        if let Some(d) = &self.discriminant {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }
    }
}
