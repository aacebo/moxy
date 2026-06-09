use moxy_token::keyword::Type as KwType;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq, Plus, Semi};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Generics, Ident, Punctuated, Type, TypeBound};

/// An associated type inside a trait definition (`type Name: Bound = Default;`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemType {
    pub attrs: Vec<Attribute>,
    pub type_keyword: KwType,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Option<Colon>,
    pub bounds: Punctuated<TypeBound, Plus>,
    pub default: Option<(Eq, Type)>,
    pub semi: Option<Semi>,
}

impl Parse for TraitItemType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse::<Vec<Attribute>>()?;

        if stream.curr().and_then(|t| t.text()) != Some("type") {
            return Err(LexError::new(at).message("expected trait type").into());
        }

        let type_keyword = stream.parse::<KwType>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;

        let (colon, bounds) = if stream.peek::<Colon>() {
            let colon = stream.parse::<Colon>()?;
            (Some(colon), crate::TypeBound::parse_bounds(stream)?)
        } else {
            (None, Punctuated::new())
        };

        let default = if stream.peek::<Eq>() {
            let eq = stream.parse::<Eq>()?;
            Some((eq, stream.parse::<Type>()?))
        } else {
            None
        };

        let semi = stream.parse_if::<Semi>();
        Ok(TraitItemType {
            attrs,
            type_keyword,
            ident,
            generics,
            colon,
            bounds,
            default,
            semi,
        })
    }
}

impl Spanner for TraitItemType {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.type_keyword.span()
        };

        start.join(
            self.semi
                .as_ref()
                .map(|s| s.span())
                .or_else(|| self.default.as_ref().map(|(_, t)| t.span()))
                .unwrap_or(self.ident.span()),
        )
    }
}

impl ToTokens for TraitItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);

        if !self.bounds.is_empty() {
            self.colon.to_tokens(t);
            self.bounds.to_tokens(t);
        }

        if let Some((eq, d)) = &self.default {
            eq.to_tokens(t);
            d.to_tokens(t);
        }

        self.semi.to_tokens(t);
    }
}

impl TraitItemType {
    pub fn into_trait_item(self) -> super::TraitItem {
        super::TraitItem::from(self)
    }
}
