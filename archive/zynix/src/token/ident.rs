use std::borrow::Cow;

use super::{ToTokens, TokenStream};
use crate::Span;
use crate::token::lex::{Cursor, LexError, Scan};

#[derive(Debug, Clone)]
pub struct Ident {
    name: Box<str>,
    span: Span,
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        Self { name: name.into(), span }
    }

    pub fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.name.as_ref())
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl From<proc_macro::Ident> for Ident {
    fn from(value: proc_macro::Ident) -> Self {
        Self::new(&value.to_string(), value.span().into())
    }
}

impl From<Ident> for proc_macro::Ident {
    fn from(value: Ident) -> Self {
        let span: proc_macro::Span = value.span.into();

        match value.name.strip_prefix("r#") {
            Some(name) => proc_macro::Ident::new_raw(name, span),
            None => proc_macro::Ident::new(&value.name, span),
        }
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
        let name = &cursor.rest()[..end.offset() as usize - cursor.offset() as usize];
        Ok((end, Self::new(name, span)))
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(crate::Token::Ident(self.clone()).into());
    }
}

impl crate::Parse for Ident {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Ident(v))) => Ok(v.clone()),
            _ => Err(crate::token::lex::LexError::new(stream.span())
                .message("expected Ident")
                .into()),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Ident {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name.serialize(s)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    mod serde {
        use std::str::FromStr;

        use crate::TokenStream;
        use crate::token::Ident;

        #[test]
        fn ident_serializes_as_string() {
            let ts = TokenStream::from_str("foo").unwrap();
            let id = ts.parse().parse::<Ident>().unwrap();
            assert_eq!(serde_json::to_value(&id).unwrap(), serde_json::json!("foo"));
        }
    }
}
