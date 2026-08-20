use moxy_token::keyword::{Auto, Trait};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Eq, Plus, Semi};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Generics, Ident, Punctuated, TypeBound, Unsafety, Visibility};

/// A trait alias item (`trait Alias<T> = Bound1 + Bound2;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTraitAlias {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub trait_keyword: Trait,
    pub ident: Ident,
    pub generics: Generics,
    pub eq_punct: Eq,
    pub bounds: Punctuated<TypeBound, Plus>,
    pub semi_punct: Semi,
}

impl Parse for ItemTraitAlias {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let vis = stream.parse::<Visibility>()?;
        let _unsafety = stream.parse::<Unsafety>()?;

        // skip optional `auto`
        if stream.peek::<Auto>() {
            let _ = stream.parse::<Auto>()?;
        }

        let trait_keyword = stream.parse::<Trait>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let eq_punct = stream.parse::<Eq>()?;
        let bounds = crate::TypeBound::parse_bounds(stream)?;
        let semi_punct = stream.parse::<Semi>()?;
        Ok(Self {
            attrs,
            vis,
            trait_keyword,
            ident,
            generics,
            eq_punct,
            bounds,
            semi_punct,
        })
    }
}

impl Spanner for ItemTraitAlias {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi_punct.span())
    }
}

impl ToTokens for ItemTraitAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.trait_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.bounds.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}

impl ItemTraitAlias {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
