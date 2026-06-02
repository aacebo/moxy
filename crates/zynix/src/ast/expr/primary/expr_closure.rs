use crate::ast::*;
use crate::parse::ParseStream;
use crate::token::keyword::Move;
use crate::token::punct::{Comma, Or, OrOr};
use crate::token::{Punctuation, ToTokens, Token, TokenTree};
use crate::{Span, TokenStream};

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
    pub capture: bool,
    pub inputs: Punctuated<ClosureParam, Comma>,
    pub output: ReturnType,
    pub body: Box<super::super::Expr>,
}

impl ExprClosure {
    /// Returns `true` when the stream is positioned at the start of a closure
    /// expression (`|...|`, `||`, `move`, or a `const`/`async` not followed by a block).
    pub fn is_start(stream: &mut ParseStream) -> bool {
        use crate::token::keyword::Const;
        if stream.peek::<Or>().is_some() || stream.peek::<OrOr>().is_some() || stream.peek::<Move>().is_some() {
            return true;
        }

        let leads_closure = matches!(
            stream.nth(1),
            Some(TokenTree::Token(Token::Punct(Punctuation::Or(_) | Punctuation::OrOr(_))))
                | Some(TokenTree::Token(Token::Keyword(_)))
        );

        (stream.peek::<Const>().is_some() || stream.peek::<crate::token::keyword::Async>().is_some())
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
        if self.capture {
            Move::default().to_tokens(t);
        }
        Or::default().to_tokens(t);
        self.inputs.to_tokens(t);
        Or::default().to_tokens(t);
        self.output.to_tokens(t);
        self.body.to_tokens(t);
    }
}
