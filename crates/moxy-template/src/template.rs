use std::str::FromStr;

use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Parse, ToTokens, TokenStream, TokenTree};

use crate::ast::Node;

#[doc = "A parsed template: a sequence of nodes (literal tokens, interpolations, and control flow)."]
#[derive(Debug, Clone)]
pub struct Template {
    pub nodes: Vec<Node>,
}

impl Template {
    pub fn expand(&self) -> TokenStream {
        let mut body = TokenStream::from_str("let mut __moxy_tmpl = ::moxy_token::TokenStream::new();").unwrap();
        self.to_tokens(&mut body);
        body.extend(TokenStream::from_str("__moxy_tmpl").unwrap());
        TokenStream::from(vec![TokenTree::Group(Group::new(Delim::Brace, body))])
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
