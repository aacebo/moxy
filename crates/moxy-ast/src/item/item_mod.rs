use moxy_token::{Delim, Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream, TokenTree};

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
    /// The inner attributes that open the content, as `#![a]`.
    pub inner_attrs: Attributes,
    pub content: Option<Delimited<Vec<Item>>>,
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
        let mut inner_attrs = Attributes::default();
        let (content, semi_punct) = if matches!(parser.curr(), Some(TokenTree::Group(g)) if g.delim == Delim::Brace) {
            let brace = Delimited::parse_brace_with(parser, |parser| {
                inner_attrs = Attributes::parse_inner(parser)?;
                <Vec<Item> as Parse>::parse(parser)
            })?;
            (Some(brace), None)
        } else {
            let semi_punct = <_ as Parse>::parse(parser)?;
            (None, Some(semi_punct))
        };

        Ok(Self {
            attrs,
            vis,
            unsafety,
            mod_keyword,
            ident,
            inner_attrs,
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

        if cursor.is_delimited(Delim::Brace) {
            let mut inner = Attributes::skip_inner(cursor.descend(Delim::Brace)?)?;

            while !inner.is_empty() {
                inner = Item::skip(inner)?;
            }

            Some(cursor.offset(1))
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

        match &self.content {
            Some(brace) => {
                let mut body = TokenStream::new();
                self.inner_attrs.to_tokens(&mut body);
                brace.inner.to_tokens(&mut body);
                brace.surround(t, body);
            }
            None => self.semi_punct.to_tokens(t),
        }
    }
}

impl ItemMod {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
