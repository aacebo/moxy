use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

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
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Ident>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let ident = parser.parse()?;
        let (colon_punct, bounds) = if parser.peek::<Token![:]>() {
            let colon_punct = parser.parse()?;
            let bounds = parser.parse()?;
            (Some(colon_punct), bounds)
        } else {
            (None, Punctuated::new())
        };

        let (eq_punct, default) = if parser.peek::<Token![=]>() {
            let eq_punct = parser.parse()?;
            let default = parser.parse()?;
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
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.default.to_tokens(t);
    }
}

impl TypeParam {
    pub fn into_generic_param(self) -> super::GenericParam {
        super::GenericParam::from(self)
    }
}
