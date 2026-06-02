use crate::ast::Expr;
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Eq;
use crate::token::{Delim, Group, LexError, ToTokens};
use crate::{Parse, TokenStream, TokenTree};

#[doc = "The arguments of an attribute, after its path (e.g. the `(Clone)` in `derive(Clone)` or the `= \"x\"` in `path = \"x\"`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum AttrArgs {
    Empty,
    Delimited { delim: Delim, tokens: TokenStream },
    NameValue(Expr),
}

impl Parse for AttrArgs {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            None => Ok(AttrArgs::Empty),
            Some(TokenTree::Group(g)) => {
                let delim = g.delim;
                let tokens = g.stream();
                stream.advance();
                Ok(AttrArgs::Delimited { delim, tokens })
            }
            Some(TokenTree::Token(crate::Token::Punct(crate::token::Punctuation::Eq(_)))) => {
                let _ = stream.parse::<Eq>()?;
                Ok(AttrArgs::NameValue(stream.parse::<Expr>()?))
            }
            _ => Err(LexError::new(stream.span()).message("expected attribute arguments").into()),
        }
    }
}

impl ToTokens for AttrArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AttrArgs::Empty => {}
            AttrArgs::Delimited { delim, tokens: inner } => {
                tokens.extend_one(TokenTree::Group(Group::new(*delim, inner.clone())));
            }
            AttrArgs::NameValue(expr) => {
                Eq::default().to_tokens(tokens);
                expr.to_tokens(tokens);
            }
        }
    }
}
