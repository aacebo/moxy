use super::TraitItem;
use crate::ast::{Attribute, Expr, Generics, Ident, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Const;
use crate::token::punct::{Colon, Eq, Semi};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A constant item inside a trait definition (`const NAME: Type;` or `const NAME: Type = expr;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemConst {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
    pub default: Option<Expr>,
}

impl Parse for TraitItemConst {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("const") {
            return Err(LexError::new(at).message("expected trait const").into());
        }

        let _ = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let _ = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;

        let default = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Expr>()?)
        } else {
            None
        };

        let _ = stream.parse::<Semi>();
        Ok(TraitItemConst {
            span: Span::default(),
            attrs,
            ident,
            generics,
            ty,
            default,
        })
    }
}

impl ToTokens for TraitItemConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Const::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);

        if let Some(d) = &self.default {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }

        Semi::default().to_tokens(t);
    }
}
