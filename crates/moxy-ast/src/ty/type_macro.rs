use crate::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::MacroCall;

/// A macro invocation in type position (`path!(...)`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeMacro {
    pub mac: MacroCall,
}

impl Parse for TypeMacro {
    fn peek(cursor: Cursor<'_>) -> bool {
        MacroCall::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self { mac: parser.parse()? })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        MacroCall::skip(cursor)
    }
}

impl Spanner for TypeMacro {
    fn span(&self) -> Span {
        self.mac.span()
    }
}

impl ToTokens for TypeMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.mac.to_tokens(tokens);
    }
}
