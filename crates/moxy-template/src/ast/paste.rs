use moxy_ast::{Parse, ParseError, Parser};
use moxy_token::{Delim, Group, Ident, Span, TokenStream, TokenTree};

#[doc = "A parsed `paste!` body: a token tree where each `{{ ... }}` marker is collapsed to one identifier."]
#[derive(Debug, Clone)]
pub struct Paste {
    nodes: Vec<PasteNode>,
}

#[derive(Debug, Clone)]
enum PasteNode {
    Verbatim(TokenTree),
    Group(Delim, Vec<Self>),
    Splice(Span, TokenStream),
}

impl Parse for Paste {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            nodes: parse_nodes(parser)?,
        })
    }
}

impl Paste {
    pub fn expand(&self) -> TokenStream {
        let mut out = TokenStream::new();

        for node in &self.nodes {
            match node.expand() {
                Ok(parser) => out.extend(parser),
                Err(e) => return e.to_compile_error(),
            }
        }

        out
    }
}

fn parse_nodes(parser: &Parser) -> Result<Vec<PasteNode>, ParseError> {
    let mut nodes = Vec::new();

    while let Some(tt) = parser.curr() {
        match tt {
            TokenTree::Group(g) if is_marker(g) => {
                let span = g.span().into();
                let outer = parser.parse_group(Delim::Brace)?;
                let outer_ps = Parser::from_tokens(&outer);
                let inner = outer_ps.parse_group(Delim::Brace)?;
                nodes.push(PasteNode::Splice(span, inner));
            }
            TokenTree::Group(g) => {
                let delim = g.delim();
                let body = g.stream();
                parser.advance();
                let parser = Parser::from_tokens(&body);
                nodes.push(PasteNode::Group(delim, parse_nodes(&parser)?));
            }
            _ => nodes.push(PasteNode::Verbatim(parser.advance().unwrap().clone())),
        }
    }

    Ok(nodes)
}

fn is_marker(g: &Group) -> bool {
    if g.delim() != Delim::Brace {
        return false;
    }

    let inner = g.stream();
    inner.len() == 1 && matches!(inner.get(0), Some(TokenTree::Group(ig)) if ig.delim() == Delim::Brace)
}

impl PasteNode {
    fn expand(&self) -> Result<TokenStream, ParseError> {
        match self {
            Self::Verbatim(tt) => Ok(TokenStream::from(vec![tt.clone()])),
            Self::Group(delim, kids) => {
                let mut body = TokenStream::new();

                for kid in kids {
                    body.extend(kid.expand()?);
                }

                Ok(TokenStream::from(vec![Group::new(*delim, body).to_token_tree()]))
            }
            Self::Splice(span, inner) => {
                let mut text = String::new();

                for tt in inner.iter() {
                    text.push_str(&segment_text(tt));
                }

                match Ident::lex(&text) {
                    Ok(id) => Ok(TokenStream::from(vec![id.with_span(*span).into_token_tree()])),
                    Err(e) => Err(e.into()),
                }
            }
        }
    }
}

fn segment_text(tt: &TokenTree) -> String {
    match tt {
        TokenTree::Ident(v) => v.text().to_string(),
        TokenTree::Keyword(v) => v.as_str().to_string(),
        TokenTree::Literal(v) => unquote(v.repr()).to_string(),
        other => other.to_string(),
    }
}

fn unquote(repr: &str) -> &str {
    let bytes = repr.as_bytes();

    match (bytes.first(), bytes.last()) {
        (Some(b'"'), Some(b'"')) | (Some(b'\''), Some(b'\'')) if repr.len() >= 2 => &repr[1..repr.len() - 1],
        _ => repr,
    }
}
