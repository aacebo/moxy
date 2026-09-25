use moxy_ast::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Delim, Group, Ident, Span, ToTokenStream, TokenStream, TokenTree};

#[doc = "A parsed `paste!` body: a token tree where each `{{ ... }}` marker is collapsed to one identifier."]
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug))]
pub struct Paste {
    nodes: Vec<PasteNode>,
}

#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug))]
enum PasteNode {
    Verbatim(TokenTree),
    Group(Delim, Vec<Self>),
    Splice(Span, TokenStream),
}

impl Parse for Paste {
    fn peek(_: Cursor<'_>) -> bool {
        true
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            nodes: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while !cursor.is_empty() {
            cursor = PasteNode::skip(cursor)?;
        }

        Some(cursor)
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

impl Parse for PasteNode {
    fn peek(cursor: Cursor<'_>) -> bool {
        !cursor.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Group(group)) if group.delim == Delim::Brace && super::lone_brace_child(&group.tokens).is_some() => {
                let span = group.span.into();
                let outer = parser.parse_group(Delim::Brace)?;
                let inner = outer.parse_group(Delim::Brace)?;
                Ok(Self::Splice(span, inner.to_token_stream()))
            }
            Some(TokenTree::Group(group)) => {
                let delim = group.delim;
                let body = parser.parse_group(delim)?;
                Ok(Self::Group(delim, <_ as Parse>::parse(&body)?))
            }
            Some(_) => {
                let token = parser.advance().ok_or_else(|| parser.error("expected paste node"))?.clone();
                Ok(Self::Verbatim(token))
            }
            None => parser.error("expected paste node").into(),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.curr().map(|_| cursor.offset(1))
    }
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
