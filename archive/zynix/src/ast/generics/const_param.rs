use crate::ast::{Attribute, Expr, Ident, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Const;
use crate::token::punct::{Colon, Eq};
use crate::{Parse, Span, TokenStream};

#[doc = "A const generic parameter (`const N: usize = 0`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstParam {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}

impl Parse for ConstParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let _ = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let _ = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;

        let default = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Expr>()?)
        } else {
            None
        };

        Ok(Self {
            span: Span::default(),
            attrs,
            ident,
            ty,
            default,
        })
    }
}

impl ToTokens for ConstParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Const::default().to_tokens(t);
        self.ident.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
        if let Some(d) = &self.default {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }
    }
}
