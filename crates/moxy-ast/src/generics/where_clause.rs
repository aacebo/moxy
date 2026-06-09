use moxy_token::keyword::Where;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::WherePredicate;
use crate::Punctuated;

/// A `where` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct WhereClause {
    pub where_keyword: Where,
    pub predicates: Punctuated<WherePredicate, Comma>,
}

impl Parse for WhereClause {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let where_keyword = stream.parse::<Where>()?;
        let mut predicates = Punctuated::new();

        while !stream.is_empty() && !matches!(stream.curr(), Some(moxy_token::TokenTree::Group(_))) {
            predicates.push_value(stream.parse::<WherePredicate>()?);
            if stream.peek::<Comma>() {
                predicates.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        Ok(Self {
            where_keyword,
            predicates,
        })
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
