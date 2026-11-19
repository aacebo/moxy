use crate::{
    Delim, Group, Ident, Keyword, LexError, Lit, Punct, Spacing, Span, ToTokens, TokenStream, TokenTree, TryIntoTokenStream,
};

// --- Span (fallback) ---

impl From<proc_macro::Span> for crate::span::fallback::Span {
    fn from(#[allow(unused)] value: proc_macro::Span) -> Self {
        #[cfg(nightly)]
        {
            let r = value.byte_range();
            Self::new(r.start as u32, r.end as u32)
        }

        #[cfg(not(nightly))]
        {
            Self::default()
        }
    }
}

impl From<crate::span::fallback::Span> for proc_macro::Span {
    fn from(_value: crate::span::fallback::Span) -> Self {
        Self::call_site()
    }
}

// --- Span ---

impl From<proc_macro::Span> for Span {
    #[inline]
    fn from(value: proc_macro::Span) -> Self {
        Self::Compiler(value)
    }
}

impl From<Span> for proc_macro::Span {
    fn from(value: Span) -> Self {
        match value {
            Span::Compiler(s) => s,
            Span::Fallback(_) => Self::call_site(),
        }
    }
}

#[cfg(nightly)]
impl proc_macro::MultiSpan for Span {
    fn into_spans(self) -> Vec<proc_macro::Span> {
        match self {
            Self::Compiler(s) => vec![s],
            Self::Fallback(_) => vec![proc_macro::Span::call_site()],
        }
    }
}

// --- Delim ---

impl From<proc_macro::Delimiter> for Delim {
    #[inline]
    fn from(value: proc_macro::Delimiter) -> Self {
        match value {
            proc_macro::Delimiter::Parenthesis => Self::Paren,
            proc_macro::Delimiter::Brace => Self::Brace,
            proc_macro::Delimiter::Bracket => Self::Bracket,
            proc_macro::Delimiter::None => Self::None,
        }
    }
}

impl From<Delim> for proc_macro::Delimiter {
    #[inline]
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

impl From<proc_macro::Spacing> for Spacing {
    #[inline]
    fn from(value: proc_macro::Spacing) -> Self {
        match value {
            proc_macro::Spacing::Alone => Self::Alone,
            proc_macro::Spacing::Joint => Self::Joint,
        }
    }
}

impl From<Spacing> for proc_macro::Spacing {
    #[inline]
    fn from(value: Spacing) -> Self {
        match value {
            Spacing::Alone => Self::Alone,
            Spacing::Joint => Self::Joint,
        }
    }
}

// --- Ident ---

impl From<proc_macro::Ident> for Ident {
    #[inline]
    fn from(value: proc_macro::Ident) -> Self {
        Self::new(value.to_string()).with_span(value.span().into())
    }
}

impl From<Ident> for proc_macro::Ident {
    fn from(value: Ident) -> Self {
        let span: proc_macro::Span = value.span().into();

        if value.is_raw() {
            Self::new_raw(value.text(), span)
        } else {
            Self::new(value.text(), span)
        }
    }
}

impl ToTokens for proc_macro::Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = Ident::from(self.clone()).into_token_tree();
        tokens.extend_one(token);
    }
}

// --- Literal ---

impl From<proc_macro::Literal> for Lit {
    fn from(value: proc_macro::Literal) -> Self {
        Self::from_repr(&value.to_string(), value.span().into())
    }
}

impl From<Lit> for proc_macro::Literal {
    fn from(value: Lit) -> Self {
        let repr = value.repr();
        let mut lit = repr.parse().unwrap_or_else(|_| Self::string(repr));

        lit.set_span(value.span().into());
        lit
    }
}

impl ToTokens for proc_macro::Literal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = Lit::from(self.clone()).into_token_tree();
        tokens.extend_one(token);
    }
}

// --- Group ---

/// Panics when the group's tokens nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] on a stream that holds the
/// group returns that as an error.
impl From<proc_macro::Group> for Group {
    #[inline]
    fn from(value: proc_macro::Group) -> Self {
        // Converted as a stream so the group's own level counts
        let stream = proc_macro::TokenStream::from(proc_macro::TokenTree::Group(value));
        let mut converted = super::expect_converted(convert(stream)).into_iter();

        match converted.next() {
            Some(TokenTree::Group(group)) => group,
            _ => unreachable!("a group converts to a group"),
        }
    }
}

impl From<Group> for proc_macro::Group {
    #[inline]
    fn from(value: Group) -> Self {
        let span = value.span.span().into();
        let mut group = Self::new(value.delim.into(), value.tokens.into());
        // `proc_macro::Group` accepts one span, the whole group's, which also
        // places both delimiters.
        group.set_span(span);
        group
    }
}

/// Panics when the group's tokens nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] on a stream that holds the
/// group returns that as an error.
impl ToTokens for proc_macro::Group {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = Group::from(self.clone()).into_token_tree();
        tokens.extend_one(token);
    }
}

// --- TokenStream ---

/// Panics when a group's tokens nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] on a stream that holds the
/// tree returns that as an error.
impl ToTokens<TokenStream> for proc_macro::TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(v) => {
                let span = v.span().into();
                let text = v.to_string();
                let tt = match text.as_str() {
                    "true" => TokenTree::Literal(Lit::Bool(crate::LitBool::new(true, span))),
                    "false" => TokenTree::Literal(Lit::Bool(crate::LitBool::new(false, span))),
                    _ => match Keyword::from_str(&text, span) {
                        Some(kw) => TokenTree::Keyword(kw),
                        None => TokenTree::Ident(v.clone().into()),
                    },
                };
                tokens.extend_one(tt)
            }
            Self::Literal(v) => tokens.extend_one(TokenTree::Literal(v.clone().into())),
            Self::Group(v) => tokens.extend_one(TokenTree::Group(v.clone().into())),
            Self::Punct(p) => crate::scan_puncts_spanned(&[(p.as_char(), p.span().into(), p.spacing().into())], tokens),
        }
    }
}

/// Panics when the stream's groups nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] returns that as an error.
impl ToTokens<TokenStream> for proc_macro::TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for token in Vec::<TokenTree>::from(super::expect_converted(convert(self.clone()))) {
            tokens.extend_one(token);
        }
    }
}

fn group(value: &proc_macro::Group, tokens: TokenStream) -> Group {
    use crate::span::DelimSpan;
    let mut group = Group::new(value.delimiter().into(), tokens);
    group.span = DelimSpan::with_whole(value.span_open().into(), value.span_close().into(), value.span().into());
    group
}

// The trees of one stream being converted, and the group they go into
struct Level {
    trees: proc_macro::token_stream::IntoIter,
    out: TokenStream,
    punct_run: Vec<(char, Span, Spacing)>,
    group: Option<proc_macro::Group>,
}

impl Level {
    fn new(stream: proc_macro::TokenStream, group: Option<proc_macro::Group>) -> Self {
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
fn convert(stream: proc_macro::TokenStream) -> Result<TokenStream, LexError> {
    let mut levels = vec![Level::new(stream, None)];

    loop {
        let level = levels.last_mut().expect("the outermost level is popped last");

        match level.trees.next() {
            Some(proc_macro::TokenTree::Punct(p)) => {
                level.punct_run.push((p.as_char(), p.span().into(), p.spacing().into()));
            }
            Some(proc_macro::TokenTree::Group(group)) => {
                level.flush();
                // The outermost level holds no group
                if levels.len() > crate::MAX_DEPTH {
                    return Err(LexError::too_deep(group.span().into()));
                }

                let stream = group.stream();
                levels.push(Level::new(stream, Some(group)));
            }
            Some(other) => {
                level.flush();
                other.to_tokens(&mut level.out);
            }
            None => {
                level.flush();
                let done = levels.pop().expect("a level is open");

                let Some(value) = done.group else {
                    return Ok(done.out);
                };

                let converted = group(&value, done.out);
                let parent = levels.last_mut().expect("a group sits inside a level");
                parent.out.extend_one(TokenTree::Group(converted));
            }
        }
    }
}

impl ToTokens<proc_macro::TokenStream> for TokenTree {
    fn to_tokens(&self, out: &mut proc_macro::TokenStream) {
        match self {
            Self::Group(g) => out.extend(vec![proc_macro::TokenTree::Group(g.clone().into())]),
            Self::Ident(v) => out.extend(vec![proc_macro::TokenTree::Ident(v.clone().into())]),
            Self::Keyword(kw) => {
                let id = proc_macro::Ident::new(kw.as_str(), kw.span().into());
                out.extend(vec![proc_macro::TokenTree::Ident(id)])
            }
            // `true`/`false` are identifiers to the compiler, not literals.
            Self::Literal(Lit::Bool(v)) => {
                let id = proc_macro::Ident::new(v.repr(), v.span().into());
                out.extend(vec![proc_macro::TokenTree::Ident(id)])
            }
            Self::Literal(v) => out.extend(vec![proc_macro::TokenTree::Literal(v.clone().into())]),
            // The compiler takes `_` only as an identifier
            Self::Punct(Punct::Underscore(op)) => {
                let id = proc_macro::Ident::new(op.as_str(), op.span().into());
                out.extend(vec![proc_macro::TokenTree::Ident(id)])
            }
            Self::Punct(op) => {
                let text = op.as_str();
                let span: proc_macro::Span = op.span().into();
                let last = text.chars().count() - 1;

                for (i, ch) in text.chars().enumerate() {
                    let spacing = if i == last {
                        op.spacing().into()
                    } else {
                        proc_macro::Spacing::Joint
                    };

                    let mut p = proc_macro::Punct::new(ch, spacing);
                    p.set_span(span);
                    out.extend(vec![proc_macro::TokenTree::Punct(p)]);
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

/// Panics on a stream whose groups nest deeper than [`crate::MAX_DEPTH`];
/// [`TryIntoTokenStream::try_into_token_stream`] returns that as an error.
impl From<proc_macro::TokenStream> for TokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        super::expect_converted(convert(value))
    }
}

impl From<TokenStream> for proc_macro::TokenStream {
    fn from(value: TokenStream) -> Self {
        let mut out = Self::new();

        for t in value.iter() {
            t.to_tokens(&mut out);
        }

        out
    }
}

impl TryIntoTokenStream for proc_macro::TokenStream {
    fn try_into_token_stream(self) -> Result<TokenStream, LexError> {
        convert(self)
    }
}

impl TryIntoTokenStream for &proc_macro::TokenStream {
    fn try_into_token_stream(self) -> Result<TokenStream, LexError> {
        convert(self.clone())
    }
}
