use moxy_token::keyword::Move;
use moxy_token::parser::ParseStream;
use moxy_token::punct::{Comma, Or, OrOr};
use moxy_token::{Punctuation, Span, ToTokens, Token, TokenStream, TokenTree};

use crate::*;

#[doc = "The pipe delimiters around a closure's parameters: either an empty `||` or a pair of `|`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ClosurePipes {
    Empty(OrOr),
    Params(Or, Or),
}

#[doc = "A closure expression: `|x| x`, `move || 1`, `async |x: u32| -> u32 { x }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprClosure {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub lifetimes: Option<BoundLifetimes>,
    pub constness: Constness,
    pub movability: Movability,
    pub asyncness: Asyncness,
    pub capture: Option<Move>,
    pub pipes: ClosurePipes,
    pub inputs: Punctuated<ClosureParam, Comma>,
    pub output: ReturnType,
    pub body: Box<super::super::Expr>,
}

impl ExprClosure {
    /// Returns `true` when the stream is positioned at the start of a closure
    /// expression (`|...|`, `||`, `move`, or a `const`/`async` not followed by a block).
    pub fn is_start(stream: &mut ParseStream) -> bool {
        use moxy_token::keyword::Const;
        if stream.peek::<Or>().is_some() || stream.peek::<OrOr>().is_some() || stream.peek::<Move>().is_some() {
            return true;
        }

        let leads_closure = matches!(
            stream.nth(1),
            Some(TokenTree::Token(Token::Punct(Punctuation::Or(_) | Punctuation::OrOr(_))))
                | Some(TokenTree::Token(Token::Keyword(_)))
        );

        (stream.peek::<Const>().is_some() || stream.peek::<moxy_token::keyword::Async>().is_some())
            && leads_closure
            && !super::super::block::ExprBrace::is_next(stream)
    }
}

impl ToTokens for ExprClosure {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.constness.to_tokens(t);
        self.movability.to_tokens(t);
        self.asyncness.to_tokens(t);
        self.capture.to_tokens(t);

        match &self.pipes {
            ClosurePipes::Empty(oror) => oror.to_tokens(t),
            ClosurePipes::Params(open, close) => {
                open.to_tokens(t);
                self.inputs.to_tokens(t);
                close.to_tokens(t);
            }
        }

        self.output.to_tokens(t);
        self.body.to_tokens(t);
    }
}
