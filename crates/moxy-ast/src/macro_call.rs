use moxy_token::{Delim, Group, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A macro invocation (`path!(...)`, `path![...]`, `path!{...}`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MacroCall {
    pub path: Path,
    pub bang: Token![!],
    pub body: Group,
}

impl MacroCall {
    /// The delimiter of the macro body (`(`, `[`, or `{`).
    pub fn delim(&self) -> Delim {
        self.body.delim
    }

    /// The token parser inside the macro body delimiters.
    pub fn tokens(&self) -> &TokenStream {
        &self.body.tokens
    }
}

impl Parse for MacroCall {
    fn peek(cursor: Cursor<'_>) -> bool {
        // A strict keyword such as `if` or `while` followed by `!` starts an
        // expression (`if !ready {}`), never a macro call. Only the path keywords
        // and the weak keywords, which are valid identifiers, can name a macro.
        let first = Option::<Token![::]>::skip(cursor).unwrap_or(cursor);

        if let Some(token) = first.curr()
            && token.is_keyword()
            && !(token.is_keyword_crate()
                || token.is_keyword_self_value()
                || token.is_keyword_self_type()
                || token.is_keyword_super()
                || token.is_keyword_auto()
                || token.is_keyword_default()
                || token.is_keyword_macro_rules()
                || token.is_keyword_raw()
                || token.is_keyword_safe()
                || token.is_keyword_union())
        {
            return false;
        }

        let Some(cursor) = Path::skip(cursor) else {
            return false;
        };

        // `macro_rules! name { .. }` defines a macro rather than calling one
        <Token![!]>::skip(cursor).is_some_and(|cursor| matches!(cursor.curr(), Some(TokenTree::Group(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            path: <_ as Parse>::parse(parser)?,
            bang: <_ as Parse>::parse(parser)?,
            body: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Path::skip(cursor)?;
        cursor = <Token![!]>::skip(cursor)?;
        Group::skip(cursor)
    }
}

impl Spanner for MacroCall {
    fn span(&self) -> Span {
        self.path.span().join(self.body.span())
    }
}

impl ToTokens for MacroCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.bang.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}
