use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Generics, Ident, Punctuated, TypeBound, Unsafety, Visibility};

/// A trait alias item (`trait Alias<T> = Bound1 + Bound2;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTraitAlias {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub trait_keyword: Token![trait],
    pub ident: Ident,
    pub generics: Generics,
    pub eq_punct: Token![=],
    pub bounds: Punctuated<TypeBound, Token![+]>,
    pub semi_punct: Token![;],
}

impl Parse for ItemTraitAlias {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse::<Attributes>()?;
        let vis = parser.parse::<Visibility>()?;
        let _unsafety = parser.parse::<Unsafety>()?;

        // skip optional `auto`
        if parser.peek::<Token![auto]>() {
            let _ = parser.parse::<Token![auto]>()?;
        }

        let trait_keyword = parser.parse::<Token![trait]>()?;
        let ident = parser.parse::<Ident>()?;
        let generics = parser.parse::<Generics>()?;
        let eq_punct = parser.parse::<Token![=]>()?;
        let bounds = crate::TypeBound::parse_bounds(parser)?;
        let semi_punct = parser.parse::<Token![;]>()?;

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
