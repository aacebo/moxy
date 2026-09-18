use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A closure expression: `|x| x`, `move || 1`, `async |x: u32| -> u32 { x }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprClosure {
    pub attrs: Attributes,
    pub lifetimes: Option<BoundLifetimes>,
    pub constness: Constness,
    pub movability: Movability,
    pub asyncness: Asyncness,
    pub capture: Option<Token![move]>,
    pub pipes: ClosurePipes,
    pub inputs: Punctuated<ClosureParam, Token![,]>,
    pub output: ReturnType,
    pub body: Box<Expr>,
}

impl From<ExprClosure> for Expr {
    fn from(value: ExprClosure) -> Self {
        Self::Closure(value)
    }
}

impl Spanner for ExprClosure {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ExprClosure {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
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

/// The pipe delimiters around a closure's parameters: either an empty `||` or a pair of `|`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ClosurePipes {
    Empty(Token![||]),
    Params(Token![|], Token![|]),
}
