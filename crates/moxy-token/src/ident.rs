use super::{ToTokens, TokenStream};
use crate::lex::{Cursor, LexError, Scan};
use crate::{Span, TokenTree};

/// Builds an [`Ident`] from one or more segments, lexing the result and panicking
/// on invalid identifier syntax.
///
/// Accepts a bare identifier, a string/expression that evaluates to a name, or a
/// comma-separated list of segments that are concatenated into a single identifier.
/// Bare identifiers are captured by name via `stringify!`; expressions are converted
/// with [`ToString`].
///
/// # Examples
///
/// ```
/// use moxy_token::ident;
///
/// // Bare identifier
/// let counter = ident!(counter);
/// assert_eq!(counter, "counter");
///
/// // Raw identifier
/// let kw = ident!(r#struct);
/// assert_eq!(kw.text(), "struct");
/// assert!(kw.is_raw());
///
/// // From a string or expression
/// let name = ident!("buffer");
/// assert_eq!(name, "buffer");
///
/// // Concatenate segments into one identifier
/// let field = ident!(get, "_", value);
/// assert_eq!(field, "get_value");
///
/// // Bare identifier segments are taken by name, not evaluated
/// let slot = ident!(slot, "_", n);
/// assert_eq!(slot, "slot_n");
/// ```
///
/// # Panics
///
/// Panics if the concatenated text is not a valid identifier.
#[macro_export]
macro_rules! ident {
    ($x:ident) => { $crate::Ident::lex(stringify!($x).to_string()).expect("invalid syntax") };
    ($x:expr)  => { $crate::Ident::lex($x).expect("invalid syntax") };
    ($head:ident, $($tail:tt)+) => {{
        let mut __ident = stringify!($head).to_string();
        ident!(@accum __ident, $($tail)+)
    }};
    ($head:expr, $($tail:tt)+) => {{
        let mut __ident = $head.to_string();
        ident!(@accum __ident, $($tail)+)
    }};
    (@accum $acc:ident, $next:ident, $($tail:tt)+) => {{
        $acc += stringify!($next);
        ident!(@accum $acc, $($tail)+)
    }};
    (@accum $acc:ident, $next:expr, $($tail:tt)+) => {{
        $acc += &$next.to_string();
        ident!(@accum $acc, $($tail)+)
    }};
    (@accum $acc:ident, $last:ident) => {{
        $acc += stringify!($last);
        $crate::Ident::lex($acc).expect("invalid syntax")
    }};
    (@accum $acc:ident, $last:expr) => {{
        $acc += &$last.to_string();
        $crate::Ident::lex($acc).expect("invalid syntax")
    }};
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    text: Box<str>,
    span: Span,
}

impl Ident {
    #[inline]
    pub fn new(text: impl std::fmt::Display) -> Self {
        Self {
            text: text.to_string().into_boxed_str(),
            span: Span::default(),
        }
    }

    #[inline]
    pub fn lex(input: impl std::fmt::Display) -> Result<Self, LexError> {
        std::str::FromStr::from_str(&input.to_string())
    }

    #[inline]
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
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
    pub fn to_lowercase(mut self) -> Self {
        self.text = self.text.to_lowercase().into_boxed_str();
        self
    }

    #[inline]
    pub fn to_uppercase(mut self) -> Self {
        self.text = self.text.to_uppercase().into_boxed_str();
        self
    }

    #[inline]
    pub fn to_token_tree(&self) -> TokenTree {
        TokenTree::Ident(self.clone())
    }

    #[inline]
    pub fn into_token_tree(self) -> TokenTree {
        TokenTree::Ident(self)
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
            return Ok((end, Self::new(name).with_span(span)));
        }

        let first = cursor.first().ok_or(cursor.error())?;

        if first != '_' && !unicode_ident::is_xid_start(first) {
            return cursor.error().into();
        }

        let end = cursor.advance(first.len_utf8()).skip_while(unicode_ident::is_xid_continue);
        let span = cursor.span_to(&end);
        let text = &cursor.rest()[..end.offset() as usize - cursor.offset() as usize];
        Ok((end, Self::new(text).with_span(span)))
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(TokenTree::Ident(self.clone()));
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
            Some(crate::TokenTree::Ident(v)) => Ok(v.clone()),
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
    fn ident_plain() {
        let id = ident!(foo);
        assert_eq!(id, "foo");
        assert!(!id.is_raw());
        assert_eq!(id, "foo");
    }

    #[test]
    fn literal_plain() {
        let id = ident!("foo");
        assert_eq!(id, "foo");
        assert!(!id.is_raw());
        assert_eq!(id, "foo");
    }

    #[test]
    fn ident_raw() {
        let id = ident!(r#fn);
        assert_eq!(id.text(), "fn");
        assert!(id.is_raw());
        assert_eq!(id, "r#fn");
    }

    #[test]
    fn literal_raw() {
        let id = ident!("r#fn");
        assert_eq!(id.text(), "fn");
        assert!(id.is_raw());
        assert_eq!(id, "r#fn");
    }

    #[test]
    fn ident_plus_str() {
        let id = ident!(a, "_", b);
        assert_eq!(id, "a_b");
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
