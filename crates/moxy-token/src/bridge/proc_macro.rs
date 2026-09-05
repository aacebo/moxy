use crate::span::fallback;
use crate::{Delim, Group, Ident, Keyword, Lit, Spacing, Span, ToTokens, TokenStream, TokenTree};

// --- Span (fallback) ---

impl From<proc_macro::Span> for fallback::Span {
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

impl From<fallback::Span> for proc_macro::Span {
    fn from(_value: fallback::Span) -> Self {
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
        let name = value.text();

        match name.strip_prefix("r#") {
            Some(raw) => Self::new_raw(raw, span),
            None => Self::new(name, span),
        }
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
        let span = value.span().span().into();
        let mut group = Self::new(value.delim.into(), value.tokens.into());
        group.set_span(span);
        group
    }
}

// --- TokenStream ---

impl ToTokens<TokenStream> for proc_macro::TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(v) => {
                let tt = match Keyword::from_str(&v.to_string(), v.span().into()) {
                    Some(kw) => TokenTree::Keyword(kw),
                    None => TokenTree::Ident(v.clone().into()),
                };
                tokens.extend_one(tt)
            }
            Self::Literal(v) => tokens.extend_one(TokenTree::Literal(v.clone().into())),
            Self::Group(v) => tokens.extend_one(TokenTree::Group(v.clone().into())),
            Self::Punct(p) => crate::scan_puncts_spanned(&[(p.as_char(), p.span().into(), p.spacing().into())], tokens),
        }
    }
}

impl ToTokens<TokenStream> for proc_macro::TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut punct_run: Vec<(char, Span, Spacing)> = Vec::new();

        for tt in self.clone() {
            match tt {
                proc_macro::TokenTree::Punct(p) => punct_run.push((p.as_char(), p.span().into(), p.spacing().into())),
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
            Self::Group(g) => out.extend(vec![proc_macro::TokenTree::Group(g.clone().into())]),
            Self::Ident(v) => out.extend(vec![proc_macro::TokenTree::Ident(v.clone().into())]),
            Self::Keyword(kw) => {
                let id = proc_macro::Ident::new(kw.as_str(), kw.span().into());
                out.extend(vec![proc_macro::TokenTree::Ident(id)])
            }
            Self::Literal(v) => out.extend(vec![proc_macro::TokenTree::Literal(v.clone().into())]),
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

impl From<proc_macro::TokenStream> for TokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        let mut out = Self::new();
        value.to_tokens(&mut out);
        out
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
