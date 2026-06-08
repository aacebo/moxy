use super::{ToTokens, TokenStream};
use crate::lex::{Cursor, LexError, Scan};
use crate::{Span, Token, TokenTree};

#[macro_export]
macro_rules! ident {
    ($fmt:expr) => {{
        $crate::Ident::lex($fmt).expect("invalid syntax")
    }};
    ($fmt:expr, $($token:tt)*) => {{
        $crate::Ident::lex(format!($fmt, $($token)*)).expect("invalid syntax")
    }};
}

#[derive(Debug, Clone)]
pub struct Ident {
    text: Box<str>,
    span: Span,
}

impl Ident {
    #[inline]
    pub fn new(text: impl std::fmt::Display, span: Span) -> Self {
        Self {
            text: text.to_string().into_boxed_str(),
            span,
        }
    }

    #[inline]
    pub fn lex(input: impl std::fmt::Display) -> Result<Self, LexError> {
        std::str::FromStr::from_str(&input.to_string())
    }

    #[inline]
    pub fn text(&self) -> &str {
        match self.text.strip_prefix("r#") {
            Some(rest) => rest,
            None => &self.text,
        }
    }

    #[inline]
    pub fn is_raw(&self) -> bool {
        self.text.starts_with("r#")
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    #[inline]
    pub fn to_token(&self) -> Token {
        Token::Ident(self.clone())
    }

    #[inline]
    pub fn into_token(self) -> Token {
        Token::Ident(self)
    }

    #[inline]
    pub fn to_token_tree(&self) -> TokenTree {
        TokenTree::Token(self.to_token())
    }

    #[inline]
    pub fn into_token_tree(self) -> TokenTree {
        TokenTree::Token(self.into_token())
    }
}

impl Scan for Ident {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        // Raw ident: r#ident
        if cursor.starts_with("r#") {
            let after = cursor.advance(2);
            let end = after.skip_while(unicode_ident::is_xid_continue);

            if end.offset() == after.offset() {
                return cursor.error().into();
            }

            let span = cursor.span_to(&end);
            let name = &cursor.rest()[..end.offset() as usize - cursor.offset() as usize];
            return Ok((end, Self::new(name, span)));
        }

        let first = cursor.first().ok_or(cursor.error())?;

        if first != '_' && !unicode_ident::is_xid_start(first) {
            return cursor.error().into();
        }

        let end = cursor.advance(first.len_utf8()).skip_while(unicode_ident::is_xid_continue);
        let span = cursor.span_to(&end);
        let text = &cursor.rest()[..end.offset() as usize - cursor.offset() as usize];
        Ok((end, Self::new(text, span)))
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(crate::Token::Ident(self.clone()).into());
    }
}

impl crate::Spanner for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl crate::Parse for Ident {
    fn parse(stream: &mut crate::parser::ParseStream) -> Result<Self, crate::parser::ParseError> {
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Ident(v))) => Ok(v.clone()),
            _ => Err(crate::lex::LexError::new(stream.span()).message("expected Ident").into()),
        }
    }
}

impl std::str::FromStr for Ident {
    type Err = LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cursor = Cursor::new(s, 0);
        let (_, ident) = Self::scan(cursor)?;
        Ok(ident)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Ident {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.text.serialize(s)
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.text.as_ref() == other
    }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.text.as_ref() == *other
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn text_plain() {
        let id = ident!("foo");
        assert_eq!(id, "foo");
        assert!(!id.is_raw());
        assert_eq!(id, "foo");
    }

    #[test]
    fn text_raw() {
        let id = ident!("r#fn");
        assert_eq!(id.text(), "fn");
        assert!(id.is_raw());
        assert_eq!(id, "r#fn");
    }

    #[cfg(feature = "serde")]
    mod serde {
        use std::str::FromStr;

        use crate::{Ident, TokenStream};

        #[test]
        fn ident_serializes_as_string() {
            let ts = TokenStream::from_str("foo").unwrap();
            let id = ts.parse().parse::<Ident>().unwrap();
            assert_eq!(serde_json::to_value(&id).unwrap(), serde_json::json!("foo"));
        }
    }
}
