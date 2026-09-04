use moxy_token::Token;
use moxy_token::span::DelimSpan;
use moxy_token::{Delim, Group, Ident, LexError, Lit, Punct, Span, ToTokenStream, ToTokens, TokenStream};

#[derive(Debug, Clone)]
pub struct ParseError {
    span: Span,
    message: String,
    children: Vec<Self>,
}

impl ParseError {
    pub fn new(span: Span, message: impl std::fmt::Display) -> Self {
        Self {
            span,
            message: message.to_string(),
            children: vec![],
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn children(&self) -> &[Self] {
        &self.children
    }

    /// Combine two errors, appending `other` as a child of `self`.
    pub fn combine(mut self, other: Self) -> Self {
        self.children.push(other);
        self
    }

    pub fn to_compile_error(&self) -> TokenStream {
        let span = self.span;
        let ident = Ident::new("compile_error").with_span(span);
        let bang = <Token![!]>::new(span);
        let mut lit = Lit::string(&self.message);
        lit.set_span(span);

        let mut group = Group::new(Delim::Paren, lit.into_token_tree().into_token_stream());
        group.set_span(DelimSpan::new(span, span));

        vec![
            ident.into_token_tree(),
            Punct::from(bang).into_token_tree(),
            group.into_token_tree(),
            Punct::from(<Token![;]>::new(span)).into_token_tree(),
        ]
        .into_token_stream()
    }
}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        Self::new(e.span(), e)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;

        for child in &self.children {
            write!(f, "\n{}", child)?;
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {}

impl ToTokens for ParseError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_compile_error().to_tokens(tokens);
    }
}

impl<T> From<ParseError> for Result<T, ParseError> {
    fn from(value: ParseError) -> Self {
        Self::Err(value)
    }
}
