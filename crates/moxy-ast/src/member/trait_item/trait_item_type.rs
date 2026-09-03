use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Generics, Ident, Punctuated, Type, TypeBound};

/// An associated type inside a trait definition (`type Name: Bound = Default;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemType {
    pub attrs: Attributes,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Option<Token![:]>,
    pub bounds: Punctuated<TypeBound, Token![+]>,
    pub default: Option<(Token![=], Type)>,
    pub semi: Token![;],
}

impl Parse for TraitItemType {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let type_keyword = parser.parse::<Token![type]>()?;
        let ident = parser.parse::<Ident>()?;
        let generics = parser.parse::<Generics>()?;
        let (colon, bounds) = if parser.peek::<Token![:]>() {
            (Some(parser.parse::<Token![:]>()?), TypeBound::parse_bounds(parser)?)
        } else {
            (None, Punctuated::new())
        };

        let default = if parser.peek::<Token![=]>() {
            let eq = parser.parse::<Token![=]>()?;
            Some((eq, parser.parse::<Type>()?))
        } else {
            None
        };

        let semi = parser.parse()?;

        Ok(Self {
            attrs,
            type_keyword,
            ident,
            generics,
            colon,
            bounds,
            default,
            semi,
        })
    }
}

impl Spanner for TraitItemType {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi.span())
    }
}

impl ToTokens for TraitItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon.to_tokens(t);
        self.bounds.to_tokens(t);

        if let Some((eq, d)) = &self.default {
            eq.to_tokens(t);
            d.to_tokens(t);
        }

        self.semi.to_tokens(t);
    }
}

impl TraitItemType {
    pub fn into_trait_item(self) -> super::TraitItem {
        super::TraitItem::from(self)
    }
}
