pub mod keyword;
mod paste;
mod tmpl_interp;
mod tmpl_tokens;

pub use keyword::TmplKeyword;
use moxy_ast::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Delim, Group, Keyword, LexError, Punct, Span, ToTokens, TokenStream, TokenTree};
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
    fn peek(_: Cursor<'_>) -> bool {
        true
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let nodes = parser.parse_until_empty()?;
        Ok(Self { nodes })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while !cursor.is_empty() {
            cursor = cursor.skip::<Node>()?;
        }

        Some(cursor)
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
        g.delim == Delim::Brace && lone_brace_child(&g.tokens).is_some()
    }

    pub fn group_has_interp(g: &Group) -> bool {
        Self::is_interp_group(g) || is_template(&g.tokens)
    }
}

pub fn lone_brace_child(parser: &TokenStream) -> Option<Group> {
    match (parser.len(), parser.get(0)) {
        (1, Some(TokenTree::Group(g))) if g.delim == Delim::Brace => Some(g.clone()),
        _ => None,
    }
}

impl Parse for Node {
    fn peek(cursor: Cursor<'_>) -> bool {
        !cursor.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::At(_))) => Ok(Self::Keyword(parser.parse()?)),
            Some(TokenTree::Group(g)) if Self::is_interp_group(g) => {
                let interp: TmplInterp = parser.parse()?;
                let wrap = interp.wrap;
                let mut node = Self::Interp(interp);

                for _ in 0..wrap {
                    node = Self::Group(Delim::Brace, Box::new(Template { nodes: vec![node] }));
                }

                Ok(node)
            }
            Some(TokenTree::Group(g)) if Self::group_has_interp(g) => {
                let delim = g.delim;
                let parser = parser.parse_group(g.delim)?;
                Ok(Self::Group(delim, Box::new(parser.parse()?)))
            }
            Some(_) => Ok(Self::Tokens(parser.parse()?)),
            None => Err(LexError::new(Span::default()).message("unexpected end of template").into()),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<TmplKeyword>() {
            return cursor.skip::<TmplKeyword>();
        }

        if cursor.peek::<TmplInterp>() {
            return cursor.skip::<TmplInterp>();
        }

        if let Some(TokenTree::Group(group)) = cursor.curr()
            && Self::group_has_interp(group)
        {
            let inner = cursor.descend(group.delim)?;
            let inner = Template::skip(inner)?;
            return inner.is_empty().then(|| cursor.offset(1));
        }

        cursor.skip::<TmplTokens>()
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

fn is_template(parser: &TokenStream) -> bool {
    let mut iter = parser.iter();

    while let Some(token) = iter.next() {
        if let TokenTree::Punct(Punct::At(_)) = token {
            if let Some(TokenTree::Keyword(next)) = iter.next() {
                if matches!(next, Keyword::If(_) | Keyword::Else(_) | Keyword::For(_) | Keyword::Match(_)) {
                    return true;
                }
            }
        } else if let TokenTree::Group(g) = token
            && (Node::is_interp_group(g) || is_template(&g.tokens))
        {
            return true;
        }
    }

    false
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
