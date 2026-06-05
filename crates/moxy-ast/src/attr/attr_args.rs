use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Eq;
use moxy_token::{LexError, Parse, ToTokens, TokenStream, TokenTree};

use crate::{Delimited, Expr};

#[doc = "The arguments of an attribute, after its path (e.g. the `(Clone)` in `derive(Clone)` or the `= \"x\"` in `path = \"x\"`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum AttrArgs {
    Empty,
    Delimited(Delimited<TokenStream>),
    NameValue { eq: Eq, value: Box<Expr> },
}

impl Parse for AttrArgs {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            None => Ok(AttrArgs::Empty),
            Some(TokenTree::Group(g)) => {
                let style = g.delim();
                let span = g.span();
                let tokens = g.stream();
                stream.advance();
                Ok(AttrArgs::Delimited(Delimited::new(style, span, tokens)))
            }
            Some(TokenTree::Token(moxy_token::Token::Punct(moxy_token::Punctuation::Eq(_)))) => {
                let eq = stream.parse::<Eq>()?;
                let value = stream.parse::<Expr>()?;
                Ok(AttrArgs::NameValue {
                    eq,
                    value: Box::new(value),
                })
            }
            _ => Err(LexError::new(stream.span()).message("expected attribute arguments").into()),
        }
    }
}

impl ToTokens for AttrArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AttrArgs::Empty => {}
            AttrArgs::Delimited(d) => d.to_tokens(tokens),
            AttrArgs::NameValue { eq, value } => {
                eq.to_tokens(tokens);
                value.to_tokens(tokens);
            }
        }
    }
}
