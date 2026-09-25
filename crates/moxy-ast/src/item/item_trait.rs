use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Delimited, Generics, Ident, Punctuated, TraitItem, TypeBound, Visibility};

/// A trait definition item (`trait Name: Super { ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTrait {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub unsafety: Option<Token![unsafe]>,
    pub auto_keyword: Option<Token![auto]>,
    pub trait_keyword: Token![trait],
    pub ident: Ident,
    pub generics: Generics,
    pub colon_punct: Option<Token![:]>,
    pub supertraits: Punctuated<TypeBound, Token![+]>,
    pub items: Delimited<Vec<TraitItem>>,
}

impl Parse for ItemTrait {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = cursor.skip::<Option<Token![unsafe]>>().unwrap_or(cursor);
        let cursor = cursor.skip::<Option<Token![auto]>>().unwrap_or(cursor);
        cursor.peek::<Token![trait]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let vis = parser.parse()?;
        let unsafety = parser.parse()?;
        let auto_keyword = if parser.peek::<Token![auto]>() {
            Some(parser.parse()?)
        } else {
            None
        };

        let trait_keyword = parser.parse()?;
        let ident = parser.parse()?;
        let mut generics: Generics = parser.parse()?;
        let (colon_punct, supertraits) = if parser.peek::<Token![:]>() {
            let colon_punct = parser.parse()?;
            let supertraits = if !parser.peek::<Token![;]>() && parser.peek::<TypeBound>() {
                TypeBound::parse_bounds(parser)?
            } else {
                Punctuated::new()
            };

            (Some(colon_punct), supertraits)
        } else {
            (None, Punctuated::new())
        };

        generics.where_clause = parser.parse()?;
        let items = Delimited::parse_brace_with(parser, |parser| parser.parse_until_empty::<TraitItem>())?;

        Ok(Self {
            attrs,
            vis,
            unsafety,
            auto_keyword,
            trait_keyword,
            ident,
            generics,
            colon_punct,
            supertraits,
            items,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = cursor.skip::<Option<Token![unsafe]>>()?;
        cursor = cursor.skip::<Option<Token![auto]>>()?;
        cursor = cursor.skip::<Token![trait]>()?;
        cursor = cursor.skip::<Ident>()?;
        cursor = Generics::skip(cursor)?;

        if cursor.peek::<Token![:]>() {
            cursor = cursor.skip::<Token![:]>()?;

            if cursor.peek::<TypeBound>() {
                cursor = cursor.skip::<TypeBound>()?;

                while cursor.peek::<Token![+]>() {
                    cursor = cursor.skip::<Token![+]>()?;
                    cursor = cursor.skip::<TypeBound>()?;
                }
            }
        }

        cursor = cursor.skip::<Option<crate::WhereClause>>()?;
        let mut inner = cursor.descend(moxy_token::Delim::Brace)?;

        while !inner.is_empty() {
            inner = inner.skip::<TraitItem>()?;
        }

        Some(cursor.offset(1))
    }
}

impl Spanner for ItemTrait {
    fn span(&self) -> Span {
        self.attrs.span().join(self.items.span())
    }
}

impl ToTokens for ItemTrait {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.auto_keyword.to_tokens(t);
        self.trait_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.supertraits.to_tokens(t);
        self.items.to_tokens(t);
    }
}

impl ItemTrait {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
