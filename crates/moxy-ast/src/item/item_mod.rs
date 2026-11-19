use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A module item (`mod foo;` or `mod foo { ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMod {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub unsafety: Option<Token![unsafe]>,
    pub mod_keyword: Token![mod],
    pub ident: Ident,
    pub content: Option<ItemModContent>,
    pub semi_punct: Option<Token![;]>,
}

impl Attributed for ItemMod {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ItemMod {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![unsafe]>::skip(cursor).unwrap_or(cursor);
        <Token![mod]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let unsafety = <_ as Parse>::parse(parser)?;
        let mod_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        // The body is read here rather than in its own parse, which would add
        // a frame per nested module
        let (content, semi_punct) = if ItemModContent::peek(parser.cursor()) {
            let mut inner_attrs = Attributes::default();
            let items = Delimited::parse_brace_with(parser, |parser| {
                inner_attrs = Attributes::parse_inner(parser)?;
                <Vec<Item> as Parse>::parse(parser)
            })?;
            (Some(ItemModContent { inner_attrs, items }), None)
        } else {
            (None, Some(<_ as Parse>::parse(parser)?))
        };

        Ok(Self {
            attrs,
            vis,
            unsafety,
            mod_keyword,
            ident,
            content,
            semi_punct,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Option::<Token![unsafe]>::skip(cursor)?;
        cursor = <Token![mod]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;

        if ItemModContent::peek(cursor) {
            ItemModContent::skip(cursor)
        } else {
            <Token![;]>::skip(cursor)
        }
    }
}

impl Spanner for ItemMod {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ItemMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.mod_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.content.to_tokens(t);
        self.semi_punct.to_tokens(t);
    }
}

impl ItemMod {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
