use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A tuple pattern, e.g. `(a, b, c)`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTuple {
    pub attrs: Attributes,
    pub elems: Delimited<Punctuated<Pattern, Token![,]>>,
}

impl Spanner for PatTuple {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl Parse for PatTuple {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let Some(mut inner) = cursor.descend(Delim::Paren) else {
            return false;
        };

        if inner.is_empty() {
            return true;
        }

        let Some(next) = Pattern::skip(inner) else {
            return false;
        };

        inner = next;
        !inner.is_empty() && <Token![,]>::peek(inner)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;

        Ok(Self {
            attrs,
            elems: Delimited::paren(span, Punctuated::parse_terminated(&parser)?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = Attributes::skip(cursor)?;
        let mut inner = cursor.descend(Delim::Paren)?;

        while !inner.is_empty() {
            inner = Pattern::skip(inner)?;

            if inner.is_empty() {
                break;
            }

            inner = <Token![,]>::skip(inner)?;
        }

        Some(cursor.offset(1))
    }
}

impl ToTokens for PatTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.elems.to_tokens(t);
    }
}
