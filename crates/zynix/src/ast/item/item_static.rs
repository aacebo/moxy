use crate::ast::{Attribute, Expr, Ident, Mutability, Type, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Static;
use crate::token::punct::{Colon, Eq, Semi};
use crate::{Parse, Span, TokenStream};

#[doc = "A static item (`static [mut] NAME: Type = expr;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStatic {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Ident,
    pub ty: Type,
    pub expr: Expr,
}

impl Parse for ItemStatic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let _ = stream.parse::<Static>()?;
        let mutability = stream.parse::<Mutability>()?;
        let ident = stream.parse::<Ident>()?;
        let _ = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;
        let _ = stream.parse::<Eq>()?;
        let expr = stream.parse::<Expr>()?;
        let _ = stream.parse::<Semi>();
        Ok(ItemStatic {
            span: Span::default(),
            attrs,
            vis,
            mutability,
            ident,
            ty,
            expr,
        })
    }
}

impl ToTokens for ItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        Static::default().to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
        Eq::default().to_tokens(t);
        self.expr.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
