use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

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
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Defaultness::skip(cursor).unwrap_or(cursor);
        let cursor = Unsafety::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![impl]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let defaultness = parser.parse()?;
        let unsafety = parser.parse()?;
        let impl_keyword = parser.parse()?;
        let mut generics: Generics = parser.parse()?;
        let polarity = if parser.peek::<Token![!]>() {
            BoundPolarity::Negative(parser.parse()?)
        } else {
            BoundPolarity::Positive
        };

        let first = parser.parse()?;
        let (for_keyword, trait_ref, self_ty) = if parser.peek::<Token![for]>() {
            let for_keyword = parser.parse()?;
            let self_ty = parser.parse()?;
            (Some(for_keyword), Some(Self::type_to_trait_ref(first, polarity)?), self_ty)
        } else {
            (None, None, first)
        };

        generics.where_clause = parser.parse()?;
        let items = Delimited::<Vec<ImplItem>>::parse_brace(parser)?;

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

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Defaultness::skip(cursor)?;
        cursor = Unsafety::skip(cursor)?;
        cursor = cursor.skip::<Token![impl]>()?;
        cursor = Generics::skip(cursor)?;
        cursor = BoundPolarity::skip(cursor)?;
        cursor = cursor.skip::<Type>()?;

        if cursor.peek::<Token![for]>() {
            cursor = cursor.skip::<Token![for]>()?;
            cursor = cursor.skip::<Type>()?;
        }

        cursor = cursor.skip::<Option<crate::WhereClause>>()?;
        let mut inner = cursor.descend(moxy_token::Delim::Brace)?;

        while !inner.is_empty() {
            inner = inner.skip::<ImplItem>()?;
        }

        Some(cursor.offset(1))
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
        self.trait_ref.to_tokens(t);
        self.for_keyword.to_tokens(t);
        self.self_ty.to_tokens(t);
        self.items.to_tokens(t);
    }
}
