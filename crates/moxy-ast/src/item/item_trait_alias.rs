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
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Unsafety::skip(cursor).unwrap_or(cursor);
        let cursor = cursor.skip::<Option<Token![auto]>>().unwrap_or(cursor);
        let Some(cursor) = cursor.skip::<Token![trait]>() else {
            return false;
        };

        let Some(cursor) = cursor.skip::<Ident>() else {
            return false;
        };

        let Some(cursor) = Generics::skip(cursor) else {
            return false;
        };

        cursor.peek::<Token![=]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let vis = parser.parse()?;
        let _unsafety: Unsafety = parser.parse()?;

        // skip optional `auto`
        if parser.peek::<Token![auto]>() {
            let _: Token![auto] = parser.parse()?;
        }

        let trait_keyword = parser.parse()?;
        let ident = parser.parse()?;
        let generics = parser.parse()?;
        let eq_punct = parser.parse()?;
        let bounds = crate::TypeBound::parse_bounds(parser)?;
        let semi_punct = parser.parse()?;

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
        cursor = Unsafety::skip(cursor)?;
        cursor = cursor.skip::<Option<Token![auto]>>()?;
        cursor = cursor.skip::<Token![trait]>()?;
        cursor = cursor.skip::<Ident>()?;
        cursor = Generics::skip(cursor)?;
        cursor = cursor.skip::<Token![=]>()?;
        cursor = cursor.skip::<TypeBound>()?;

        while cursor.peek::<Token![+]>() {
            cursor = cursor.skip::<Token![+]>()?;
            cursor = cursor.skip::<TypeBound>()?;
        }

        cursor.skip::<Token![;]>()
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
