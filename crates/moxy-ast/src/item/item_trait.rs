use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

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
    pub supertraits: List<TypeBound, Token![+]>,
    /// The inner attributes that open the body, as `#![a]`.
    pub inner_attrs: Attributes,
    pub items: Delimited<Vec<TraitItem>>,
}

impl Attributed for ItemTrait {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ItemTrait {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![unsafe]>::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![auto]>::skip(cursor).unwrap_or(cursor);
        <Token![trait]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match Self::parse_or_alias(parser)? {
            super::Item::Trait(item) => Ok(item),
            _ => parser.error("expected trait").into(),
        }
    }

    fn skip(cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        let (cursor, alias) = Self::skip_or_alias(cursor)?;
        (!alias).then_some(cursor)
    }
}

impl ItemTrait {
    // Parses a trait or a trait alias, which share their start up to the
    // generics, so the generics are read once and the `=` after them tells an
    // alias
    pub(crate) fn parse_or_alias(parser: &Parser) -> Result<super::Item, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let unsafety: Option<Token![unsafe]> = <_ as Parse>::parse(parser)?;
        let auto_keyword: Option<Token![auto]> = <_ as Parse>::parse(parser)?;
        let trait_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let mut generics: Generics = <_ as Parse>::parse(parser)?;

        if <Token![=]>::peek(parser.cursor()) {
            // As in rustc, a trait alias is neither `unsafe` nor `auto`
            if unsafety.is_some() || auto_keyword.is_some() {
                return parser.error("a trait alias cannot be `unsafe` or `auto`").into();
            }

            return Ok(super::Item::TraitAlias(ItemTraitAlias {
                attrs,
                vis,
                trait_keyword,
                ident,
                generics,
                eq_punct: <_ as Parse>::parse(parser)?,
                bounds: TypeBound::parse_bounds(parser)?,
                semi_punct: <_ as Parse>::parse(parser)?,
            }));
        }

        // Supertraits come before a where clause, never after it
        if generics.where_clause.is_some() && <Token![:]>::peek(parser.cursor()) {
            return parser.error("expected `{`").into();
        }

        let (colon_punct, supertraits) = if <Token![:]>::peek(parser.cursor()) {
            let colon_punct = <_ as Parse>::parse(parser)?;
            let supertraits = if !<Token![;]>::peek(parser.cursor()) && TypeBound::peek(parser.cursor()) {
                TypeBound::parse_bounds(parser)?
            } else {
                List::new()
            };

            (Some(colon_punct), supertraits)
        } else {
            (None, List::new())
        };

        // Without supertraits, `Generics` has already read a where clause that
        // directly follows the parameters
        if generics.where_clause.is_none() {
            generics.where_clause = <_ as Parse>::parse(parser)?;
        }

        let mut inner_attrs = Attributes::default();
        let items = Delimited::parse_brace_with(parser, |parser| {
            inner_attrs = Attributes::parse_inner(parser)?;
            parser.parse_until_empty::<TraitItem>()
        })?;

        Ok(super::Item::Trait(Self {
            attrs,
            vis,
            unsafety,
            auto_keyword,
            trait_keyword,
            ident,
            generics,
            colon_punct,
            supertraits,
            inner_attrs,
            items,
        }))
    }

    // Skips a trait or a trait alias as `parse_or_alias` reads them, and
    // tells whether it was an alias
    pub(crate) fn skip_or_alias(mut cursor: crate::Cursor<'_>) -> Option<(crate::Cursor<'_>, bool)> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        let modified = <Token![unsafe]>::peek(cursor) || <Token![auto]>::peek(cursor);
        cursor = Option::<Token![unsafe]>::skip(cursor)?;
        cursor = Option::<Token![auto]>::skip(cursor)?;
        cursor = <Token![trait]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        let start = cursor;
        cursor = Generics::skip(cursor)?;

        if <Token![=]>::peek(cursor) {
            if modified {
                return None;
            }

            cursor = <Token![=]>::skip(cursor)?;
            cursor = TypeBound::skip_bounds(cursor)?;
            return Some((<Token![;]>::skip(cursor)?, true));
        }

        if <Token![:]>::peek(cursor) {
            if cursor.range(start).iter().any(|token| token.is_keyword_where()) {
                return None;
            }

            cursor = <Token![:]>::skip(cursor)?;

            if TypeBound::peek(cursor) {
                cursor = TypeBound::skip(cursor)?;

                while <Token![+]>::peek(cursor) {
                    cursor = <Token![+]>::skip(cursor)?;
                    cursor = TypeBound::skip(cursor)?;
                }
            }
        }

        cursor = Option::<crate::WhereClause>::skip(cursor)?;
        let mut inner = Attributes::skip_inner(cursor.descend(moxy_token::Delim::Brace)?)?;

        while !inner.is_empty() {
            inner = TraitItem::skip(inner)?;
        }

        Some((cursor.offset(1), false))
    }
}

impl Spanner for ItemTrait {
    fn span(&self) -> Span {
        self.to_token_stream().span()
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
        self.generics.lt.to_tokens(t);
        self.generics.params.to_tokens(t);
        self.generics.gt.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.supertraits.to_tokens(t);
        self.generics.where_clause.to_tokens(t);
        let mut body = TokenStream::new();
        self.inner_attrs.to_tokens(&mut body);
        self.items.inner.to_tokens(&mut body);
        self.items.surround(t, body);
    }
}

impl ItemTrait {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
