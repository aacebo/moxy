pub mod keyword;
mod tmpl_interp;
mod tmpl_tokens;

pub use keyword::TmplKeyword;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, LexError, Parse, Punctuation, Span, ToTokens, Token, TokenStream, TokenTree};
pub use tmpl_interp::*;
pub use tmpl_tokens::*;

#[doc = "A single node in a template."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "lowercase"))]
pub enum Node {
    Tokens(TmplTokens),
    Interp(TmplInterp),
    Keyword(TmplKeyword),
}

impl Parse for Node {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            Some(TokenTree::Token(Token::Punct(Punctuation::At(_)))) => Ok(Node::Keyword(stream.parse::<TmplKeyword>()?)),
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
            Some(TokenTree::Token(Token::Punct(Punctuation::At(_)))) => break,
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
