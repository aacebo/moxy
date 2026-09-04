use crate::Token;
use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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
    pub lt: Option<Token![<]>,
    pub gt: Option<Token![>]>,
    pub params: Punctuated<GenericParam, Token![,]>,
    pub where_clause: Option<WhereClause>,
}

impl Parse for Generics {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let lt = parser.parse_if();

        let params = if lt.is_some() {
            Punctuated::parse_separated_nonempty(parser)?
        } else {
            Punctuated::new()
        };

        let gt = parser.parse_if();
        let where_clause = parser.parse_if();

        Ok(Self {
            lt,
            gt,
            params,
            where_clause,
        })
    }
}

impl Spanner for Generics {
    fn span(&self) -> Span {
        let end = if let Some(w) = &self.where_clause {
            w.span()
        } else {
            self.gt.map(|v| v.span()).unwrap_or(self.params.span())
        };

        self.lt.map(|v| v.span()).unwrap_or(self.params.span()).join(end)
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lt.to_tokens(t);
        self.params.to_tokens(t);
        self.gt.to_tokens(t);
        self.where_clause.to_tokens(t);
    }
}
