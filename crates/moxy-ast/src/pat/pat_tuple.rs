use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A tuple pattern, e.g. `(a, b, c)`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTuple {
    pub attrs: Attributes,
    pub elems: Delimited<Punctuated<Pattern, Token![,]>>,
}

impl Spanner for PatTuple {
    fn span(&self) -> Span {
        self.elems.span()
    }
}

impl Parse for PatTuple {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.descend(Delim::Paren).map(|c| c.peek::<Pattern>()).unwrap_or_default()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;

        Ok(Self {
            attrs,
            elems: Delimited::paren(span, Punctuated::parse_separated_nonempty(&parser)?),
        })
    }
}

impl ToTokens for PatTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.elems.to_tokens(t);
    }
}
