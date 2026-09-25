use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Delimited, Generics, ImplItem, TraitRef, Type};

/// An `impl` block, optionally implementing a trait (`impl Trait for Type { ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemImpl {
    pub attrs: Attributes,
    pub defaultness: Option<Token![default]>,
    pub unsafety: Option<Token![unsafe]>,
    pub impl_keyword: Token![impl],
    pub generics: Generics,
    pub for_keyword: Option<Token![for]>,
    pub trait_ref: Option<TraitRef>,
    pub self_ty: Type,
    pub items: Delimited<Vec<ImplItem>>,
}

impl ItemImpl {
    fn type_to_trait_ref(ty: Type, polarity: Option<Token![!]>) -> Result<TraitRef, ParseError> {
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
        let cursor = Option::<Token![default]>::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![unsafe]>::skip(cursor).unwrap_or(cursor);
        <Token![impl]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let defaultness = <_ as Parse>::parse(parser)?;
        let unsafety = <_ as Parse>::parse(parser)?;
        let impl_keyword = <_ as Parse>::parse(parser)?;
        let mut generics: Generics = <_ as Parse>::parse(parser)?;
        let polarity = <_ as Parse>::parse(parser)?;

        let first = <_ as Parse>::parse(parser)?;
        let (for_keyword, trait_ref, self_ty) = if <Token![for]>::peek(parser.cursor()) {
            let for_keyword = <_ as Parse>::parse(parser)?;
            let self_ty = <_ as Parse>::parse(parser)?;
            (Some(for_keyword), Some(Self::type_to_trait_ref(first, polarity)?), self_ty)
        } else {
            (None, None, first)
        };

        generics.where_clause = <_ as Parse>::parse(parser)?;
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
        cursor = Option::<Token![default]>::skip(cursor)?;
        cursor = Option::<Token![unsafe]>::skip(cursor)?;
        cursor = <Token![impl]>::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        cursor = Option::<Token![!]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;

        if <Token![for]>::peek(cursor) {
            cursor = <Token![for]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
        }

        cursor = Option::<crate::WhereClause>::skip(cursor)?;
        let mut inner = cursor.descend(moxy_token::Delim::Brace)?;

        while !inner.is_empty() {
            inner = ImplItem::skip(inner)?;
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
