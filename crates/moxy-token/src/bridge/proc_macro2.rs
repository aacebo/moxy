use crate::parser::ParseError;
use crate::{Delim, Group, Ident, Keyword, Lit, Spacing, Span, ToTokens, TokenStream, TokenTree};

// --- LexError ---

impl From<proc_macro2::LexError> for ParseError {
    fn from(value: proc_macro2::LexError) -> Self {
        let span = Span::Fallback(value.span().into());
        Self::new(span, value)
    }
}

// --- Span ---

impl From<proc_macro2::Span> for crate::span::fallback::Span {
    fn from(value: proc_macro2::Span) -> Self {
        Self::new(value.byte_range().start as u32, value.byte_range().end as u32)
    }
}

impl From<proc_macro2::Span> for Span {
    fn from(value: proc_macro2::Span) -> Self {
        Self::Fallback(value.into())
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
        let span: Span = value.span().into();
        Self::new(value.to_string()).with_span(span)
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(value: Ident) -> Self {
        let name = value.text();
        let span = proc_macro2::Span::call_site();

        match name.strip_prefix("r#") {
            Some(name) => Self::new_raw(name, span),
            None => Self::new(name, span),
        }
    }
}

// --- Literal ---

impl From<proc_macro2::Literal> for Lit {
    fn from(value: proc_macro2::Literal) -> Self {
        let span: Span = value.span().into();
        Self::from_repr(&value.to_string(), span)
    }
}

impl From<Lit> for proc_macro2::Literal {
    fn from(value: Lit) -> Self {
        let repr = value.repr();
        repr.parse().unwrap_or_else(|_| Self::string(repr))
    }
}

// --- Group ---

impl From<proc_macro2::Group> for Group {
    fn from(value: proc_macro2::Group) -> Self {
        let mut inner = TokenStream::new();
        value.stream().to_tokens(&mut inner);
        Self::new(value.delimiter().into(), inner)
    }
}

impl From<Group> for proc_macro2::Group {
    fn from(value: Group) -> Self {
        let delim: proc_macro2::Delimiter = value.delim().into();
        let mut stream = proc_macro2::TokenStream::new();
        value.stream().to_tokens(&mut stream);
        Self::new(delim, stream)
    }
}

// --- TokenStream ---

impl ToTokens<TokenStream> for proc_macro2::TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut punct_run: Vec<(char, Span)> = Vec::new();

        for tt in self.clone() {
            match tt {
                proc_macro2::TokenTree::Punct(p) => punct_run.push((p.as_char(), p.span().into())),
                other => {
                    if !punct_run.is_empty() {
                        crate::scan_puncts_spanned(&punct_run, tokens);
                        punct_run.clear();
                    }
                    match other {
                        proc_macro2::TokenTree::Ident(v) => {
                            let span: Span = v.span().into();
                            let tt = match Keyword::from_str(&v.to_string(), span) {
                                Some(kw) => TokenTree::Keyword(kw),
                                None => TokenTree::Ident(v.into()),
                            };
                            tokens.extend_one(tt)
                        }
                        proc_macro2::TokenTree::Literal(v) => tokens.extend_one(TokenTree::Literal(v.into())),
                        proc_macro2::TokenTree::Group(v) => tokens.extend_one(TokenTree::Group(v.into())),
                        proc_macro2::TokenTree::Punct(_) => unreachable!(),
                    }
                }
            }
        }

        if !punct_run.is_empty() {
            crate::scan_puncts_spanned(&punct_run, tokens);
        }
    }
}

impl ToTokens<proc_macro2::TokenStream> for TokenTree {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        match self {
            Self::Group(g) => out.extend([proc_macro2::TokenTree::Group(g.clone().into())]),
            Self::Ident(v) => out.extend([proc_macro2::TokenTree::Ident(v.clone().into())]),
            Self::Keyword(kw) => {
                let id = proc_macro2::Ident::new(kw.as_str(), proc_macro2::Span::call_site());
                out.extend([proc_macro2::TokenTree::Ident(id)])
            }
            Self::Literal(v) => out.extend([proc_macro2::TokenTree::Literal(v.clone().into())]),
            Self::Punct(op) => {
                let text = op.as_str();
                let last = text.chars().count() - 1;
                let joint_last = text == "'";

                for (i, ch) in text.chars().enumerate() {
                    let spacing = if i == last && !joint_last {
                        proc_macro2::Spacing::Alone
                    } else {
                        proc_macro2::Spacing::Joint
                    };
                    out.extend([proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(ch, spacing))]);
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

impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(stream: proc_macro2::TokenStream) -> Self {
        let mut out = Self::new();
        stream.to_tokens(&mut out);
        out
    }
}

impl From<TokenStream> for proc_macro2::TokenStream {
    fn from(stream: TokenStream) -> Self {
        let mut out = Self::new();
        stream.to_tokens(&mut out);
        out
    }
}
