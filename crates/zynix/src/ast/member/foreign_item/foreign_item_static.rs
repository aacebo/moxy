use super::ForeignItem;
use crate::ast::{Attribute, Ident, Mutability, Type, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Static;
use crate::token::punct::{Colon, Semi};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A foreign static declaration inside an `extern` block (`static NAME: Type;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemStatic {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Ident,
    pub ty: Type,
}

impl Parse for ForeignItemStatic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;

        if stream.curr().and_then(|t| t.name()).as_deref() != Some("static") {
            return Err(LexError::new(at).message("expected foreign static").into());
        }

        let _ = stream.parse::<Static>()?;
        let mutability = stream.parse::<Mutability>()?;
        let ident = stream.parse::<Ident>()?;
        let _ = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;
        let _ = stream.parse::<Semi>();
        Ok(ForeignItemStatic {
            span: Span::default(),
            attrs,
            vis,
            mutability,
            ident,
            ty,
        })
    }
}

impl ToTokens for ForeignItemStatic {
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
        Semi::default().to_tokens(t);
    }
}
