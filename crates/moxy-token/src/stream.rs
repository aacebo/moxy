use std::str::FromStr;

use super::ToTokens;
use crate::lex::{Cursor, LexError, Scan};
use crate::span::DelimSpan;
use crate::{Span, Spanner, Token, TokenTree};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TokenStream(Vec<TokenTree>);

impl TokenStream {
    #[inline]
    pub fn new() -> Self {
        Self(vec![])
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.0.iter()
    }

    #[inline]
    pub fn first(&self) -> Span {
        self.0.first().map(|v| v.span()).unwrap_or_default()
    }

    #[inline]
    pub fn last(&self) -> Span {
        self.0.last().map(|v| v.span()).unwrap_or_default()
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.first().join(self.last())
    }

    #[inline]
    pub fn delim(&self) -> DelimSpan {
        DelimSpan::new(self.first(), self.last())
    }

    #[inline]
    pub fn extend_one(&mut self, token: TokenTree) {
        self.0.push(token);
    }

    #[inline]
    pub fn into_inner(self) -> Vec<TokenTree> {
        self.0
    }

    #[inline]
    pub fn to_vec(self) -> Vec<TokenTree> {
        self.0
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [TokenTree];

    fn deref(&self) -> &[TokenTree] {
        self.0.as_slice()
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<T: IntoIterator<Item = TokenTree>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<Self> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        Self(iter.into_iter().flat_map(|s| s.into_iter()).collect())
    }
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<TokenTree>> for TokenStream {
    #[inline]
    fn from(value: Vec<TokenTree>) -> Self {
        Self(value)
    }
}

impl From<&[TokenTree]> for TokenStream {
    #[inline]
    fn from(value: &[TokenTree]) -> Self {
        Self(value.to_vec())
    }
}

impl From<TokenStream> for Vec<TokenTree> {
    #[inline]
    fn from(value: TokenStream) -> Self {
        value.0
    }
}

impl Spanner for TokenStream {
    fn span(&self) -> Span {
        self.first().join(self.last())
    }
}

impl Scan for TokenStream {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let mut tokens = Vec::with_capacity(8);
        let mut c = cursor;

        loop {
            c = c.skip_whitespace();

            if c.is_empty() {
                break;
            }

            // Doc comment → `#[doc = "..."]` (outer) / `#![doc = "..."]` (inner).
            if let Some((next, inner, text)) = c.doc_comment() {
                push_doc_attr(&mut tokens, inner, &text, c.span_to(&next));
                c = next;
                continue;
            }

            // Check for closing delimiter — return to caller (Group::scan handles matching)
            if let Some(')' | ']' | '}') = c.first() {
                break;
            }

            // Try group first (opening delimiter)
            if let Ok((next, group)) = crate::Group::scan(c) {
                tokens.push(TokenTree::Group(group));
                c = next;
                continue;
            }

            if let Ok((next, lit)) = crate::Lit::scan(c) {
                tokens.push(TokenTree::Literal(lit));
                c = next;
                continue;
            }

            if let Ok((next, ident)) = crate::Ident::scan(c) {
                tokens.push(match crate::Keyword::from_str(ident.text(), ident.span()) {
                    Some(kw) if !ident.is_raw() => TokenTree::Keyword(kw),
                    _ => TokenTree::Ident(ident),
                });

                c = next;
                continue;
            }

            if let Ok((next, op)) = crate::Punctuation::scan(c) {
                tokens.push(TokenTree::Punct(op));
                c = next;
                continue;
            }

            return Err(c
                .error()
                .message(format!("unexpected character '{}'", c.first().unwrap_or('\0'))));
        }

        Ok((c, Self(tokens)))
    }
}

impl FromStr for TokenStream {
    type Err = LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s.to_owned())
    }
}

impl TokenStream {
    /// Build a token stream from an owned string while moving that allocation
    /// directly into the fallback source map.
    #[doc(hidden)]
    pub fn from_string(s: String) -> Result<Self, LexError> {
        use crate::source::SourceMap;

        SourceMap::with_mut(|sm| {
            let span = sm.push(s);
            let source = sm.find(span).expect("new source missing from source map");
            let cursor = Cursor::new(source.text(), span.byte_range().start as u32);
            let (rest, stream) = Self::scan(cursor)?;
            let rest = rest.skip_whitespace();

            if !rest.is_empty() {
                return rest.error().message("unexpected trailing input").into();
            }

            Ok(stream)
        })
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::Punctuation;

        let mut first = true;
        let mut prev_was_tick = false;

        for tt in self.0.iter() {
            if !first && !prev_was_tick {
                write!(f, " ")?;
            }

            write!(f, "{}", tt)?;
            first = false;
            // A `'` glues to the following token to form a lifetime (`'a`).
            prev_was_tick = matches!(tt, TokenTree::Punct(Punctuation::Quote(_)));
        }

        Ok(())
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut Self) {
        tokens.extend(self.clone());
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TokenStream {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(s)
    }
}

fn push_doc_attr(tokens: &mut Vec<TokenTree>, inner: bool, text: &str, span: Span) {
    use crate::{Delim, Group, Ident, Punctuation};

    tokens.push(TokenTree::Punct(Punctuation::Pound(<Token![#]>::new(span))));

    if inner {
        tokens.push(TokenTree::Punct(Punctuation::Not(<Token![!]>::new(span))));
    }

    let mut body = TokenStream::with_capacity(3);
    body.extend_one(TokenTree::Ident(Ident::new("doc").with_span(span)));
    body.extend_one(TokenTree::Punct(Punctuation::Eq(<Token![=]>::new(span))));
    body.extend_one(TokenTree::Literal(crate::Lit::Str(crate::LitStr::new(text, span))));

    tokens.push(TokenTree::Group(Group::new(Delim::Bracket, body)));
}
