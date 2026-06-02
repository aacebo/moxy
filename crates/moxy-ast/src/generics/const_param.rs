use moxy_token::keyword::Const;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Expr, Ident, Type};

#[doc = "A const generic parameter (`const N: usize = 0`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstParam {
    pub span: Span,
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
        let attrs = stream.parse_vec::<Attribute>()?;
        let const_keyword = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let colon_punct = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;

        let (default_eq_punct, default) = if stream.peek::<Eq>().is_some() {
            let eq_punct = stream.parse::<Eq>()?;
            let expr = stream.parse::<Expr>()?;
            (Some(eq_punct), Some(expr))
        } else {
            (None, None)
        };

        Ok(Self {
            span: Span::default(),
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
