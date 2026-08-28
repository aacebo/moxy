use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Ident, Type};

/// A const generic parameter (`const N: usize = 0`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstParam {
    pub attrs: Attributes,
    pub const_keyword: Token![const],
    pub ident: Ident,
    pub colon_punct: Token![:],
    pub ty: Type,
    pub default_eq_punct: Option<Token![=]>,
    pub default: Option<Expr>,
}

impl Parse for ConstParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let const_keyword = stream.parse::<Token![const]>()?;
        let ident = stream.parse::<Ident>()?;
        let colon_punct = stream.parse::<Token![:]>()?;
        let ty = stream.parse::<Type>()?;
        let (default_eq_punct, default) = if stream.peek::<Token![=]>() {
            let eq_punct = stream.parse::<Token![=]>()?;
            let expr = stream.parse::<Expr>()?;
            (Some(eq_punct), Some(expr))
        } else {
            (None, None)
        };

        Ok(Self {
            attrs,
            const_keyword,
            ident,
            colon_punct,
            ty,
            default_eq_punct,
            default,
        })
    }
}

impl Spanner for ConstParam {
    fn span(&self) -> Span {
        let end = if let Some(d) = &self.default {
            d.span()
        } else {
            self.ty.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for ConstParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.ty.to_tokens(t);

        if let Some(eq_punct) = &self.default_eq_punct {
            eq_punct.to_tokens(t);
        }
        if let Some(d) = &self.default {
            d.to_tokens(t);
        }
    }
}

impl ConstParam {
    pub fn into_generic_param(self) -> super::GenericParam {
        super::GenericParam::from(self)
    }
}
