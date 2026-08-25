use moxy_token::keyword::Where;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Gt, Lt};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::Punctuated;

mod const_param;
mod generic_param;
mod lifetime_param;
mod lifetime_predicate;
mod trait_bound;
mod trait_ref;
mod type_bound;
mod type_param;
mod type_predicate;
mod use_bound;
mod where_clause;
mod where_predicate;

pub use const_param::*;
pub use generic_param::*;
pub use lifetime_param::*;
pub use lifetime_predicate::*;
pub use trait_bound::*;
pub use trait_ref::*;
pub use type_bound::*;
pub use type_param::*;
pub use type_predicate::*;
pub use use_bound::*;
pub use where_clause::*;
pub use where_predicate::*;

/// Generic parameters and an optional `where` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Generics {
    pub lt_punct: Lt,
    pub gt_punct: Gt,
    pub params: Punctuated<GenericParam, Comma>,
    pub where_clause: Option<WhereClause>,
}

impl Parse for Generics {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let params = if stream.peek::<Lt>() {
            let _ = stream.parse::<Lt>()?;
            let params = Punctuated::parse_separated_nonempty(stream)?;
            let _ = stream.parse::<Gt>()?;
            params
        } else {
            Punctuated::new()
        };

        let where_clause = if stream.peek::<Where>() {
            Some(stream.parse::<WhereClause>()?)
        } else {
            None
        };

        Ok(Self {
            lt_punct: Lt::default(),
            gt_punct: Gt::default(),
            params,
            where_clause,
        })
    }
}

impl Spanner for Generics {
    fn span(&self) -> Span {
        if self.params.is_empty() {
            return Span::call_site();
        }

        let end = if let Some(w) = &self.where_clause {
            w.span()
        } else {
            self.gt_punct.span()
        };

        self.lt_punct.span().join(end)
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, t: &mut TokenStream) {
        if !self.params.is_empty() {
            self.lt_punct.to_tokens(t);
            self.params.to_tokens(t);
            self.gt_punct.to_tokens(t);
        }

        if let Some(w) = &self.where_clause {
            w.to_tokens(t);
        }
    }
}
