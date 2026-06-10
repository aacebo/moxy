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

    fn fused(self) -> Self {
        let mut nodes = Vec::with_capacity(self.nodes.len());
        let mut iter = self.nodes.into_iter().peekable();

        while let Some(node) = iter.next() {
            match node {
                Node::Interp(interp) => match iter.peek() {
                    Some(Node::Tokens(tokens)) if tokens.lead_ident().is_some() => {
                        let Some(Node::Tokens(tokens)) = iter.next() else {
                            unreachable!()
                        };

                        let suffix = tokens.lead_ident().unwrap();
                        let rest = tokens.rest();
                        nodes.push(Node::Concat(TmplConcat { interp, suffix }));

                        if !rest.is_empty() {
                            nodes.push(Node::Tokens(TmplTokens {
                                span: tokens.span,
                                stream: rest,
                            }));
                        }
                    }
                    _ => nodes.push(Node::Interp(interp)),
                },
                other => nodes.push(other),
            }
        }

        Self { nodes }
    }
}

impl Parse for Template {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let nodes = stream.parse::<Vec<Node>>()?;
        Ok(Self { nodes }.fused())
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
    Concat(TmplConcat),
    Group(Delim, Box<Template>),
    Keyword(TmplKeyword),
}

impl Node {
    fn is_interp_group(g: &Group) -> bool {
        g.delim() == Delim::Brace && matches!(g.stream().get(0), Some(TokenTree::Group(ig)) if ig.delim() == Delim::Brace)
    }

    fn group_has_interp(g: &Group) -> bool {
        !Self::is_interp_group(g) && stream_has_interp(&g.stream())
    }
}

impl Parse for Node {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            Some(TokenTree::Punct(Punctuation::At(_))) => Ok(Node::Keyword(stream.parse::<TmplKeyword>()?)),
            Some(TokenTree::Group(g)) if Node::is_interp_group(g) => Ok(Node::Interp(stream.parse::<TmplInterp>()?)),
            Some(TokenTree::Group(g)) if Node::group_has_interp(g) => {
                let delim = g.delim();
                let inner = g.stream();
                stream.advance();
                Ok(Node::Group(delim, Box::new(Template::parse(&mut inner.parse())?)))
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
            Node::Concat(v) => v.to_tokens(out),
            Node::Group(delim, body) => emit_group(*delim, body, out),
            Node::Keyword(v) => v.to_tokens(out),
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
        Delim::Paren => "::moxy_token::Delim::Paren",
        Delim::Brace => "::moxy_token::Delim::Brace",
        Delim::Bracket => "::moxy_token::Delim::Bracket",
        Delim::None => "::moxy_token::Delim::None",
    };

    let mut inner = TokenStream::from_str("let mut __moxy_tmpl = ::moxy_token::TokenStream::new();").unwrap();
    body.to_tokens(&mut inner);
    inner.extend(TokenStream::from_str("__moxy_tmpl").unwrap());

    let mut args = TokenStream::from_str(delim_path).unwrap();
    args.extend(TokenStream::from_str(",").unwrap());
    args.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));

    let mut group_call = TokenStream::from_str("::moxy_token::Group::new").unwrap();
    group_call.extend_one(TokenTree::Group(Group::new(Delim::Paren, args)));

    let mut tree_args = TokenStream::from_str("::moxy_token::TokenTree::Group").unwrap();
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
