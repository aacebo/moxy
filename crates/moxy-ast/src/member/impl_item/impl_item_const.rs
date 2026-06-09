use moxy_token::keyword::Const;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq, Semi};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Defaultness, Expr, Generics, Ident, Type, Visibility};

/// A constant item inside an `impl` block (`const NAME: Type = expr;`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemConst {
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

        if stream.curr().and_then(|t| t.text()) != Some("const") {
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

impl Spanner for ImplItemConst {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if !matches!(self.vis, Visibility::Inherited) {
            self.vis.span()
        } else {
            self.const_keyword.span()
        };
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.expr.span());
        start.join(end)
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

impl ImplItemConst {
    pub fn into_impl_item(self) -> super::ImplItem {
        super::ImplItem::from(self)
    }
}
