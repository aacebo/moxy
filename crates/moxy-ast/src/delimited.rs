use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::span::{DelimSpan, Spanner};
use moxy_token::{Delim, Group, Parse, Span, ToTokens, TokenStream, TokenTree};

#[derive(Debug, Clone)]
pub struct Delimited<T = TokenStream> {
    pub style: Delim,
    pub span: DelimSpan,
    pub inner: T,
}

impl<T> Delimited<T> {
    pub fn new(style: Delim, span: DelimSpan, inner: T) -> Self {
        Self { style, span, inner }
    }

    pub fn paren(span: DelimSpan, inner: T) -> Self {
        Self::new(Delim::Paren, span, inner)
    }

    pub fn brace(span: DelimSpan, inner: T) -> Self {
        Self::new(Delim::Brace, span, inner)
    }

    pub fn bracket(span: DelimSpan, inner: T) -> Self {
        Self::new(Delim::Bracket, span, inner)
    }

    pub fn open(&self) -> Span {
        self.span.open()
    }

    pub fn close(&self) -> Span {
        self.span.close()
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn surround(&self, tokens: &mut TokenStream, inner: TokenStream)
    where
        T: ToTokens,
    {
        let mut group = Group::new(self.style, inner);
        group.set_span(self.span);
        tokens.extend_one(TokenTree::Group(group));
    }
}

impl<T> Delimited<T> {
    pub fn parse_with_fn<F>(style: Delim, stream: &mut ParseStream, f: F) -> Result<Self, ParseError>
    where
        F: FnOnce(&mut ParseStream) -> Result<T, ParseError>,
    {
        let (span, group_tokens) = stream.parse_group_spanned(style)?;
        let mut inner_stream = group_tokens.parse();
        let inner = f(&mut inner_stream)?;
        Ok(Self { style, span, inner })
    }

    pub fn parse_paren_with<F>(stream: &mut ParseStream, f: F) -> Result<Self, ParseError>
    where
        F: FnOnce(&mut ParseStream) -> Result<T, ParseError>,
    {
        Self::parse_with_fn(Delim::Paren, stream, f)
    }

    pub fn parse_brace_with<F>(stream: &mut ParseStream, f: F) -> Result<Self, ParseError>
    where
        F: FnOnce(&mut ParseStream) -> Result<T, ParseError>,
    {
        Self::parse_with_fn(Delim::Brace, stream, f)
    }

    pub fn parse_bracket_with<F>(stream: &mut ParseStream, f: F) -> Result<Self, ParseError>
    where
        F: FnOnce(&mut ParseStream) -> Result<T, ParseError>,
    {
        Self::parse_with_fn(Delim::Bracket, stream, f)
    }
}

impl<T: Parse> Delimited<T> {
    pub fn parse_with(style: Delim, stream: &mut ParseStream) -> Result<Self, ParseError> {
        Self::parse_with_fn(style, stream, T::parse)
    }

    pub fn parse_paren(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Self::parse_with(Delim::Paren, stream)
    }

    pub fn parse_brace(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Self::parse_with(Delim::Brace, stream)
    }

    pub fn parse_bracket(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Self::parse_with(Delim::Bracket, stream)
    }
}

impl<T: ToTokens> ToTokens for Delimited<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.inner.to_tokens(&mut inner);
        self.surround(tokens, inner);
    }
}

impl<T: ToTokens> Spanner for Delimited<T> {
    fn span(&self) -> Span {
        self.span.span()
    }
}

impl<T: PartialEq> PartialEq for Delimited<T> {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style && self.inner == other.inner
    }
}

impl<T: Eq> Eq for Delimited<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Delimited<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.style.hash(state);
        self.inner.hash(state);
    }
}

impl<T: Default> Default for Delimited<T> {
    fn default() -> Self {
        Self {
            style: Delim::default(),
            span: DelimSpan::default(),
            inner: T::default(),
        }
    }
}

impl<T> std::ops::Deref for Delimited<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for Delimited<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Delimited<T> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut o = s.serialize_struct("Delimited", 2)?;
        o.serialize_field("style", self.style.as_str())?;
        o.serialize_field("inner", &self.inner)?;
        o.end()
    }
}
