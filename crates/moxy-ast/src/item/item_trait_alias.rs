use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Generics, Punctuated, TypeBound, Visibility};

/// A trait alias item (`trait Alias<T> = Bound1 + Bound2;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![unsafe]>::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![auto]>::skip(cursor).unwrap_or(cursor);
        let Some(cursor) = <Token![trait]>::skip(cursor) else {
            return false;
        };

        let Some(cursor) = Ident::skip(cursor) else {
            return false;
        };

        let Some(cursor) = Generics::skip(cursor) else {
            return false;
        };

        <Token![=]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let _unsafety: Option<Token![unsafe]> = <_ as Parse>::parse(parser)?;

        // skip optional `auto`
        if <Token![auto]>::peek(parser.cursor()) {
            let _: Token![auto] = <_ as Parse>::parse(parser)?;
        }

        let trait_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let generics = <_ as Parse>::parse(parser)?;
        let eq_punct = <_ as Parse>::parse(parser)?;
        let bounds = crate::TypeBound::parse_bounds(parser)?;
        let semi_punct = <_ as Parse>::parse(parser)?;

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

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Option::<Token![unsafe]>::skip(cursor)?;
        cursor = Option::<Token![auto]>::skip(cursor)?;
        cursor = <Token![trait]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        cursor = <Token![=]>::skip(cursor)?;
        cursor = TypeBound::skip(cursor)?;

        while <Token![+]>::peek(cursor) {
            cursor = <Token![+]>::skip(cursor)?;
            cursor = TypeBound::skip(cursor)?;
        }

        <Token![;]>::skip(cursor)
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
