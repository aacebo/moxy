use moxy_token::keyword::{For, Impl};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Not;
use moxy_token::{Delim, LexError, Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, BoundPolarity, Defaultness, Generics, ImplItem, TraitRef, Type, Unsafety};

#[doc = "An `impl` block, optionally implementing a trait (`impl Trait for Type { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemImpl {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub defaultness: Defaultness,
    pub unsafety: Unsafety,
    pub impl_keyword: Impl,
    pub generics: Generics,
    pub for_keyword: Option<For>,
    pub trait_ref: Option<TraitRef>,
    pub self_ty: Type,
    pub items: Vec<ImplItem>,
}

impl ItemImpl {
    fn type_to_trait_ref(ty: Type, polarity: BoundPolarity) -> Result<TraitRef, ParseError> {
        match ty {
            Type::Path(tp) => Ok(TraitRef {
                span: Span::default(),
                polarity,
                path: tp.path,
            }),
            _ => Err(LexError::new(Span::default()).message("expected trait path").into()),
        }
    }
}

impl Parse for ItemImpl {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let defaultness = stream.parse::<Defaultness>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let impl_keyword = stream.parse::<Impl>()?;
        let generics = stream.parse::<Generics>()?;

        // Optional `!` for a negative impl (`impl !Trait for T`).
        let polarity = if stream.peek::<Not>().is_some() {
            let _ = stream.parse::<Not>()?;
            BoundPolarity::Negative
        } else {
            BoundPolarity::Positive
        };

        // `impl Trait for Type` vs `impl Type`. Parse a type; if `for` follows, it was the trait.
        let first = stream.parse::<Type>()?;

        let (for_keyword, trait_ref, self_ty) = if stream.peek::<For>().is_some() {
            let for_keyword = stream.parse::<For>()?;
            let self_ty = stream.parse::<Type>()?;
            (
                Some(for_keyword),
                Some(ItemImpl::type_to_trait_ref(first, polarity)?),
                self_ty,
            )
        } else {
            (None, None, first)
        };

        let mut generics = generics;

        if stream.peek::<moxy_token::keyword::Where>().is_some() {
            generics.where_clause = Some(stream.parse()?);
        }

        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let items = inner.parse_vec::<ImplItem>()?;
        Ok(ItemImpl {
            span: Span::default(),
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

impl ToTokens for ItemImpl {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
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
        let mut inner = TokenStream::new();

        for it in &self.items {
            it.to_tokens(&mut inner);
        }

        t.extend_one(moxy_token::TokenTree::Group(moxy_token::Group::new(
            moxy_token::Delim::Brace,
            inner,
        )));
    }
}
