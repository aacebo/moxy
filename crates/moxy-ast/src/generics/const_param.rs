use moxy_token::keyword::Const;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Expr, Ident, Type};

/// A const generic parameter (`const N: usize = 0`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstParam {
    pub attrs: Vec<Attribute>,
    pub const_keyword: Const,
    pub ident: Ident,
    pub colon_punct: Colon,
    pub ty: Type,
    pub default_eq_punct: Option<Eq>,
    pub default: Option<Expr>,
}

impl Parse for ConstParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let const_keyword = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let colon_punct = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;

        let (default_eq_punct, default) = if stream.peek::<Eq>() {
            let eq_punct = stream.parse::<Eq>()?;
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
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.const_keyword.span()
        };
        let end = if let Some(d) = &self.default {
            d.span()
        } else {
            self.ty.span()
        };
        start.join(end)
    }
}

impl ToTokens for ConstParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
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
