use std::str::FromStr;

use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Ident, Literal, Parse, Span, ToTokens, Token, TokenStream, TokenTree};

use crate::ast::Node;

pub(crate) const SINK: &str = "__moxy_tmpl";

#[doc = "A parsed template: a sequence of nodes (literal tokens, interpolations, and control flow)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Template {
    pub nodes: Vec<Node>,
}

impl Template {
    pub fn expand(&self) -> TokenStream {
        let mut body = rust("let mut __moxy_tmpl = ::moxy_token::TokenStream::new();");
        self.to_tokens(&mut body);
        body.extend_one(ident(SINK));

        TokenStream::from(vec![group(Delim::Brace, body)])
    }
}

impl Parse for Template {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let nodes = stream.parse::<Vec<Node>>()?;
        Ok(Self { nodes })
    }
}

impl ToTokens for Template {
    fn to_tokens(&self, out: &mut TokenStream) {
        for node in &self.nodes {
            node.to_tokens(out);
        }
    }
}

/// Tokenize fixed scaffolding we author. Spans are irrelevant here, so the
/// string round-trip is fine — never use this for user-written tokens.
pub(crate) fn rust(src: &str) -> TokenStream {
    TokenStream::from_str(src).expect("internal codegen produced invalid tokens")
}

pub(crate) fn ident(name: &str) -> TokenTree {
    TokenTree::Token(Token::Ident(Ident::new(name, Span::call_site())))
}

pub(crate) fn string_lit(value: &str) -> TokenTree {
    TokenTree::Token(Token::Literal(Literal::string(value)))
}

pub(crate) fn group(delim: Delim, inner: TokenStream) -> TokenTree {
    TokenTree::Group(Group::new(delim, inner))
}

/// A brace group holding the statements for a control-flow body's nodes,
/// appending to the enclosing sink.
pub(crate) fn brace_body(nodes: &[Node]) -> TokenTree {
    let mut inner = TokenStream::new();
    for node in nodes {
        node.to_tokens(&mut inner);
    }
    group(Delim::Brace, inner)
}

/// Emit `::moxy_token::ToTokens::to_tokens(&(<value>), &mut __moxy_tmpl);`,
/// splicing `value`'s tokens by value so their original spans survive.
pub(crate) fn push_value(out: &mut TokenStream, value: TokenStream) {
    out.extend(rust("::moxy_token::ToTokens::to_tokens"));

    let mut args = TokenStream::new();
    args.extend(rust("&"));
    args.extend_one(group(Delim::Paren, value));
    args.extend(rust(", &mut"));
    args.extend_one(ident(SINK));

    out.extend_one(group(Delim::Paren, args));
    out.extend(rust(";"));
}
