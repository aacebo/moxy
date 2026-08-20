use crate::lex::{Cursor, Scan};
use crate::{Delim, Group, Ident, Keyword, Lit, Punctuation, Span, Spanner, ToTokens, TokenStream};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum TokenTree {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punctuation),
    Literal(Lit),
    Group(Group),
}

impl TokenTree {
    pub fn span(&self) -> Span {
        match self {
            Self::Ident(v) => v.span(),
            Self::Keyword(v) => v.span(),
            Self::Punct(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::Group(v) => v.span().into(),
        }
    }

    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Ident(v) => Some(v.text()),
            Self::Keyword(v) => Some(v.as_str()),
            _ => None,
        }
    }

    pub fn delim(&self) -> Option<Delim> {
        match self {
            Self::Group(g) => Some(g.delim()),
            _ => None,
        }
    }

    #[inline]
    pub fn is_token(&self) -> bool {
        !matches!(self, Self::Group(_))
    }

    #[inline]
    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    #[inline]
    pub fn as_group(&self) -> Option<&Group> {
        match self {
            Self::Group(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(_))
    }

    #[inline]
    pub fn as_ident(&self) -> Option<&Ident> {
        match self {
            Self::Ident(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_))
    }

    #[inline]
    pub fn as_literal(&self) -> Option<&Lit> {
        match self {
            Self::Literal(v) => Some(v),
            _ => None,
        }
    }
}

impl From<Ident> for TokenTree {
    #[inline]
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl From<Keyword> for TokenTree {
    #[inline]
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}

impl From<Punctuation> for TokenTree {
    #[inline]
    fn from(value: Punctuation) -> Self {
        Self::Punct(value)
    }
}

impl From<Lit> for TokenTree {
    #[inline]
    fn from(value: Lit) -> Self {
        Self::Literal(value)
    }
}

impl From<Group> for TokenTree {
    #[inline]
    fn from(value: Group) -> Self {
        Self::Group(value)
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
            Self::Ident(v) => write!(f, "{}", v),
            Self::Keyword(v) => write!(f, "{}", v),
            Self::Punct(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

impl ToTokens for TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone());
    }
}

impl Spanner for TokenTree {
    fn span(&self) -> Span {
        self.span()
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

impl ToTokens for String {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        crate::Lit::string(self).to_tokens(tokens);
    }
}

impl ToTokens for &mut String {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        crate::Lit::string(self).to_tokens(tokens);
    }
}

/// Lex a run of consecutive `proc_macro` punctuation chars into moxy
/// [`Punctuation`] tokens, preserving each token's real compiler span.
///
/// `proc_macro` emits multi-char operators (`=>`, `::`, `&&`) as single-char
/// puncts; moxy models them as one variant. We longest-match against the run's
/// text (reusing `<Punctuation as Scan>`) and retag each matched variant with
/// the joined spans of the chars it consumed.
pub(crate) fn scan_puncts_spanned(run: &[(char, Span)], tokens: &mut TokenStream) {
    let text: String = run.iter().map(|(c, _)| *c).collect();
    let mut cursor = Cursor::new(&text, 0);
    let mut idx = 0usize;

    while !cursor.is_empty() {
        match <Punctuation as Scan>::scan(cursor) {
            Ok((next, mut op)) => {
                let consumed = op.as_str().chars().count();
                let span = run[idx].1.join(run[idx + consumed - 1].1);
                op.set_span(span);
                tokens.extend_one(TokenTree::Punct(op));
                idx += consumed;
                cursor = next;
            }
            Err(_) => break,
        }
    }
}
