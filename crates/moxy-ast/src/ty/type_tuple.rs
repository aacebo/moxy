use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::{Cursor, Delimited, List, Parse, ParseError, Parser, Type};

/// A tuple type (e.g. `()`, `(A, B)`, `(T,)`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTuple {
    pub elems: Delimited<List<Type, Token![,]>>,
}

impl Parse for TypeTuple {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(mut inner) = cursor.descend(Delim::Paren) else {
            return false;
        };

        if inner.is_empty() {
            return true;
        }

        let Some(next) = Type::skip(inner) else {
            return false;
        };

        inner = next;

        if inner.is_empty() {
            return false;
        }

        while !inner.is_empty() {
            let Some(next) = <Token![,]>::skip(inner) else {
                return false;
            };

            inner = next;

            if inner.is_empty() {
                return true;
            }

            let Some(next) = Type::skip(inner) else {
                return false;
            };

            inner = next;
        }

        true
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let elems = Delimited::parse_paren_with(parser, List::parse_all)?;
        Ok(Self { elems })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Spanner for TypeTuple {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl ToTokens for TypeTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.elems.to_tokens(tokens);
    }
}
