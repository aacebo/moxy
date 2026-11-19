use moxy_token::{Delim, DelimSpan, Group, Ident, LexError, Lit, Punct, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

/// An AST representation of Rust parse error syntax.
#[derive(Debug, Clone)]
pub struct ParseError {
    span: Span,
    // The last token of the node the error covers; equal to `span` for an
    // error at one token
    end: Span,
    message: String,
    children: Vec<Self>,
}

impl ParseError {
    // The error for input nested deeper than the parser reads
    pub(crate) fn too_deep(span: Span) -> Self {
        Self::new(span, format!("nesting is deeper than {} levels", moxy_token::MAX_DEPTH))
    }

    pub fn new(span: Span, message: impl std::fmt::Display) -> Self {
        Self {
            span,
            end: span,
            message: message.to_string(),
            children: vec![],
        }
    }

    /// Returns an error that covers every token of `node`, from its first token
    /// to its last.
    ///
    /// A joined span covers the whole node only on nightly, while the
    /// compiler underlines an error from the span of its `compile_error!`
    /// name to the span of its arguments; so [`to_compile_error`] places the
    /// first token's span on the name and the last token's on the arguments,
    /// which underlines the whole node on stable Rust too.
    ///
    /// [`to_compile_error`]: Self::to_compile_error
    pub fn new_spanned(node: impl ToTokens, message: impl std::fmt::Display) -> Self {
        let tokens = node.to_token_stream();
        let start = tokens.iter().next().map(|token| match token {
            TokenTree::Group(group) => group.span.open(),
            token => token.span(),
        });
        let end = tokens.iter().last().map(|token| match token {
            TokenTree::Group(group) => group.span.close(),
            token => token.span(),
        });
        let span = start.unwrap_or_else(Span::call_site);

        Self {
            span,
            end: end.unwrap_or(span),
            message: message.to_string(),
            children: vec![],
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    /// The span of the last token the error covers; the same as [`Self::span`]
    /// unless the error came from [`Self::new_spanned`].
    pub fn end_span(&self) -> Span {
        self.end
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

    /// Returns a `compile_error!` invocation for this error, followed by one for
    /// each error combined into it, so the compiler reports all of them.
    pub fn to_compile_error(&self) -> TokenStream {
        let mut tokens = self.to_compile_error_alone();

        for child in &self.children {
            tokens.extend(child.to_compile_error());
        }

        tokens
    }

    fn to_compile_error_alone(&self) -> TokenStream {
        let (start, end) = (self.span, self.end);
        let ident = Ident::new("compile_error").with_span(start);
        let bang = <Token![!]>::new(start);
        let mut lit = Lit::string(&self.message);
        lit.set_span(end);

        let mut group = Group::new(Delim::Paren, lit.into_token_tree().into_token_stream());
        group.span = DelimSpan::new(end, end);

        vec![
            ident.into_token_tree(),
            Punct::from(bang).into_token_tree(),
            group.into_token_tree(),
            Punct::from(<Token![;]>::new(end)).into_token_tree(),
        ]
        .into_token_stream()
    }
}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        Self::new(e.span(), e)
    }
}

impl From<ParseError> for TokenStream {
    fn from(value: ParseError) -> Self {
        value.into_token_stream()
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
