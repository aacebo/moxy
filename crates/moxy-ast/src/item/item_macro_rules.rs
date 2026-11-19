use moxy_token::{Delim, Group, Ident, LexError, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A `macro_rules!` definition item.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacroRules {
    pub attrs: Attributes,
    pub macro_rules_keyword: Token![macro_rules],
    pub not_punct: Token![!],
    pub ident: Ident,
    pub body: Group,
    /// The `;` that ends a definition whose body is in parentheses or brackets.
    pub semi: Option<Token![;]>,
}

impl Attributed for ItemMacroRules {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ItemMacroRules {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        // `macro_rules` without `!` is an identifier, as in `macro_rules + 1`
        Attributes::skip(cursor)
            .and_then(|cursor| <Token![macro_rules]>::skip(cursor))
            .is_some_and(|cursor| <Token![!]>::peek(cursor))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let macro_rules_keyword = <_ as Parse>::parse(parser)?;
        let not_punct = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let body = match parser.curr() {
            Some(TokenTree::Group(g)) => {
                let g = g.clone();
                parser.advance();
                g
            }

            _ => {
                return Err(LexError::new(parser.span()).message("expected macro body").into());
            }
        };

        // Only a brace-delimited definition ends without a semicolon
        let semi = if body.delim.is_brace() {
            None
        } else if <Token![;]>::peek(parser.cursor()) {
            Some(<_ as Parse>::parse(parser)?)
        } else {
            return parser
                .error("expected `;` after a `macro_rules!` body in parentheses or brackets")
                .into();
        };

        Ok(Self {
            attrs,
            macro_rules_keyword,
            not_punct,
            ident,
            body,
            semi,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = <Token![macro_rules]>::skip(cursor)?;
        cursor = <Token![!]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;

        if cursor.is_delimited(Delim::Brace) {
            return Group::skip(cursor);
        }

        <Token![;]>::skip(Group::skip(cursor)?)
    }
}

impl Spanner for ItemMacroRules {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.body.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ItemMacroRules {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.macro_rules_keyword.to_tokens(t);
        self.not_punct.to_tokens(t);
        self.ident.to_tokens(t);
        t.extend_one(TokenTree::Group(self.body.clone()));
        self.semi.to_tokens(t);
    }
}

impl ItemMacroRules {
    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}
