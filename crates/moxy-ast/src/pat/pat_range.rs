use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A range pattern, e.g. `0..=255` or `'a'..'z'`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatRange {
    pub attrs: Attributes,
    pub start: Option<Expr>,
    pub limits: RangeLimits,
    pub end: Option<Expr>,
}

impl Spanner for PatRange {
    fn span(&self) -> Span {
        let end = if let Some(e) = &self.end {
            e.span()
        } else {
            self.limits.span()
        };

        self.attrs.span().join(end)
    }
}

impl Parse for PatRange {
    fn peek(cursor: Cursor<'_>) -> bool {
        Self::skip(cursor).is_some()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let start = if parser.peek::<RangeLimits>() {
            None
        } else {
            Some(expr::parse_unary(parser, Attributes::default())?)
        };
        let limits = parser.parse()?;
        let end = if parser.is_empty() || parser.peek::<Token![,]>() || parser.peek::<Token![|]>() || parser.peek::<Token![:]>() {
            None
        } else {
            Some(expr::parse_unary(parser, Attributes::default())?)
        };

        if start.is_none() && end.is_none() && matches!(limits, RangeLimits::HalfOpen(_)) {
            return parser.error("expected range pattern").into();
        }

        Ok(Self {
            attrs,
            start,
            limits,
            end,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        let has_start = !cursor.peek::<RangeLimits>();

        if has_start {
            cursor = expr::skip_pattern_bound(cursor)?;
        }

        let closed = cursor.peek::<Token![..=]>();
        cursor = cursor.skip::<RangeLimits>()?;

        if cursor.is_empty() || cursor.peek::<Token![,]>() || cursor.peek::<Token![|]>() || cursor.peek::<Token![:]>() {
            return (has_start || closed).then_some(cursor);
        }

        expr::skip_pattern_bound(cursor)
    }
}

impl ToTokens for PatRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.start.to_tokens(t);
        self.limits.to_tokens(t);
        self.end.to_tokens(t);
    }
}
