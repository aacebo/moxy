use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A `where` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct WhereClause {
    pub where_keyword: Token![where],
    pub predicates: Punctuated<WherePredicate, Token![,]>,
}

impl Parse for WhereClause {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![where]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            where_keyword: parser.parse()?,
            predicates: Punctuated::parse_separated_nonempty(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Token![where]>()?;
        cursor = cursor.skip::<WherePredicate>()?;

        while cursor.peek::<Token![,]>() {
            cursor = cursor.skip::<Token![,]>()?;
            cursor = cursor.skip::<WherePredicate>()?;
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
