use crate::token::punct::Not;
use crate::token::{Delim, Group, Ident, LexError, Literal, Punctuation, ToTokenStream, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[derive(Debug, Clone)]
pub struct ParseError {
    span: Span,
    message: String,
    children: Vec<ParseError>,
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

    pub fn children(&self) -> &[ParseError] {
        &self.children
    }

    /// Combine two errors, appending `other` as a child of `self`.
    pub fn combine(mut self, other: ParseError) -> ParseError {
        self.children.push(other);
        self
    }

    pub fn to_compile_error(&self) -> TokenStream {
        let ident = Ident::new("compile_error", self.span);
        let bang = Not::new(self.span);
        let mut lit = Literal::string(&self.to_string());

        lit.set_span(self.span);

        let inner: TokenTree = lit.into();
        let group = Group::new(Delim::Paren, inner.into_token_stream());

        vec![
            TokenTree::from(ident),
            TokenTree::from(Punctuation::from(bang)),
            TokenTree::from(group),
        ]
        .into()
    }
}

impl From<proc_macro::LexError> for ParseError {
    fn from(e: proc_macro::LexError) -> Self {
        Self::new(Span::default(), e)
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
