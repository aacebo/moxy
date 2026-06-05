use moxy_token::keyword::Const;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq, Semi};
use moxy_token::{LexError, Parse, Span, ToTokens, TokenStream};

use super::ImplItem;
use crate::{Attribute, Defaultness, Expr, Generics, Ident, Type, Visibility};

#[doc = "A constant item inside an `impl` block (`const NAME: Type = expr;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemConst {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub const_keyword: Const,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Colon,
    pub ty: Type,
    pub eq: Eq,
    pub expr: Expr,
    pub semi: Option<Semi>,
}

impl Parse for ImplItemConst {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<Defaultness>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("const") {
            return Err(LexError::new(at).message("expected impl const").into());
        }

        let const_keyword = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let colon = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;
        let eq = stream.parse::<Eq>()?;
        let expr = stream.parse::<Expr>()?;
        let semi = stream.parse_if::<Semi>();
        Ok(ImplItemConst {
            span: Span::default(),
            attrs,
            vis,
            defaultness,
            const_keyword,
            ident,
            generics,
            colon,
            ty,
            eq,
            expr,
            semi,
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
        self.const_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
        self.eq.to_tokens(t);
        self.expr.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
