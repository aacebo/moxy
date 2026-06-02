use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Parse, ToTokens, TokenStream};

use crate::ast::Node;

#[doc = "A parsed template: a sequence of nodes (literal tokens, interpolations, and control flow)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Template {
    pub nodes: Vec<Node>,
}

impl Parse for Template {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let nodes = stream.parse_vec::<Node>()?;
        Ok(Self { nodes })
    }
}

impl ToTokens for Template {
    fn to_tokens(&self, t: &mut TokenStream) {
        for node in &self.nodes {
            node.to_tokens(t);
        }
    }
}
