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

        let mut body = TokenStream::from_str("let mut __moxy_tmpl = ::moxy::token::TokenStream::new();").unwrap();
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
    Group(Delim, Box<Template>),
    Keyword(TmplKeyword),
}

impl Node {
    pub fn is_interp_group(g: &Group) -> bool {
        g.delim() == Delim::Brace && lone_brace_child(&g.stream()).is_some()
    }

    pub fn group_has_interp(g: &Group) -> bool {
        !Self::is_interp_group(g) && stream_has_interp(&g.stream())
    }
}

pub fn lone_brace_child(stream: &TokenStream) -> Option<Group> {
    match (stream.len(), stream.get(0)) {
        (1, Some(TokenTree::Group(g))) if g.delim() == Delim::Brace => Some(g.clone()),
        _ => None,
    }
}

impl Parse for Node {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            Some(TokenTree::Punct(Punctuation::At(_))) => Ok(Self::Keyword(stream.parse::<TmplKeyword>()?)),
            Some(TokenTree::Group(g)) if Self::is_interp_group(g) => {
                let interp = stream.parse::<TmplInterp>()?;
                let wrap = interp.wrap;
                let mut node = Self::Interp(interp);

                for _ in 0..wrap {
                    node = Self::Group(Delim::Brace, Box::new(Template { nodes: vec![node] }));
                }

                Ok(node)
            }
            Some(TokenTree::Group(g)) if Self::group_has_interp(g) => {
                let delim = g.delim();
                let inner = g.stream();
                stream.advance();
                Ok(Self::Group(delim, Box::new(Template::parse(&mut inner.parse())?)))
            }
            Some(_) => collect_tokens(stream),
            None => Err(LexError::new(Span::default()).message("unexpected end of template").into()),
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, out: &mut TokenStream) {
        match self {
            Self::Tokens(v) => v.to_tokens(out),
            Self::Interp(v) => v.to_tokens(out),
            Self::Group(delim, body) => emit_group(*delim, body, out),
            Self::Keyword(v) => v.to_tokens(out),
        }
    }
}

fn stream_has_interp(stream: &TokenStream) -> bool {
    stream.iter().any(|tt| match tt {
        TokenTree::Group(g) => Node::is_interp_group(g) || stream_has_interp(&g.stream()),
        _ => false,
    })
}

fn emit_group(delim: Delim, body: &Template, out: &mut TokenStream) {
    use std::str::FromStr;

    let delim_path = match delim {
        Delim::Paren => "::moxy::token::Delim::Paren",
        Delim::Brace => "::moxy::token::Delim::Brace",
        Delim::Bracket => "::moxy::token::Delim::Bracket",
        Delim::None => "::moxy::token::Delim::None",
    };

    let mut inner = TokenStream::from_str("let mut __moxy_tmpl = ::moxy::token::TokenStream::new();").unwrap();
    body.to_tokens(&mut inner);
    inner.extend(TokenStream::from_str("__moxy_tmpl").unwrap());

    let mut args = TokenStream::from_str(delim_path).unwrap();
    args.extend(TokenStream::from_str(",").unwrap());
    args.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));

    let mut group_call = TokenStream::from_str("::moxy::token::Group::new").unwrap();
    group_call.extend_one(TokenTree::Group(Group::new(Delim::Paren, args)));

    let mut tree_args = TokenStream::from_str("::moxy::token::TokenTree::Group").unwrap();
    tree_args.extend_one(TokenTree::Group(Group::new(Delim::Paren, group_call)));

    out.extend(TokenStream::from_str("__moxy_tmpl.extend_one").unwrap());
    out.extend_one(TokenTree::Group(Group::new(Delim::Paren, tree_args)));
    out.extend(TokenStream::from_str(";").unwrap());
}

fn collect_tokens(stream: &mut ParseStream) -> Result<Node, ParseError> {
    let span = stream.span();
    let mut tokens = TokenStream::new();

    loop {
        match stream.curr() {
            None => break,
            Some(TokenTree::Punct(Punctuation::At(_))) => break,
            Some(TokenTree::Group(g)) if Node::is_interp_group(g) || Node::group_has_interp(g) => break,
            _ => {
                tokens.extend_one(stream.advance().unwrap().clone());
            }
        }
    }

    Ok(Node::Tokens(TmplTokens { span, stream: tokens }))
}
