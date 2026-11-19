use crate::{
    Delim, DelimSpan, Group, Ident, Keyword, LexError, Lit, Punct, Spacing, Span, ToTokens, TokenStream, TokenTree,
    TryIntoTokenStream,
};

// --- Span ---

impl From<proc_macro2::Span> for crate::span::fallback::Span {
    fn from(value: proc_macro2::Span) -> Self {
        Self::new(value.byte_range().start as u32, value.byte_range().end as u32)
    }
}

impl From<proc_macro2::Span> for Span {
    fn from(value: proc_macro2::Span) -> Self {
        if proc_macro::is_available() {
            Self::Compiler(value.unwrap())
        } else {
            Self::Fallback(value.into())
        }
    }
}

impl From<Span> for proc_macro2::Span {
    fn from(value: Span) -> Self {
        match value {
            Span::Compiler(v) => v.into(),
            // `proc_macro2` cannot construct a compiler span from source
            // locations outside a procedural macro invocation.
            Span::Fallback(_) => proc_macro2::Span::call_site(),
        }
    }
}

// --- DelimSpan ---

impl From<proc_macro2::extra::DelimSpan> for DelimSpan {
    fn from(value: proc_macro2::extra::DelimSpan) -> Self {
        Self::with_whole(value.open().into(), value.close().into(), value.join().into())
    }
}

// --- Delim ---

impl From<proc_macro2::Delimiter> for Delim {
    fn from(value: proc_macro2::Delimiter) -> Self {
        match value {
            proc_macro2::Delimiter::Parenthesis => Self::Paren,
            proc_macro2::Delimiter::Brace => Self::Brace,
            proc_macro2::Delimiter::Bracket => Self::Bracket,
            proc_macro2::Delimiter::None => Self::None,
        }
    }
}

impl From<Delim> for proc_macro2::Delimiter {
    fn from(value: Delim) -> Self {
        match value {
            Delim::Paren => Self::Parenthesis,
            Delim::Brace => Self::Brace,
            Delim::Bracket => Self::Bracket,
            Delim::None => Self::None,
        }
    }
}

// --- Spacing ---

impl From<proc_macro2::Spacing> for Spacing {
    fn from(value: proc_macro2::Spacing) -> Self {
        match value {
            proc_macro2::Spacing::Alone => Self::Alone,
            proc_macro2::Spacing::Joint => Self::Joint,
        }
    }
}

impl From<Spacing> for proc_macro2::Spacing {
    fn from(value: Spacing) -> Self {
        match value {
            Spacing::Alone => Self::Alone,
            Spacing::Joint => Self::Joint,
        }
    }
}

// --- Ident ---

impl From<proc_macro2::Ident> for Ident {
    fn from(value: proc_macro2::Ident) -> Self {
        let span = value.span().into();
        Self::new(value.to_string()).with_span(span)
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(value: Ident) -> Self {
        let span = value.span().into();

        if value.is_raw() {
            Self::new_raw(value.text(), span)
        } else {
            Self::new(value.text(), span)
        }
    }
}

impl ToTokens for proc_macro2::Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = Ident::from(self.clone()).into_token_tree();
        tokens.extend_one(token);
    }
}

// --- Literal ---

impl From<proc_macro2::Literal> for Lit {
    fn from(value: proc_macro2::Literal) -> Self {
        let span = value.span().into();
        Self::from_repr(&value.to_string(), span)
    }
}

impl From<Lit> for proc_macro2::Literal {
    fn from(value: Lit) -> Self {
        let repr = value.repr();
        let mut lit = repr.parse().unwrap_or_else(|_| Self::string(repr));
        lit.set_span(value.span().into());
        lit
    }
}

impl ToTokens for proc_macro2::Literal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = Lit::from(self.clone()).into_token_tree();
        tokens.extend_one(token);
    }
}

// --- Group ---

/// Panics when the group's tokens nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] on a stream that holds the
/// group returns that as an error.
impl From<proc_macro2::Group> for Group {
    fn from(value: proc_macro2::Group) -> Self {
        // Converted as a stream so the group's own level counts
        let stream = proc_macro2::TokenStream::from(proc_macro2::TokenTree::Group(value));
        let mut converted = super::expect_converted(convert(stream)).into_iter();

        match converted.next() {
            Some(TokenTree::Group(group)) => group,
            _ => unreachable!("a group converts to a group"),
        }
    }
}

impl From<Group> for proc_macro2::Group {
    fn from(value: Group) -> Self {
        let mut group = Self::new(value.delim.into(), value.tokens.into());
        // `proc_macro2::Group` accepts one span, the whole group's, which also
        // places both delimiters.
        group.set_span(value.span.span().into());
        group
    }
}

/// Panics when the group's tokens nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] on a stream that holds the
/// group returns that as an error.
impl ToTokens for proc_macro2::Group {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = Group::from(self.clone()).into_token_tree();
        tokens.extend_one(token);
    }
}

// --- TokenStream ---

/// Panics when the stream's groups nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] returns that as an error.
impl ToTokens<TokenStream> for proc_macro2::TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for token in Vec::<TokenTree>::from(super::expect_converted(convert(self.clone()))) {
            tokens.extend_one(token);
        }
    }
}

// The trees of one stream being converted, and the group they go into
struct Level {
    trees: proc_macro2::token_stream::IntoIter,
    out: TokenStream,
    punct_run: Vec<(char, Span, Spacing)>,
    group: Option<proc_macro2::Group>,
}

impl Level {
    fn new(stream: proc_macro2::TokenStream, group: Option<proc_macro2::Group>) -> Self {
        Self {
            trees: stream.into_iter(),
            out: TokenStream::new(),
            punct_run: Vec::new(),
            group,
        }
    }

    // Joins the punctuation read so far into its multi-character operators
    fn flush(&mut self) {
        if !self.punct_run.is_empty() {
            crate::scan_puncts_spanned(&self.punct_run, &mut self.out);
            self.punct_run.clear();
        }
    }
}

// Converts a stream with a stack of the groups it is inside rather than by
// recursion, and refuses groups nested past `MAX_DEPTH` as the lexer does, so
// no stream deeper than the lexer builds reaches the traversals that recurse
// per group
fn convert(stream: proc_macro2::TokenStream) -> Result<TokenStream, LexError> {
    let mut levels = vec![Level::new(stream, None)];

    loop {
        let level = levels.last_mut().expect("the outermost level is popped last");

        match level.trees.next() {
            Some(proc_macro2::TokenTree::Punct(p)) => {
                level.punct_run.push((p.as_char(), p.span().into(), p.spacing().into()));
            }
            Some(proc_macro2::TokenTree::Group(group)) => {
                level.flush();
                // The outermost level holds no group
                if levels.len() > crate::MAX_DEPTH {
                    return Err(LexError::too_deep(group.span().into()));
                }

                let stream = group.stream();
                levels.push(Level::new(stream, Some(group)));
            }
            Some(proc_macro2::TokenTree::Ident(v)) => {
                level.flush();
                let span = v.span().into();
                let text = v.to_string();
                let tt = match text.as_str() {
                    "true" => TokenTree::Literal(Lit::Bool(crate::LitBool::new(true, span))),
                    "false" => TokenTree::Literal(Lit::Bool(crate::LitBool::new(false, span))),
                    _ => match Keyword::from_str(&text, span) {
                        Some(kw) => TokenTree::Keyword(kw),
                        None => TokenTree::Ident(v.into()),
                    },
                };
                level.out.extend_one(tt);
            }
            Some(proc_macro2::TokenTree::Literal(v)) => {
                level.flush();
                level.out.extend_one(TokenTree::Literal(v.into()));
            }
            None => {
                level.flush();
                let done = levels.pop().expect("a level is open");

                let Some(group) = done.group else {
                    return Ok(done.out);
                };

                let converted = Group {
                    span: group.delim_span().into(),
                    delim: group.delimiter().into(),
                    tokens: done.out,
                };
                let parent = levels.last_mut().expect("a group sits inside a level");
                parent.out.extend_one(TokenTree::Group(converted));
            }
        }
    }
}

impl ToTokens<proc_macro2::TokenStream> for TokenTree {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        match self {
            Self::Group(g) => out.extend([proc_macro2::TokenTree::Group(g.clone().into())]),
            Self::Ident(v) => out.extend([proc_macro2::TokenTree::Ident(v.clone().into())]),
            Self::Keyword(kw) => {
                let id = proc_macro2::Ident::new(kw.as_str(), kw.span().into());
                out.extend([proc_macro2::TokenTree::Ident(id)])
            }
            // `true`/`false` are identifiers to the compiler, not literals.
            Self::Literal(Lit::Bool(v)) => {
                let id = proc_macro2::Ident::new(v.repr(), v.span().into());
                out.extend([proc_macro2::TokenTree::Ident(id)])
            }
            Self::Literal(v) => out.extend([proc_macro2::TokenTree::Literal(v.clone().into())]),
            // The compiler takes `_` only as an identifier
            Self::Punct(Punct::Underscore(op)) => {
                let id = proc_macro2::Ident::new(op.as_str(), op.span().into());
                out.extend([proc_macro2::TokenTree::Ident(id)])
            }
            Self::Punct(op) => {
                let text = op.as_str();
                let span: proc_macro2::Span = op.span().into();
                let last = text.chars().count() - 1;

                for (i, ch) in text.chars().enumerate() {
                    let spacing = if i == last {
                        op.spacing().into()
                    } else {
                        proc_macro2::Spacing::Joint
                    };
                    let mut punct = proc_macro2::Punct::new(ch, spacing);
                    punct.set_span(span);
                    out.extend([proc_macro2::TokenTree::Punct(punct)]);
                }
            }
        }
    }
}

impl ToTokens<proc_macro2::TokenStream> for TokenStream {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        for t in Vec::<TokenTree>::from(self.clone()) {
            t.to_tokens(out);
        }
    }
}

/// Panics on a stream whose groups nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] returns that as an error.
impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(stream: proc_macro2::TokenStream) -> Self {
        super::expect_converted(convert(stream))
    }
}

impl From<TokenStream> for proc_macro2::TokenStream {
    fn from(stream: TokenStream) -> Self {
        let mut out = Self::new();
        stream.to_tokens(&mut out);
        out
    }
}

impl TryIntoTokenStream for proc_macro2::TokenStream {
    fn try_into_token_stream(self) -> Result<TokenStream, LexError> {
        convert(self)
    }
}

impl TryIntoTokenStream for &proc_macro2::TokenStream {
    fn try_into_token_stream(self) -> Result<TokenStream, LexError> {
        convert(self.clone())
    }
}
