use crate::parse::ParseError;
use crate::token::{Delim, Group, Ident, Keyword, Literal, Punctuation, Spacing, ToTokens};
use crate::{Span, Token, TokenStream, TokenTree};

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
            Delim::Paren => proc_macro2::Delimiter::Parenthesis,
            Delim::Brace => proc_macro2::Delimiter::Brace,
            Delim::Bracket => proc_macro2::Delimiter::Bracket,
            Delim::None => proc_macro2::Delimiter::None,
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
            Spacing::Alone => proc_macro2::Spacing::Alone,
            Spacing::Joint => proc_macro2::Spacing::Joint,
        }
    }
}

// --- Ident ---

impl From<proc_macro2::Ident> for Ident {
    fn from(value: proc_macro2::Ident) -> Self {
        let span: Span = value.span().into();
        Ident::new(&value.to_string(), span)
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(value: Ident) -> Self {
        let name = value.name();
        let span = proc_macro2::Span::call_site();

        match name.strip_prefix("r#") {
            Some(name) => proc_macro2::Ident::new_raw(name, span),
            None => proc_macro2::Ident::new(&name, span),
        }
    }
}

// --- Literal ---

impl From<proc_macro2::Literal> for Literal {
    fn from(value: proc_macro2::Literal) -> Self {
        let span: Span = value.span().into();
        Literal {
            repr: value.to_string().into_boxed_str(),
            span,
        }
    }
}

impl From<Literal> for proc_macro2::Literal {
    fn from(value: Literal) -> Self {
        let repr = format!("{}", value);
        repr.parse().unwrap_or_else(|_| proc_macro2::Literal::string(&repr))
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
        proc_macro2::Group::new(delim, stream)
    }
}

// --- TokenStream ---

impl ToTokens<TokenStream> for proc_macro2::TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut punct_buf = String::new();

        for tt in self.clone() {
            match tt {
                proc_macro2::TokenTree::Punct(p) => punct_buf.push(p.as_char()),
                other => {
                    if !punct_buf.is_empty() {
                        scan_puncts(&punct_buf, tokens);
                        punct_buf.clear();
                    }
                    match other {
                        proc_macro2::TokenTree::Ident(v) => {
                            let span: Span = v.span().into();
                            let token = match Keyword::from_str(&v.to_string(), span) {
                                Some(kw) => Token::Keyword(kw),
                                None => Token::Ident(v.into()),
                            };
                            tokens.extend_one(token.into())
                        }
                        proc_macro2::TokenTree::Literal(v) => tokens.extend_one(Token::Literal(v.into()).into()),
                        proc_macro2::TokenTree::Group(v) => tokens.extend_one(TokenTree::Group(v.into())),
                        proc_macro2::TokenTree::Punct(_) => unreachable!(),
                    }
                }
            }
        }

        if !punct_buf.is_empty() {
            scan_puncts(&punct_buf, tokens);
        }
    }
}

fn scan_puncts(s: &str, tokens: &mut TokenStream) {
    use crate::source::SourceMap;
    use crate::token::lex::{Cursor, Scan};

    let span = SourceMap::with_mut(|sm| sm.push(s));
    let mut cursor = Cursor::new(s, span.byte_range().start as u32);

    while !cursor.is_empty() {
        match <Punctuation as Scan>::scan(cursor) {
            Ok((next, op)) => {
                tokens.extend_one(Token::Punct(op).into());
                cursor = next;
            }
            Err(_) => break,
        }
    }
}

impl ToTokens<proc_macro2::TokenStream> for TokenTree {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        match self {
            TokenTree::Group(g) => out.extend([proc_macro2::TokenTree::Group(g.clone().into())]),
            TokenTree::Token(Token::Ident(v)) => out.extend([proc_macro2::TokenTree::Ident(v.clone().into())]),
            TokenTree::Token(Token::Keyword(kw)) => {
                let id = proc_macro2::Ident::new(kw.as_str(), proc_macro2::Span::call_site());
                out.extend([proc_macro2::TokenTree::Ident(id)])
            }
            TokenTree::Token(Token::Literal(v)) => out.extend([proc_macro2::TokenTree::Literal(v.clone().into())]),
            TokenTree::Token(Token::Punct(op)) => {
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
        let mut out = TokenStream::new();
        stream.to_tokens(&mut out);
        out
    }
}

impl From<TokenStream> for proc_macro2::TokenStream {
    fn from(stream: TokenStream) -> Self {
        let mut out = proc_macro2::TokenStream::new();
        stream.to_tokens(&mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::TokenStream;
    use crate::token::{Punctuation, Token, TokenTree};

    #[test]
    fn coalesces_joint_puncts_inbound() {
        let pm2: proc_macro2::TokenStream = "a == b".parse().unwrap();
        let ours: TokenStream = pm2.into();
        let trees: Vec<TokenTree> = ours.into_iter().collect();
        assert!(matches!(trees[1], TokenTree::Token(Token::Punct(Punctuation::EqEq(_)))));
    }

    #[test]
    fn builds_owned_types_inbound() {
        let pm2: proc_macro2::TokenStream = "foo 1 (x)".parse().unwrap();
        let ours: TokenStream = pm2.into();
        let trees: Vec<TokenTree> = ours.into_iter().collect();

        let TokenTree::Token(Token::Ident(id)) = &trees[0] else {
            panic!("expected ident, got {:?}", trees[0]);
        };
        assert_eq!(id.name().as_ref(), "foo");

        assert!(matches!(trees[1], TokenTree::Token(Token::Literal(_))));

        let TokenTree::Group(g) = &trees[2] else {
            panic!("expected group, got {:?}", trees[2]);
        };
        assert_eq!(g.delim(), crate::token::Delim::Paren);
        assert!(matches!(
            g.stream().into_iter().next().unwrap(),
            TokenTree::Token(Token::Ident(_))
        ));
    }

    #[test]
    fn expands_ops_outbound() {
        let ours = TokenStream::from_str("==").unwrap();
        let pm2: proc_macro2::TokenStream = ours.into();
        let puncts = pm2
            .into_iter()
            .filter(|t| matches!(t, proc_macro2::TokenTree::Punct(_)))
            .count();
        assert_eq!(puncts, 2);
    }

    #[test]
    fn roundtrips_through_pm2() {
        let cases = [
            "a == b => c",
            "x :: y",
            "a >> b",
            "a <<= b",
            "x ..= y",
            "Vec<Vec<T>>",
            "a::b::<T>()",
            "a(b(c))",
            "fn f(x: Vec<(A, B)>) { g([1, 2]) }",
            "\"hello\"",
            "'c'",
            "42u8",
            "3.14",
            "b\"bytes\"",
            "pub fn foo() -> Self",
            "let _ = &mut x",
            "r#foo",
            "fn f<'a>(x: &'a Self) {}",
            "&'a T",
        ];

        for src in cases {
            let pm2: proc_macro2::TokenStream = src.parse().unwrap();

            let once: proc_macro2::TokenStream = TokenStream::from(pm2.clone()).into();
            let twice: proc_macro2::TokenStream = TokenStream::from(once.clone()).into();

            assert_eq!(once.to_string(), twice.to_string(), "boundary not idempotent for {src:?}");

            assert!(
                once.to_string().parse::<proc_macro2::TokenStream>().is_ok(),
                "round-tripped stream not reparseable for {src:?}"
            );
        }
    }

    #[test]
    fn nested_groups_preserve_structure_inbound() {
        let pm2: proc_macro2::TokenStream = "a(b(c))".parse().unwrap();
        let ours: TokenStream = pm2.into();
        let trees: Vec<TokenTree> = ours.into_iter().collect();

        let TokenTree::Group(outer) = &trees[1] else {
            panic!("expected outer group, got {:?}", trees[1]);
        };
        let inner: Vec<TokenTree> = outer.stream().into_iter().collect();
        assert!(
            inner.iter().any(|t| matches!(t, TokenTree::Group(_))),
            "expected a nested group inside the outer group"
        );
    }
}
