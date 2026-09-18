use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A macro call pattern, e.g. `format!(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatMacro {
    pub attrs: Attributes,
    pub call: MacroCall,
}

impl Spanner for PatMacro {
    fn span(&self) -> Span {
        self.attrs.span().join(self.call.span())
    }
}

impl Parse for PatMacro {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        cursor.peek::<MacroCall>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            call: parser.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Attributes::skip(cursor)?.skip::<MacroCall>()
    }
}

impl ToTokens for PatMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.call.to_tokens(t);
    }
}
