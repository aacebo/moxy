use moxy_token::keyword::Where;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use super::WherePredicate;
use crate::Punctuated;

#[doc = "A `where` clause."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct WhereClause {
    pub span: Span,
    pub where_keyword: Where,
    pub predicates: Punctuated<WherePredicate, Comma>,
}

impl Parse for WhereClause {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let where_keyword = stream.parse::<Where>()?;
        let mut predicates = Punctuated::new();

        while !stream.is_empty() && !matches!(stream.curr(), Some(moxy_token::TokenTree::Group(_))) {
            predicates.push_value(stream.parse::<WherePredicate>()?);
            if stream.peek::<Comma>().is_some() {
                predicates.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        Ok(Self {
            span: Span::default(),
            where_keyword,
            predicates,
        })
    }
}

impl ToTokens for WhereClause {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.where_keyword.to_tokens(t);
        self.predicates.to_tokens(t);
    }
}
