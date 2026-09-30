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

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// Generic parameters and an optional `where` clause.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Generics {
    pub lt: Option<Token![<]>,
    pub params: Punctuated<GenericParam, Token![,]>,
    pub gt: Option<Token![>]>,
    pub where_clause: Option<WhereClause>,
}

impl Generics {
    pub fn split(&self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>) {
        (self.into(), self.into(), self.where_clause.as_ref())
    }
}

impl Parse for Generics {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![<]>::peek(cursor) || WhereClause::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let lt: Option<Token![<]> = <_ as Parse>::parse(parser)?;
        let params = if lt.is_some() {
            Punctuated::parse_separated_nonempty(parser)?
        } else {
            Punctuated::new()
        };

        let gt = if lt.is_some() {
            Some(<_ as Parse>::parse(parser)?)
        } else {
            None
        };

        let where_clause = <_ as Parse>::parse(parser)?;

        Ok(Self {
            lt,
            gt,
            params,
            where_clause,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![<]>::peek(cursor) {
            cursor = <Token![<]>::skip(cursor)?;
            cursor = GenericParam::skip(cursor)?;

            while <Token![,]>::peek(cursor) {
                cursor = <Token![,]>::skip(cursor)?;
                cursor = GenericParam::skip(cursor)?;
            }

            cursor = <Token![>]>::skip(cursor)?;
        }

        Option::<WhereClause>::skip(cursor)
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

#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplGenerics<'a>(&'a Generics);

impl<'a> From<&'a Generics> for ImplGenerics<'a> {
    fn from(value: &'a Generics) -> Self {
        Self(value)
    }
}

impl Spanner for ImplGenerics<'_> {
    fn span(&self) -> Span {
        self.0.span()
    }
}

impl ToTokens for ImplGenerics<'_> {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lt.to_tokens(t);

        for pair in self.params.pairs() {
            match pair.value() {
                GenericParam::Lifetime(param) => param.to_tokens(t),
                GenericParam::Type(param) => {
                    param.attrs.to_tokens(t);
                    param.ident.to_tokens(t);
                    param.colon_punct.to_tokens(t);
                    param.bounds.to_tokens(t);
                }
                GenericParam::Const(param) => {
                    param.attrs.to_tokens(t);
                    param.const_keyword.to_tokens(t);
                    param.ident.to_tokens(t);
                    param.colon_punct.to_tokens(t);
                    param.ty.to_tokens(t);
                }
            };

            pair.punct().to_tokens(t);
        }

        self.gt.to_tokens(t);
    }
}

impl<'a> std::ops::Deref for ImplGenerics<'a> {
    type Target = Generics;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeGenerics<'a>(&'a Generics);

impl<'a> From<&'a Generics> for TypeGenerics<'a> {
    fn from(value: &'a Generics) -> Self {
        Self(value)
    }
}

impl Spanner for TypeGenerics<'_> {
    fn span(&self) -> Span {
        self.0.span()
    }
}

impl ToTokens for TypeGenerics<'_> {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lt.to_tokens(t);

        for pair in self.params.pairs() {
            match pair.value() {
                GenericParam::Lifetime(param) => param.lifetime.to_tokens(t),
                GenericParam::Type(param) => {
                    param.ident.to_tokens(t);
                }
                GenericParam::Const(param) => {
                    param.ident.to_tokens(t);
                }
            };

            pair.punct().to_tokens(t);
        }

        self.gt.to_tokens(t);
    }
}

impl<'a> std::ops::Deref for TypeGenerics<'a> {
    type Target = Generics;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
