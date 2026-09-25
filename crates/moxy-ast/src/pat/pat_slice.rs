use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A slice pattern, e.g. `[a, b, c]`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatSlice {
    pub attrs: Attributes,
    pub elems: Delimited<Punctuated<Pattern, Token![,]>>,
}

impl Spanner for PatSlice {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl Parse for PatSlice {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor).unwrap_or(cursor).is_delimited(Delim::Bracket)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let (span, parser) = parser.parse_group_spanned(Delim::Bracket)?;

        Ok(Self {
            attrs,
            elems: Delimited::bracket(span, Punctuated::parse_terminated(&parser)?),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = Attributes::skip(cursor)?;
        let mut inner = cursor.descend(Delim::Bracket)?;

        while !inner.is_empty() {
            inner = Pattern::skip(inner)?;

            if inner.is_empty() {
                break;
            }

            inner = <Token![,]>::skip(inner)?;
        }

        Some(cursor.offset(1))
    }
}

impl ToTokens for PatSlice {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.elems.to_tokens(t);
    }
}
