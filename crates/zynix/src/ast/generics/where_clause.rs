use super::WherePredicate;
use crate::ast::Punctuated;
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Where;
use crate::token::punct::Comma;
use crate::{Parse, Span, TokenStream};

#[doc = "A `where` clause."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct WhereClause {
    pub span: Span,
    pub predicates: Punctuated<WherePredicate, Comma>,
}

impl Parse for WhereClause {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Where>()?;
        let mut predicates = Punctuated::new();

        while !stream.is_empty() && !matches!(stream.curr(), Some(crate::TokenTree::Group(_))) {
            predicates.push_value(stream.parse::<WherePredicate>()?);
            if stream.peek::<Comma>().is_some() {
                predicates.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        Ok(Self {
            span: Span::default(),
            predicates,
        })
    }
}

impl ToTokens for WhereClause {
    fn to_tokens(&self, t: &mut TokenStream) {
        Where::default().to_tokens(t);
        self.predicates.to_tokens(t);
    }
}
