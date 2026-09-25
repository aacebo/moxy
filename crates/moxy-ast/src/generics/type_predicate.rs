use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A type predicate in a `where` clause (`T: Bound`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePredicate {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: Type,
    pub colon_punct: Token![:],
    pub bounds: Punctuated<TypeBound, Token![+]>,
}

impl Parse for TypePredicate {
    fn peek(cursor: Cursor<'_>) -> bool {
        BoundLifetimes::peek(cursor) || Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            lifetimes: <_ as Parse>::parse(parser)?,
            bounded_ty: <_ as Parse>::parse(parser)?,
            colon_punct: <_ as Parse>::parse(parser)?,
            bounds: TypeBound::parse_bounds(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<BoundLifetimes>::skip(cursor)?;
        cursor = Type::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = TypeBound::skip(cursor)?;

        while <Token![+]>::peek(cursor) {
            cursor = <Token![+]>::skip(cursor)?;
            cursor = TypeBound::skip(cursor)?;
        }

        Some(cursor)
    }
}

impl Spanner for TypePredicate {
    fn span(&self) -> Span {
        let start = if let Some(l) = &self.lifetimes {
            l.span()
        } else {
            self.bounded_ty.span()
        };

        let end = self
            .bounds
            .last()
            .map(|b| b.span())
            .unwrap_or_else(|| self.colon_punct.span());
        start.join(end)
    }
}

impl ToTokens for TypePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lifetimes.to_tokens(t);
        self.bounded_ty.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
