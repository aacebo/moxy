pub mod keyword;
mod paste;
mod tmpl_interp;
mod tmpl_tokens;

pub use keyword::TmplKeyword;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Delim, Group, LexError, Parse, Punctuation, Span, ToTokens, TokenStream, TokenTree};
pub use paste::Paste;
pub use tmpl_interp::*;
pub use tmpl_tokens::*;

/// A parsed template: a sequence of nodes (literal tokens, interpolations, and control flow).
#[derive(Debug, Clone)]
pub struct Template {
    pub nodes: Vec<Node>,
}

impl Template {
    pub fn expand(&self) -> TokenStream {
        use std::str::FromStr;

        let mut body = TokenStream::from_str("let mut __moxy_tmpl = ::moxy_token::TokenStream::new();").unwrap();
        self.to_tokens(&mut body);
        body.extend(TokenStream::from_str("__moxy_tmpl").unwrap());
        TokenStream::from(vec![Group::new(Delim::Brace, body).to_token_tree()])
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

/// A single node in a template.
#[derive(Debug, Clone)]
pub enum Node {
    Tokens(TmplTokens),
    Interp(TmplInterp),
    Keyword(TmplKeyword),
}

impl Parse for Node {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            Some(TokenTree::Punct(Punctuation::At(_))) => Ok(Node::Keyword(stream.parse::<TmplKeyword>()?)),
            Some(TokenTree::Group(g)) if g.delim() == Delim::Brace => {
                let inner = g.stream();
                if matches!(inner.get(0), Some(TokenTree::Group(ig)) if ig.delim() == Delim::Brace) {
                    Ok(Node::Interp(stream.parse::<TmplInterp>()?))
                } else {
                    collect_tokens(stream)
                }
            }
            Some(_) => collect_tokens(stream),
            None => Err(LexError::new(Span::default()).message("unexpected end of template").into()),
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, out: &mut TokenStream) {
        match self {
            Node::Tokens(v) => v.to_tokens(out),
            Node::Interp(v) => v.to_tokens(out),
            Node::Keyword(v) => v.to_tokens(out),
        }
    }
}

fn collect_tokens(stream: &mut ParseStream) -> Result<Node, ParseError> {
    let span = stream.span();
    let mut tokens = TokenStream::new();

    loop {
        match stream.curr() {
            None => break,
            Some(TokenTree::Punct(Punctuation::At(_))) => break,
            Some(TokenTree::Group(g)) if g.delim() == Delim::Brace => {
                let inner = g.stream();
                if matches!(inner.get(0), Some(TokenTree::Group(ig)) if ig.delim() == Delim::Brace) {
                    break;
                }
                tokens.extend_one(stream.advance().unwrap().clone());
            }
            _ => {
                tokens.extend_one(stream.advance().unwrap().clone());
            }
        }
    }

    Ok(Node::Tokens(TmplTokens { span, stream: tokens }))
}
