use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A `where` clause.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct WhereClause {
    pub where_keyword: Token![where],
    pub predicates: Punctuated<WherePredicate, Token![,]>,
}

impl Parse for WhereClause {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![where]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let where_keyword = <_ as Parse>::parse(parser)?;
        let mut predicates = Punctuated::new();

        while WherePredicate::peek(parser.cursor()) {
            predicates.push_value(<_ as Parse>::parse(parser)?);

            if !<Token![,]>::peek(parser.cursor()) {
                break;
            }

            predicates.push_punct(<_ as Parse>::parse(parser)?);
        }

        Ok(Self {
            where_keyword,
            predicates,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = <Token![where]>::skip(cursor)?;
        cursor = WherePredicate::skip(cursor)?;

        while <Token![,]>::peek(cursor) {
            cursor = <Token![,]>::skip(cursor)?;
            cursor = WherePredicate::skip(cursor)?;
        }

        Some(cursor)
    }
}

impl Spanner for WhereClause {
    fn span(&self) -> Span {
        let end = self
            .predicates
            .last()
            .map(|p| p.span())
            .unwrap_or_else(|| self.where_keyword.span());
        self.where_keyword.span().join(end)
    }
}

impl ToTokens for WhereClause {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.where_keyword.to_tokens(t);
        self.predicates.to_tokens(t);
    }
}
