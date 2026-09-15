use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A literal pattern, e.g. `42`, `'a'`, or `"hello"`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatLit {
    pub attrs: Attributes,
    pub lit: Lit,
}

impl Spanner for PatLit {
    fn span(&self) -> Span {
        self.attrs.span().join(self.expr.span())
    }
}

impl Parse for PatLit {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Lit>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            lit: parser.parse()?,
        })
    }
}

impl ToTokens for PatLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
