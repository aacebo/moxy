use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A slice pattern, e.g. `[a, b, c]`.
#[derive(Debug, Clone, PartialEq, Eq)]
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
        cursor
            .descend(Delim::Bracket)
            .map(|c| c.peek::<Pattern>())
            .unwrap_or_default()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let (span, parser) = parser.parse_group_spanned(Delim::Bracket)?;

        Ok(Self {
            attrs,
            elems: Delimited::bracket(span, Punctuated::parse_separated_nonempty(&parser)?),
        })
    }
}

impl ToTokens for PatSlice {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.elems.to_tokens(t);
    }
}
