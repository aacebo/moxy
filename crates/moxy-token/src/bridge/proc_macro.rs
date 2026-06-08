use crate::parser::ParseError;
use crate::span::fallback;
use crate::{Delim, Group, Ident, Keyword, Literal, Spacing, Span, ToTokens, TokenStream, TokenTree};

// --- LexError ---

impl From<proc_macro::LexError> for ParseError {
    fn from(e: proc_macro::LexError) -> Self {
        Self::new(Span::default(), e)
    }
}

// --- Span (fallback) ---

impl From<proc_macro::Span> for fallback::Span {
    fn from(value: proc_macro::Span) -> Self {
        if cfg!(nightly) {
            let r = value.byte_range();
            return Self::new(r.start as u32, r.end as u32);
        }

        Self::default()
    }
}

impl From<fallback::Span> for proc_macro::Span {
    fn from(_value: fallback::Span) -> Self {
        proc_macro::Span::call_site()
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
            Span::Fallback(_) => proc_macro::Span::call_site(),
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
            Delim::Paren => proc_macro::Delimiter::Parenthesis,
            Delim::Brace => proc_macro::Delimiter::Brace,
            Delim::Bracket => proc_macro::Delimiter::Bracket,
            Delim::None => proc_macro::Delimiter::None,
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
            Spacing::Alone => proc_macro::Spacing::Alone,
            Spacing::Joint => proc_macro::Spacing::Joint,
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
        let name = value.text();

        match name.strip_prefix("r#") {
            Some(raw) => proc_macro::Ident::new_raw(raw, span),
            None => proc_macro::Ident::new(name, span),
        }
    }
}

// --- Literal ---

impl From<proc_macro::Literal> for Literal {
    fn from(value: proc_macro::Literal) -> Self {
        Self {
            repr: value.to_string().into_boxed_str(),
            span: value.span().into(),
        }
    }
}

impl From<Literal> for proc_macro::Literal {
    fn from(value: Literal) -> Self {
        value
            .repr
            .parse()
            .unwrap_or_else(|_| proc_macro::Literal::string(&value.repr))
    }
}

// --- Group ---

impl From<proc_macro::Group> for Group {
    #[inline]
    fn from(value: proc_macro::Group) -> Self {
        use crate::span::DelimSpan;
        let mut group = Self::new(value.delimiter().into(), value.stream().into());
        group.set_span(DelimSpan::new(value.span_open().into(), value.span_close().into()));
        group
    }
}

impl From<Group> for proc_macro::Group {
    #[inline]
    fn from(value: Group) -> Self {
        proc_macro::Group::new(value.delim.into(), value.tokens.into())
    }
}

// --- TokenStream ---

impl ToTokens<TokenStream> for proc_macro::TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            proc_macro::TokenTree::Ident(v) => {
                let tt = match Keyword::from_str(&v.to_string(), v.span().into()) {
                    Some(kw) => TokenTree::Keyword(kw),
                    None => TokenTree::Ident(v.clone().into()),
                };
                tokens.extend_one(tt)
            }
            proc_macro::TokenTree::Literal(v) => tokens.extend_one(TokenTree::Literal(v.clone().into())),
            proc_macro::TokenTree::Group(v) => tokens.extend_one(TokenTree::Group(v.clone().into())),
            proc_macro::TokenTree::Punct(p) => crate::scan_puncts_spanned(&[(p.as_char(), p.span().into())], tokens),
        }
    }
}

impl ToTokens<TokenStream> for proc_macro::TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut punct_run: Vec<(char, Span)> = Vec::new();

        for tt in self.clone() {
            match tt {
                proc_macro::TokenTree::Punct(p) => punct_run.push((p.as_char(), p.span().into())),
                other => {
                    if !punct_run.is_empty() {
                        crate::scan_puncts_spanned(&punct_run, tokens);
                        punct_run.clear();
                    }
                    other.to_tokens(tokens);
                }
            }
        }

        if !punct_run.is_empty() {
            crate::scan_puncts_spanned(&punct_run, tokens);
        }
    }
}

impl ToTokens<proc_macro::TokenStream> for TokenTree {
    fn to_tokens(&self, out: &mut proc_macro::TokenStream) {
        match self {
            TokenTree::Group(g) => out.extend_one(proc_macro::TokenTree::Group(g.clone().into())),
            TokenTree::Ident(v) => out.extend_one(proc_macro::TokenTree::Ident(v.clone().into())),
            TokenTree::Keyword(kw) => {
                let id = proc_macro::Ident::new(kw.as_str(), kw.span().into());
                out.extend_one(proc_macro::TokenTree::Ident(id))
            }
            TokenTree::Literal(v) => out.extend_one(proc_macro::TokenTree::Literal(v.clone().into())),
            TokenTree::Punct(op) => {
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
        for t in value.iter() {
            t.to_tokens(&mut out);
        }
        out
    }
}
