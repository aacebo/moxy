use std::str::FromStr;

use super::ToTokens;
use crate::lex::{Cursor, LexError, Scan};
use crate::span::DelimSpan;
use crate::{Span, Spanner, ToTokenStream, Token, TokenTree};

/// An ordered, owned sequence of Rust token trees.
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
    pub fn span(&self) -> Span {
        let first = self.0.first().map(|v| v.span()).unwrap_or_default();
        let last = self.0.last().map(|v| v.span()).unwrap_or_default();
        first.join(last)
    }

    #[inline]
    pub fn delim(&self) -> DelimSpan {
        let first = self.0.first().map(|v| v.span()).unwrap_or_default();
        let last = self.0.last().map(|v| v.span()).unwrap_or_default();
        DelimSpan::new(first, last)
    }

    #[inline]
    pub fn extend_one(&mut self, token: TokenTree) {
        self.0.push(token);
    }

    #[inline]
    pub fn into_inner(mut self) -> Vec<TokenTree> {
        std::mem::take(&mut self.0)
    }

    #[inline]
    pub fn to_vec(mut self) -> Vec<TokenTree> {
        std::mem::take(&mut self.0)
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

// Drops nested groups from a worklist rather than by recursion, so a stream
// nested any depth, as one converted from another library can be, drops on any
// stack
impl Drop for TokenStream {
    fn drop(&mut self) {
        let mut pending = Vec::new();

        for token in self.0.drain(..) {
            if let TokenTree::Group(group) = token {
                pending.push(group);
            }
        }

        while let Some(mut group) = pending.pop() {
            for token in group.tokens.0.drain(..) {
                if let TokenTree::Group(group) = token {
                    pending.push(group);
                }
            }
        }
    }
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;

    fn into_iter(mut self) -> Self::IntoIter {
        std::mem::take(&mut self.0).into_iter()
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
    fn from(mut value: TokenStream) -> Self {
        std::mem::take(&mut value.0)
    }
}

impl<T, E> From<Result<T, E>> for TokenStream
where
    T: ToTokens,
    E: ToTokens,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Err(err) => err.into_token_stream(),
            Ok(v) => v.into_token_stream(),
        }
    }
}

impl Spanner for TokenStream {
    fn span(&self) -> Span {
        let first = self.0.first().map(|v| v.span()).unwrap_or_default();
        let last = self.0.last().map(|v| v.span()).unwrap_or_default();
        first.join(last)
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut Self) {
        tokens.extend(self.clone());
    }
}

impl Scan for TokenStream {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let mut tokens = Vec::with_capacity(8);
        let mut c = cursor;

        loop {
            c = c.skip_whitespace();

            // `skip_whitespace` stops at a block comment it cannot close
            if c.starts_with("/*") && c.skip_comment().is_none() {
                return c.error().message("unterminated block comment").into();
            }

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

            // An opening delimiter starts a group, whose error, such as an
            // unclosed delimiter or one inside it, is the error of the stream
            if let Some('(' | '[' | '{') = c.first() {
                let (next, group) = crate::Group::scan(c)?;
                tokens.push(TokenTree::Group(group));
                c = next;
                continue;
            }

            // A digit or a quote always starts a literal, whose error is the
            // error of the stream
            let starts_literal = matches!(c.first(), Some('0'..='9' | '"'))
                || ["'\\", "b'", "b\"", "br\"", "br#", "c\"", "cr\"", "cr#", "r\"", "r#\"", "r##"]
                    .iter()
                    .any(|prefix| c.starts_with(prefix));

            // Any other word is read once, then told apart as a boolean, a
            // keyword, or an identifier
            if !starts_literal
                && !c.starts_with("r#")
                && let Some(next) = word_end(c)
            {
                let span = c.span_to(&next);
                let text = &c.rest()[..next.offset() as usize - c.offset() as usize];
                tokens.push(match text {
                    "true" | "false" => TokenTree::Literal(crate::LitBool::new(text == "true", span).into()),
                    _ => match crate::Keyword::from_str(text, span) {
                        Some(keyword) => TokenTree::Keyword(keyword),
                        None => TokenTree::Ident(crate::Ident::scanned(text, span)),
                    },
                });
                c = next;
                continue;
            }

            let literal = if starts_literal {
                Some(crate::Lit::scan(c)?)
            } else {
                crate::Lit::scan(c).ok()
            };

            if let Some((next, lit)) = literal {
                // After a field access dot, `0.1` is two tuple indices, as in
                // `x.0.1`, and `0.` a tuple index and a dot, as in `x.0. 1`, so
                // the tokens read back as they parse
                if let crate::Lit::Float(float) = &lit
                    && after_field_dot(&tokens)
                    && let Some((first, dot, second)) = float.split_tuple_index()
                {
                    tokens.push(TokenTree::Literal(first.into()));
                    tokens.push(TokenTree::Punct(dot.into()));

                    if let Some(second) = second {
                        tokens.push(TokenTree::Literal(second));
                    }
                } else {
                    tokens.push(TokenTree::Literal(lit));
                }

                c = next;
                continue;
            }

            if let Ok((next, keyword)) = crate::Keyword::scan(c) {
                tokens.push(TokenTree::Keyword(keyword));
                c = next;
                continue;
            }

            // `r#` and a name start a raw identifier, whose error is the error
            if c.starts_with("r#")
                && c.advance_by(2)
                    .first()
                    .is_some_and(|ch| ch == '_' || unicode_ident::is_xid_start(ch))
            {
                let (next, ident) = crate::Ident::scan(c)?;
                tokens.push(TokenTree::Ident(ident));
                c = next;
                continue;
            }

            if let Ok((next, ident)) = crate::Ident::scan(c) {
                tokens.push(TokenTree::Ident(ident));
                c = next;
                continue;
            }

            // A quote that starts no character literal starts a lifetime, as
            // rustc's lexer reads it; any other quote is an error there
            if c.first() == Some('\'')
                && !c
                    .advance_by(1)
                    .first()
                    .is_some_and(|ch| ch == '_' || unicode_ident::is_xid_start(ch))
            {
                let message = match c.advance_by(1).first() {
                    Some('\'') => "empty character literal",
                    Some('0'..='9') => "lifetimes cannot start with a number",
                    _ => "unterminated character literal",
                };
                return c.error().message(message).into();
            }

            if let Ok((next, punct)) = crate::Punct::scan(c) {
                tokens.push(TokenTree::Punct(punct));
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
    ///
    /// The string is lexed as a source file is: a leading byte order mark and
    /// a shebang line are skipped (as `Cursor::skip_source_start` does), so
    /// `"#!a b"` gives no tokens, while `"#![a]"` is an inner attribute.
    pub fn from_string(s: String) -> Result<Self, LexError> {
        use crate::source::SourceMap;

        SourceMap::with_mut(|sm| {
            let span = sm.push(s);
            let source = sm.find(span).expect("new source missing from source map");
            let cursor = Cursor::new(source.text(), span.byte_range().start as u32).skip_source_start();
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
        let mut first = true;
        let mut prev_was_joint = false;

        for tt in self.0.iter() {
            if !first && !prev_was_joint {
                write!(f, " ")?;
            }

            write!(f, "{}", tt)?;
            first = false;
            prev_was_joint = matches!(tt, TokenTree::Punct(punct) if punct.spacing().is_joint());
        }

        Ok(())
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

// The end of an identifier or keyword that starts at the cursor
fn word_end(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    let first = cursor.first()?;

    (first == '_' || unicode_ident::is_xid_start(first))
        .then(|| cursor.advance_by(first.len_utf8()).skip_while(unicode_ident::is_xid_continue))
}

// Whether the last token is a single `.`, not the end of `..`
fn after_field_dot(tokens: &[TokenTree]) -> bool {
    match tokens {
        [
            ..,
            TokenTree::Punct(crate::Punct::Dot(first)),
            TokenTree::Punct(crate::Punct::Dot(_)),
        ] => first.spacing() != crate::Spacing::Joint,
        [.., TokenTree::Punct(crate::Punct::Dot(_))] => true,
        _ => false,
    }
}

fn push_doc_attr(tokens: &mut Vec<TokenTree>, inner: bool, text: &str, span: Span) {
    use crate::{Delim, Group, Ident, Punct};

    tokens.push(TokenTree::Punct(Punct::Pound(<Token![#]>::new(span))));

    if inner {
        tokens.push(TokenTree::Punct(Punct::Not(<Token![!]>::new(span))));
    }

    let mut body = TokenStream::with_capacity(3);
    body.extend_one(TokenTree::Ident(Ident::new("doc").with_span(span)));
    body.extend_one(TokenTree::Punct(Punct::Eq(<Token![=]>::new(span))));
    body.extend_one(TokenTree::Literal(crate::Lit::Str(crate::LitStr::new(text, span))));

    tokens.push(TokenTree::Group(Group::new(Delim::Bracket, body)));
}
