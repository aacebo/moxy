use moxy_token::{Delim, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A tuple pattern, e.g. `(a, b, c)`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTuple {
    pub attrs: Attributes,
    pub elems: Delimited<List<Pattern, Token![,]>>,
}

impl Attributed for PatTuple {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Spanner for PatTuple {
    fn span(&self) -> Span {
        self.to_token_stream().span()
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

        // `(..)` is a tuple, as is any list with a comma
        if PatRest::skip(inner).is_some_and(|rest| rest.is_empty()) {
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
        let (span, inner) = parser.parse_group_spanned(Delim::Paren)?;
        let elems: List<Pattern, Token![,]> = List::parse_all(&inner)?;

        // One element without a comma is a parenthesized pattern, except `..`
        if elems.len() == 1 && !elems.is_trailing() && !elems.first().is_some_and(Pattern::is_rest) {
            return Err(inner.expected("`,` after the only element of a tuple"));
        }

        Ok(Self {
            attrs,
            elems: Delimited::paren(span, elems),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let start = Attributes::skip(cursor)?;
        let inner = start.descend(Delim::Paren)?;
        let rest = PatRest::skip(inner).is_some_and(|rest| rest.is_empty());
        let one = Pattern::skip(inner).is_some_and(|after| after.is_empty());

        // One element without a comma is a parenthesized pattern, as in the
        // parser
        if one && !rest {
            return None;
        }

        Self::skip_paren_or_tuple(cursor)
    }
}

impl PatTuple {
    // Skips a tuple pattern or a parenthesized one, which the pattern parser
    // tells apart after reading the elements
    pub(crate) fn skip_paren_or_tuple(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
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

        // A lone rest pattern, as in `(..)`, needs no comma
        if self.elems.inner.first().is_some_and(Pattern::is_rest) {
            self.elems.to_tokens(t);
        } else {
            let mut inner = TokenStream::new();
            self.elems.inner.tuple_to_tokens(&mut inner);
            self.elems.surround(t, inner);
        }
    }
}
