use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::TypeBound;
use crate::{Attributes, Ident, Punctuated, Type};

/// A type parameter (`T: Bound = Default`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParam {
    pub attrs: Attributes,
    pub ident: Ident,
    pub colon_punct: Option<Token![:]>,
    pub bounds: Punctuated<TypeBound, Token![+]>,
    pub eq_punct: Option<Token![=]>,
    pub default: Option<Type>,
}

impl Parse for TypeParam {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let ident = parser.parse::<Ident>()?;
        let (colon_punct, bounds) = if parser.peek::<Token![:]>() {
            let colon_punct = parser.parse::<Token![:]>()?;
            let bounds = TypeBound::parse_bounds(parser)?;
            (Some(colon_punct), bounds)
        } else {
            (None, Punctuated::new())
        };

        let (eq_punct, default) = if parser.peek::<Token![=]>() {
            let eq_punct = parser.parse::<Token![=]>()?;
            let default = parser.parse::<Type>()?;
            (Some(eq_punct), Some(default))
        } else {
            (None, None)
        };

        Ok(Self {
            attrs,
            ident,
            colon_punct,
            bounds,
            eq_punct,
            default,
        })
    }
}

impl Spanner for TypeParam {
    fn span(&self) -> Span {
        let end = if let Some(d) = &self.default {
            d.span()
        } else if let Some(b) = self.bounds.last() {
            b.span()
        } else {
            self.ident.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for TypeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.ident.to_tokens(t);

        if !self.bounds.is_empty() {
            if let Some(colon_punct) = &self.colon_punct {
                colon_punct.to_tokens(t);
            }
            self.bounds.to_tokens(t);
        }

        if let Some(d) = &self.default {
            if let Some(eq_punct) = &self.eq_punct {
                eq_punct.to_tokens(t);
            }
            d.to_tokens(t);
        }
    }
}

impl TypeParam {
    pub fn into_generic_param(self) -> super::GenericParam {
        super::GenericParam::from(self)
    }
}
