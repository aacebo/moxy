use crate::ast::{Attribute, Generics, Ident, Punctuated, TypeBound, Unsafety, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::{Auto, Trait};
use crate::token::punct::{Eq, Plus, Semi};
use crate::{Parse, Span, TokenStream};

#[doc = "A trait alias item (`trait Alias<T> = Bound1 + Bound2;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTraitAlias {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for ItemTraitAlias {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let _unsafety = stream.parse::<Unsafety>()?;

        // skip optional `auto`
        if stream.peek::<Auto>().is_some() {
            let _ = stream.parse::<Auto>()?;
        }

        let _ = stream.parse::<Trait>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let _ = stream.parse::<Eq>()?;
        let bounds = crate::ast::TypeBound::parse_bounds(stream)?;
        let _ = stream.parse::<Semi>();
        Ok(ItemTraitAlias {
            span: Span::default(),
            attrs,
            vis,
            ident,
            generics,
            bounds,
        })
    }
}

impl ToTokens for ItemTraitAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        Trait::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Eq::default().to_tokens(t);
        self.bounds.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
