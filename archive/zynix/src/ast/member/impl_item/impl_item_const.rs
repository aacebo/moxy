use super::ImplItem;
use crate::ast::{Attribute, Defaultness, Expr, Generics, Ident, Type, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Const;
use crate::token::punct::{Colon, Eq, Semi};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A constant item inside an `impl` block (`const NAME: Type = expr;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemConst {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
    pub expr: Expr,
}

impl Parse for ImplItemConst {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<Defaultness>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("const") {
            return Err(LexError::new(at).message("expected impl const").into());
        }

        let _ = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let _ = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;
        let _ = stream.parse::<Eq>()?;
        let expr = stream.parse::<Expr>()?;
        let _ = stream.parse::<Semi>();
        Ok(ImplItemConst {
            span: Span::default(),
            attrs,
            vis,
            defaultness,
            ident,
            generics,
            ty,
            expr,
        })
    }
}

impl ToTokens for ImplItemConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        Const::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
        Eq::default().to_tokens(t);
        self.expr.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
