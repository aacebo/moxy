use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, BoundPolarity, Defaultness, Delimited, Generics, ImplItem, TraitRef, Type, Unsafety};

/// An `impl` block, optionally implementing a trait (`impl Trait for Type { ... }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemImpl {
    pub attrs: Attributes,
    pub defaultness: Defaultness,
    pub unsafety: Unsafety,
    pub impl_keyword: Token![impl],
    pub generics: Generics,
    pub for_keyword: Option<Token![for]>,
    pub trait_ref: Option<TraitRef>,
    pub self_ty: Type,
    pub items: Delimited<Vec<ImplItem>>,
}

impl ItemImpl {
    fn type_to_trait_ref(ty: Type, polarity: BoundPolarity) -> Result<TraitRef, ParseError> {
        match ty {
            Type::Path(tp) => Ok(TraitRef { polarity, path: tp.path }),
            _ => Err(LexError::new(Span::default()).message("expected trait path").into()),
        }
    }

    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}

impl Parse for ItemImpl {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let defaultness = stream.parse::<Defaultness>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let impl_keyword = stream.parse::<Token![impl]>()?;
        let mut generics = stream.parse::<Generics>()?;
        let polarity = if stream.peek::<Token![!]>() {
            BoundPolarity::Negative(stream.parse::<Token![!]>()?)
        } else {
            BoundPolarity::Positive
        };

        let first = stream.parse::<Type>()?;
        let (for_keyword, trait_ref, self_ty) = if stream.peek::<Token![for]>() {
            let for_keyword = stream.parse::<Token![for]>()?;
            let self_ty = stream.parse::<Type>()?;
            (Some(for_keyword), Some(Self::type_to_trait_ref(first, polarity)?), self_ty)
        } else {
            (None, None, first)
        };

        generics.where_clause = stream.parse_if();
        let items = Delimited::<Vec<ImplItem>>::parse_brace(stream)?;

        Ok(Self {
            attrs,
            defaultness,
            unsafety,
            impl_keyword,
            generics,
            for_keyword,
            trait_ref,
            self_ty,
            items,
        })
    }
}

impl Spanner for ItemImpl {
    fn span(&self) -> Span {
        self.attrs.span().join(self.items.span())
    }
}

impl ToTokens for ItemImpl {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.impl_keyword.to_tokens(t);
        self.generics.to_tokens(t);

        if let Some(tr) = &self.trait_ref {
            tr.to_tokens(t);
            if let Some(for_keyword) = &self.for_keyword {
                for_keyword.to_tokens(t);
            }
        }

        self.self_ty.to_tokens(t);
        self.items.to_tokens(t);
    }
}
