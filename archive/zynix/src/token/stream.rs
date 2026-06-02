use std::str::FromStr;

use super::ToTokens;
use crate::parse::{ParseError, ParseStream};
use crate::span::DelimSpan;
use crate::token::lex::{Cursor, LexError, Scan};
use crate::{Span, TokenTree};

#[derive(Debug, Default, Clone)]
pub struct TokenStream(Vec<TokenTree>);

impl TokenStream {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.0.iter()
    }

    pub fn first(&self) -> Span {
        self.0.first().map(|v| v.span()).unwrap_or_default()
    }

    pub fn last(&self) -> Span {
        self.0.last().map(|v| v.span()).unwrap_or_default()
    }

    pub fn span(&self) -> Span {
        self.first().join(self.last())
    }

    pub fn delim(&self) -> DelimSpan {
        DelimSpan::new(self.first(), self.last())
    }

    pub fn extend_one(&mut self, token: TokenTree) {
        self.0.push(token);
    }

    pub fn parse(&self) -> ParseStream<'_> {
        ParseStream::new(self)
    }

    pub fn into_inner(self) -> Vec<TokenTree> {
        self.0
    }

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
    fn from(value: Vec<TokenTree>) -> Self {
        Self(value)
    }
}

impl From<&[TokenTree]> for TokenStream {
    fn from(value: &[TokenTree]) -> Self {
        Self(value.to_vec())
    }
}

impl From<TokenStream> for Vec<TokenTree> {
    fn from(value: TokenStream) -> Self {
        value.0
    }
}

impl From<proc_macro::TokenStream> for TokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        let mut out = Self::new();
        value.to_tokens(&mut out);
        out
    }
}

impl From<TokenStream> for proc_macro::TokenStream {
    fn from(value: TokenStream) -> Self {
        let mut out = proc_macro::TokenStream::new();
        for t in value.0 {
            t.to_tokens(&mut out);
        }
        out
    }
}

impl Scan for TokenStream {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let mut tokens = Vec::new();
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
            if let Ok((next, group)) = crate::token::Group::scan(c) {
                tokens.push(crate::TokenTree::Group(group));
                c = next;
                continue;
            }

            if let Ok((next, lit)) = crate::token::Literal::scan(c) {
                tokens.push(crate::Token::Literal(lit).into());
                c = next;
                continue;
            }

            if let Ok((next, ident)) = crate::token::Ident::scan(c) {
                let token = match crate::token::Keyword::from_str(ident.name().as_ref(), ident.span()) {
                    Some(kw) => crate::Token::Keyword(kw),
                    None => crate::Token::Ident(ident),
                };
                tokens.push(token.into());
                c = next;
                continue;
            }

            if let Ok((next, op)) = <crate::token::Punctuation as Scan>::scan(c) {
                tokens.push(crate::Token::Punct(op).into());
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
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::source::SourceMap;

        let span = SourceMap::with_mut(|sm| sm.push(s));
        let cursor = Cursor::new(s, span.byte_range().start as u32);
        let (rest, stream) = Self::scan(cursor)?;
        let rest = rest.skip_whitespace();

        if !rest.is_empty() {
            return Err(rest.error().message("unexpected trailing input").into());
        }

        Ok(stream)
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::token::{Punctuation, Token};

        let mut first = true;
        let mut prev_was_tick = false;

        for tt in self.0.iter() {
            if !first && !prev_was_tick {
                write!(f, " ")?;
            }

            write!(f, "{}", tt)?;
            first = false;
            // A `'` glues to the following token to form a lifetime (`'a`).
            prev_was_tick = matches!(tt, TokenTree::Token(Token::Punct(Punctuation::Quote(_))));
        }

        Ok(())
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
    use crate::token::punct::{Eq, Not, Pound};
    use crate::token::{Delim, Group, Ident, Literal, Punctuation};

    tokens.push(crate::Token::Punct(Punctuation::Pound(Pound::new(span))).into());

    if inner {
        tokens.push(crate::Token::Punct(Punctuation::Not(Not::new(span))).into());
    }

    let mut body = TokenStream::new();
    body.extend_one(crate::Token::Ident(Ident::new("doc", span)).into());
    body.extend_one(crate::Token::Punct(Punctuation::Eq(Eq::new(span))).into());
    body.extend_one(crate::Token::Literal(Literal::string(text)).into());

    tokens.push(TokenTree::Group(Group::new(Delim::Bracket, body)));
}

#[cfg(test)]
mod tests {
    mod display {
        use std::str::FromStr;

        use crate::TokenStream;

        fn render(src: &str) -> String {
            TokenStream::from_str(src).unwrap().to_string()
        }

        #[test]
        fn separates_word_tokens() {
            // adjacent idents/keywords must not merge into invalid source
            assert!(!render("let x").contains("letx"));
            assert_eq!(render("let x"), "let x");
            assert_eq!(render("a + b"), "a + b");
        }

        #[test]
        fn keeps_joint_punct_tight() {
            // multi-char puncts stay glued (Joint spacing)
            assert_eq!(render("x == y"), "x == y");
            assert_eq!(render("a -> b"), "a -> b");
            assert_eq!(render("a :: b"), "a :: b");
            assert_eq!(render("..="), "..=");
        }

        #[test]
        fn renders_group_with_delimiters() {
            // delimiters are preserved (proc_macro2-style spacing inside)
            assert_eq!(render("(a + b)"), "(a + b)");
            assert_eq!(render("[a, b]"), "[a , b]");
            assert_eq!(render("{ x }"), "{x}");
        }

        #[test]
        fn group_follows_word_with_space() {
            // proc_macro2-style: groups are word tokens
            assert_eq!(render("foo(a, b)"), "foo (a , b)");
        }

        #[test]
        fn roundtrips_to_reparseable_source() {
            for src in ["let x = (a + b) * c;", "x == y", "a::b::c", "foo(1, 2)"] {
                let rendered = render(src);
                // must re-parse without error
                assert!(
                    TokenStream::from_str(&rendered).is_ok(),
                    "failed to reparse {:?} from {:?}",
                    rendered,
                    src
                );
                // and rendering is stable (idempotent)
                assert_eq!(render(&rendered), rendered);
            }
        }
    }

    mod lex {
        use std::str::FromStr;

        use crate::TokenStream;
        use crate::token::{Punctuation, Token, TokenTree};

        fn trees(src: &str) -> Vec<TokenTree> {
            TokenStream::from_str(src).unwrap().into_iter().collect()
        }

        #[test]
        fn doc_comment_becomes_attr() {
            // `/// x` → `# [doc = "x"]`; plain `//` is still skipped.
            let t = trees("/// hello\nfn f() {}");
            assert!(matches!(t[0], TokenTree::Token(Token::Punct(Punctuation::Pound(_)))));
            assert!(matches!(t[1], TokenTree::Group(_)));
            // inner doc `//!`
            let inner = trees("//! crate doc\n");
            assert!(matches!(inner[1], TokenTree::Token(Token::Punct(Punctuation::Not(_)))));
            // plain comment still skipped
            assert_eq!(trees("// plain\nx").len(), 1);
            // block doc
            assert!(matches!(
                trees("/** b */ x")[0],
                TokenTree::Token(Token::Punct(Punctuation::Pound(_)))
            ));
        }

        #[test]
        fn lifetime_lexes_as_quote_then_ident() {
            let t = trees("&'a str");
            assert!(matches!(t[1], TokenTree::Token(Token::Punct(Punctuation::Quote(_)))));
            assert!(matches!(t[2], TokenTree::Token(Token::Ident(_))));
        }

        #[test]
        fn char_literal_still_lexes_as_literal() {
            let t = trees("'c'");
            assert_eq!(t.len(), 1);
            assert!(matches!(t[0], TokenTree::Token(Token::Literal(_))));
        }
    }

    #[cfg(feature = "serde")]
    mod serde {
        use std::str::FromStr;

        use crate::TokenStream;

        #[test]
        fn serializes_as_array_of_token_strings() {
            let ts = TokenStream::from_str("a + 1").unwrap();
            let json = serde_json::to_value(&ts).unwrap();
            assert_eq!(json, serde_json::json!(["a", "+", "1"]));
        }

        #[test]
        fn nested_group_serializes_as_nested_array() {
            let ts = TokenStream::from_str("a + (b * 2)").unwrap();
            let json = serde_json::to_value(&ts).unwrap();
            assert_eq!(
                json,
                serde_json::json!([
                    "a",
                    "+",
                    { "delim": "paren", "tokens": ["b", "*", "2"] }
                ])
            );
        }
    }
}
