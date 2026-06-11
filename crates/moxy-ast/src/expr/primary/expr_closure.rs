use moxy_token::keyword::Move;
use moxy_token::parser::ParseStream;
use moxy_token::punct::{Comma, Or, OrOr};
use moxy_token::{Punctuation, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::expr::block::ExprBrace;
use crate::*;

/// The pipe delimiters around a closure's parameters: either an empty `||` or a pair of `|`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ClosurePipes {
    Empty(OrOr),
    Params(Or, Or),
}

/// A closure expression: `|x| x`, `move || 1`, `async |x: u32| -> u32 { x }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprClosure {
    pub attrs: Attributes,
    pub lifetimes: Option<BoundLifetimes>,
    pub constness: Constness,
    pub movability: Movability,
    pub asyncness: Asyncness,
    pub capture: Option<Move>,
    pub pipes: ClosurePipes,
    pub inputs: Punctuated<ClosureParam, Comma>,
    pub output: ReturnType,
    pub body: Box<Expr>,
}

impl Spanner for ExprClosure {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(l) = &self.lifetimes {
            l.span()
        } else if !matches!(self.constness, Constness::NoConst) {
            self.constness.span()
        } else if !matches!(self.asyncness, Asyncness::Sync) {
            self.asyncness.span()
        } else {
            match &self.pipes {
                ClosurePipes::Empty(oror) => oror.span(),
                ClosurePipes::Params(open, _) => open.span(),
            }
        };
        start.join(self.body.span())
    }
}

impl ExprClosure {
    /// Returns `true` when the stream is positioned at the start of a closure
    /// expression (`|...|`, `||`, `move`, or a `const`/`async` not followed by a block).
    pub fn is_start(stream: &mut ParseStream) -> bool {
        use moxy_token::keyword::Const;
        if stream.peek::<Or>() || stream.peek::<OrOr>() || stream.peek::<Move>() {
            return true;
        }

        let leads_closure = matches!(
            stream.nth(1),
            Some(TokenTree::Punct(Punctuation::Or(_) | Punctuation::OrOr(_))) | Some(TokenTree::Keyword(_))
        );

        (stream.peek::<Const>() || stream.peek::<moxy_token::keyword::Async>()) && leads_closure && !ExprBrace::is_next(stream)
    }

    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
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
