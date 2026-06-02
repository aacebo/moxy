pub mod delim;
mod group;
mod ident;
pub mod keyword;
pub mod lex;
mod literal;
pub mod punct;
mod spacing;
mod stream;

pub use delim::*;
pub use group::*;
pub use ident::*;
pub use keyword::*;
pub use lex::{LexError, Scan};
pub use literal::*;
pub use punct::*;
pub use spacing::*;
pub use stream::*;

use crate::Span;

pub trait ToTokens<T = TokenStream> {
    fn to_tokens(&self, tokens: &mut T);
}

pub trait ToTokenStream: ToTokens<TokenStream> {
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}

impl<X: ToTokens<TokenStream> + ?Sized> ToTokenStream for X {}

impl<T: ToTokens> ToTokens for ::std::boxed::Box<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(v) = self {
            v.to_tokens(tokens);
        }
    }
}

impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for v in self {
            v.to_tokens(tokens);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punctuation),
    Literal(Literal),
}

impl Token {
    pub fn span(&self) -> Span {
        match self {
            Self::Ident(v) => v.span(),
            Self::Keyword(v) => v.span(),
            Self::Punct(v) => v.span(),
            Self::Literal(v) => v.span(),
        }
    }

    pub fn name(&self) -> Option<std::borrow::Cow<'_, str>> {
        match self {
            Self::Ident(v) => Some(v.name()),
            Self::Keyword(v) => Some(std::borrow::Cow::Borrowed(v.as_str())),
            _ => None,
        }
    }
}

impl From<Ident> for Token {
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl From<Keyword> for Token {
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}

impl From<Punctuation> for Token {
    fn from(value: Punctuation) -> Self {
        Self::Punct(value)
    }
}

impl From<Literal> for Token {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(v) => write!(f, "{}", v),
            Self::Keyword(v) => write!(f, "{}", v),
            Self::Punct(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
        }
    }
}

impl ToTokens for Token {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(TokenTree::from(self.clone()));
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Token {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Ident(v) => v.serialize(s),
            Self::Keyword(v) => v.serialize(s),
            Self::Punct(v) => v.serialize(s),
            Self::Literal(v) => v.serialize(s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenTree {
    Token(Token),
    Group(Group),
}

impl TokenTree {
    pub fn span(&self) -> Span {
        match self {
            Self::Token(v) => v.span(),
            Self::Group(v) => v.span().into(),
        }
    }

    pub fn name(&self) -> Option<std::borrow::Cow<'_, str>> {
        match self {
            Self::Token(t) => t.name(),
            Self::Group(_) => None,
        }
    }

    pub fn delim(&self) -> Option<Delim> {
        match self {
            Self::Group(g) => Some(g.delim()),
            Self::Token(_) => None,
        }
    }
}

impl From<Token> for TokenTree {
    fn from(value: Token) -> Self {
        Self::Token(value)
    }
}

impl From<Ident> for TokenTree {
    fn from(value: Ident) -> Self {
        Self::Token(Token::from(value))
    }
}

impl From<Keyword> for TokenTree {
    fn from(value: Keyword) -> Self {
        Self::Token(Token::from(value))
    }
}

impl From<Punctuation> for TokenTree {
    fn from(value: Punctuation) -> Self {
        Self::Token(Token::from(value))
    }
}

impl From<Literal> for TokenTree {
    fn from(value: Literal) -> Self {
        Self::Token(Token::from(value))
    }
}

impl From<Group> for TokenTree {
    fn from(value: Group) -> Self {
        Self::Group(value)
    }
}

impl ToTokens<TokenStream> for proc_macro::TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            proc_macro::TokenTree::Ident(v) => {
                let token = match Keyword::from_str(&v.to_string(), v.span().into()) {
                    Some(kw) => Token::Keyword(kw),
                    None => Token::Ident(v.clone().into()),
                };
                tokens.extend_one(token.into())
            }
            proc_macro::TokenTree::Literal(v) => tokens.extend_one(Token::Literal(v.clone().into()).into()),
            proc_macro::TokenTree::Group(v) => tokens.extend_one(TokenTree::Group(v.clone().into())),
            proc_macro::TokenTree::Punct(p) => scan_puncts(&p.to_string(), tokens),
        }
    }
}

impl ToTokens<TokenStream> for proc_macro::TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut punct_buf = String::new();

        for tt in self.clone() {
            match tt {
                proc_macro::TokenTree::Punct(p) => punct_buf.push(p.as_char()),
                other => {
                    if !punct_buf.is_empty() {
                        scan_puncts(&punct_buf, tokens);
                        punct_buf.clear();
                    }
                    other.to_tokens(tokens);
                }
            }
        }

        if !punct_buf.is_empty() {
            scan_puncts(&punct_buf, tokens);
        }
    }
}

impl ToTokens<proc_macro::TokenStream> for TokenTree {
    fn to_tokens(&self, out: &mut proc_macro::TokenStream) {
        match self {
            TokenTree::Group(g) => out.extend_one(proc_macro::TokenTree::Group(g.clone().into())),
            TokenTree::Token(Token::Ident(v)) => out.extend_one(proc_macro::TokenTree::Ident(v.clone().into())),
            TokenTree::Token(Token::Keyword(kw)) => {
                let id = proc_macro::Ident::new(kw.as_str(), kw.span().into());
                out.extend_one(proc_macro::TokenTree::Ident(id))
            }
            TokenTree::Token(Token::Literal(v)) => out.extend_one(proc_macro::TokenTree::Literal(v.clone().into())),
            TokenTree::Token(Token::Punct(op)) => {
                let text = op.as_str();
                let span: proc_macro::Span = op.span().into();
                let last = text.chars().count() - 1;
                let joint_last = text == "'";

                for (i, ch) in text.chars().enumerate() {
                    let spacing = if i == last && !joint_last {
                        proc_macro::Spacing::Alone
                    } else {
                        proc_macro::Spacing::Joint
                    };
                    let mut p = proc_macro::Punct::new(ch, spacing);
                    p.set_span(span);
                    out.extend_one(proc_macro::TokenTree::Punct(p));
                }
            }
        }
    }
}

impl ToTokens<proc_macro::TokenStream> for TokenStream {
    fn to_tokens(&self, out: &mut proc_macro::TokenStream) {
        for t in self.iter() {
            t.to_tokens(out);
        }
    }
}

impl IntoIterator for TokenTree {
    type Item = TokenTree;
    type IntoIter = std::iter::Once<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl std::fmt::Display for TokenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

impl ToTokens for TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone());
    }
}

impl ToTokens for &str {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use std::str::FromStr;

        if let Ok(ts) = TokenStream::from_str(self) {
            ts.to_tokens(tokens);
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TokenTree {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Token(v) => v.serialize(s),
            Self::Group(v) => v.serialize(s),
        }
    }
}

fn scan_puncts(s: &str, tokens: &mut TokenStream) {
    use crate::source::SourceMap;
    use crate::token::lex::{Cursor, Scan};

    let span = SourceMap::with_mut(|sm| sm.push(s));
    let mut cursor = Cursor::new(s, span.byte_range().start as u32);

    while !cursor.is_empty() {
        match <Punctuation as Scan>::scan(cursor) {
            Ok((next, op)) => {
                tokens.extend_one(Token::Punct(op).into());
                cursor = next;
            }
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceMap;
    use crate::span::fallback as span_fb;

    fn span(start: u32, end: u32) -> Span {
        SourceMap::with_mut(|sm| {
            if sm.is_empty() {
                sm.push("0123456789abcdef");
            }
        });
        Span::Fallback(span_fb::Span::new(start, end))
    }

    // --- Ident ---

    #[test]
    fn ident_new_and_name() {
        let id = Ident::new("foo", Span::default());
        assert_eq!(id.name().as_ref(), "foo");
    }

    #[test]
    fn ident_span_and_set_span() {
        let mut id = Ident::new("x", span(0, 1));
        assert_eq!(id.span().start().index(), 0);
        id.set_span(span(5, 6));
        assert_eq!(id.span().start().index(), 5);
    }

    #[test]
    fn ident_display() {
        let id = Ident::new("hello", Span::default());
        assert_eq!(format!("{}", id), "hello");
    }

    // --- Punct (operators) ---

    #[test]
    fn op_as_str() {
        use crate::token::punct::{Plus, Semi};
        assert_eq!(Plus::default().as_str(), "+");
        assert_eq!(Semi::default().as_str(), ";");
    }

    #[test]
    fn op_display() {
        use crate::token::punct::{EqEq, Semi};
        assert_eq!(format!("{}", Semi::default()), ";");
        assert_eq!(format!("{}", EqEq::default()), "==");
    }

    #[test]
    fn op_is_a_token() {
        use crate::token::punct::Plus;
        let t: Token = Punctuation::from(Plus::default()).into();
        assert!(matches!(t, Token::Punct(Punctuation::Plus(_))));
    }

    // --- Literal ---

    #[test]
    fn literal_string() {
        let lit = Literal::string("hello");
        let s = format!("{}", lit);
        assert!(s.contains("hello"));
    }

    #[test]
    fn literal_integer() {
        let lit = Literal::u32_suffixed(42);
        let s = format!("{}", lit);
        assert!(s.contains("42"));
    }

    // --- Group ---

    #[test]
    fn group_new_and_delim() {
        let g = Group::new(Delim::Paren, TokenStream::new());
        assert_eq!(g.delim(), Delim::Paren);
    }

    // --- TokenStream ---

    #[test]
    fn token_stream_new_is_empty() {
        let ts = TokenStream::new();
        assert!(ts.is_empty());
    }

    #[test]
    fn token_stream_extend_one() {
        let mut ts = TokenStream::new();
        ts.extend_one(Ident::new("a", Span::default()).into());
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn token_stream_iter() {
        let mut ts = TokenStream::new();
        ts.extend_one(Ident::new("x", Span::default()).into());
        ts.extend_one(Punctuation::from(crate::token::punct::Plus::default()).into());
        let count = ts.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn token_stream_from_str() {
        use std::str::FromStr;
        let ts = TokenStream::from_str("fn main() {}").unwrap();
        assert!(!ts.is_empty());
    }

    // --- Token enum ---

    #[test]
    fn token_from_ident() {
        let t: Token = Ident::new("foo", Span::default()).into();
        assert!(matches!(t, Token::Ident(_)));
    }

    #[test]
    fn token_from_punct() {
        let t: Token = Punctuation::from(crate::token::punct::Plus::default()).into();
        assert!(matches!(t, Token::Punct(_)));
    }

    #[test]
    fn token_from_literal() {
        let t: Token = Literal::string("x").into();
        assert!(matches!(t, Token::Literal(_)));
    }

    #[test]
    fn token_tree_from_group() {
        let t: TokenTree = Group::new(Delim::Paren, TokenStream::new()).into();
        assert!(matches!(t, TokenTree::Group(_)));
    }

    #[test]
    fn token_span() {
        let t: Token = Ident::new("x", span(3, 4)).into();
        assert_eq!(t.span().start().index(), 3);
    }

    #[test]
    fn token_display() {
        let t: Token = Ident::new("hello", Span::default()).into();
        assert_eq!(format!("{}", t), "hello");
    }

    #[cfg(feature = "serde")]
    mod serde {
        use std::str::FromStr;

        use crate::TokenStream;

        #[test]
        fn token_serializes_as_string() {
            let ts = TokenStream::from_str("foo").unwrap();
            let tree = ts.into_iter().next().unwrap();
            assert_eq!(serde_json::to_value(&tree).unwrap(), serde_json::json!("foo"));
        }

        #[test]
        fn token_tree_group_serializes_as_object() {
            let ts = TokenStream::from_str("(x)").unwrap();
            let tree = ts.into_iter().next().unwrap();
            assert_eq!(
                serde_json::to_value(&tree).unwrap(),
                serde_json::json!({ "delim": "paren", "tokens": ["x"] })
            );
        }
    }
}
