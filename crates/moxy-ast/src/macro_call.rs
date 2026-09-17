use moxy_token::{Delim, Group, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A macro invocation (`path!(...)`, `path![...]`, `path!{...}`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MacroCall {
    pub path: Path,
    pub bang: Token![!],
    pub body: Group,
    pub semi: Option<Token![;]>,
}

impl MacroCall {
    /// The delimiter of the macro body (`(`, `[`, or `{`).
    pub fn delim(&self) -> Delim {
        self.body.delim()
    }

    /// The token parser inside the macro body delimiters.
    pub fn tokens(&self) -> &TokenStream {
        self.body.stream()
    }
}

impl Parse for MacroCall {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = cursor.skip::<Path>() else {
            return false;
        };

        cursor.peek::<Token![!]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            path: parser.parse()?,
            bang: parser.parse()?,
            body: parser.parse()?,
            semi: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Path>()?;
        cursor = cursor.skip::<Token![!]>()?;
        cursor = cursor.skip::<Group>()?;
        cursor.skip::<Option<Token![;]>>()
    }
}

impl Spanner for MacroCall {
    fn span(&self) -> Span {
        self.path.span().join(self.body.span().into())
    }
}

impl ToTokens for MacroCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.bang.to_tokens(tokens);
        self.body.to_tokens(tokens);
        self.semi.to_tokens(tokens);
    }
}
