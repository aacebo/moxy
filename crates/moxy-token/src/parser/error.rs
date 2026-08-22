use crate::punct::Not;
use crate::{Delim, Group, Ident, LexError, Lit, Punctuation, Semi, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

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
        let ident = Ident::new("compile_error").with_span(self.span);
        let bang = Not::new(self.span);
        let mut lit = Lit::string(&self.to_string());

        lit.set_span(self.span);

        let inner: TokenTree = lit.into();
        let group = Group::new(Delim::Paren, inner.into_token_stream());

        vec![
            TokenTree::from(ident),
            TokenTree::from(Punctuation::from(bang)),
            TokenTree::from(group),
            Punctuation::from(Semi::new(self.span)).into_token_tree(),
        ]
        .into()
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

impl<T: ToTokens, E: ToTokens> ToTokens for Result<T, E> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ok(v) => v.to_tokens(tokens),
            Self::Err(err) => err.to_tokens(tokens),
        }
    }
}
