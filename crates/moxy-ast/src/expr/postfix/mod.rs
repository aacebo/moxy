mod expr_await;
mod expr_call;
mod expr_field;
mod expr_index;
mod expr_method_call;

pub use expr_await::*;
pub use expr_call::*;
pub use expr_field::*;
pub use expr_index::*;
pub use expr_method_call::*;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Dot, Question};
use moxy_token::{Delim, Span, ToTokens, TokenStream};

use super::unary::ExprTry;
use super::{Expr, UnaryExpr};
use crate::{Delimited, Member, Punctuated};

#[doc = "Postfix/suffix expressions (calls, field access, indexing, await, try-propagation)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PostfixExpr {
    Call(ExprCall),
    MethodCall(ExprMethodCall),
    Field(ExprField),
    Index(ExprIndex),
    Await(ExprAwait),
}

impl ToTokens for PostfixExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            PostfixExpr::Call(v) => v.to_tokens(t),
            PostfixExpr::MethodCall(v) => v.to_tokens(t),
            PostfixExpr::Field(v) => v.to_tokens(t),
            PostfixExpr::Index(v) => v.to_tokens(t),
            PostfixExpr::Await(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprCall> for PostfixExpr {
    fn from(v: ExprCall) -> Self {
        PostfixExpr::Call(v)
    }
}

impl From<ExprMethodCall> for PostfixExpr {
    fn from(v: ExprMethodCall) -> Self {
        PostfixExpr::MethodCall(v)
    }
}

impl From<ExprField> for PostfixExpr {
    fn from(v: ExprField) -> Self {
        PostfixExpr::Field(v)
    }
}

impl From<ExprIndex> for PostfixExpr {
    fn from(v: ExprIndex) -> Self {
        PostfixExpr::Index(v)
    }
}

impl From<ExprAwait> for PostfixExpr {
    fn from(v: ExprAwait) -> Self {
        PostfixExpr::Await(v)
    }
}

// Parser

impl PostfixExpr {
    pub fn parse_from(stream: &mut ParseStream, mut expr: Expr) -> Result<Expr, ParseError> {
        loop {
            if stream.peek::<Dot>().is_some() {
                let dot = stream.parse::<Dot>()?;

                if matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("await")) {
                    let await_span = stream.span();
                    stream.advance();
                    expr = Expr::Postfix(PostfixExpr::Await(ExprAwait {
                        span: Span::default(),
                        attrs: Vec::new(),
                        base: Box::new(expr),
                        dot,
                        await_keyword: moxy_token::keyword::Await::new(await_span),
                    }));
                    continue;
                }

                let member = stream.parse::<Member>()?;

                if let Member::Named(method) = &member {
                    // Optional turbofish `::<...>` before the call parens.
                    let turbofish = ExprMethodCall::parse_turbofish(stream)?;

                    if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
                        let method = method.clone();
                        let paren = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
                        expr = Expr::Postfix(PostfixExpr::MethodCall(ExprMethodCall {
                            span: Span::default(),
                            attrs: Vec::new(),
                            receiver: Box::new(expr),
                            dot,
                            method,
                            turbofish,
                            paren,
                        }));
                        continue;
                    }
                }

                expr = Expr::Postfix(PostfixExpr::Field(ExprField {
                    span: Span::default(),
                    attrs: Vec::new(),
                    base: Box::new(expr),
                    dot,
                    member,
                }));
                continue;
            }

            if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
                let paren = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
                expr = Expr::Postfix(PostfixExpr::Call(ExprCall {
                    span: Span::default(),
                    attrs: Vec::new(),
                    func: Box::new(expr),
                    paren,
                }));
                continue;
            }

            if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
                let bracket = Delimited::parse_bracket_with(stream, |s| super::parse_expr(s, true).map(Box::new))?;
                expr = Expr::Postfix(PostfixExpr::Index(ExprIndex {
                    span: Span::default(),
                    attrs: Vec::new(),
                    base: Box::new(expr),
                    bracket,
                }));
                continue;
            }

            if stream.peek::<Question>().is_some() {
                let question_punct = stream.parse::<Question>()?;
                expr = Expr::Unary(UnaryExpr::Try(ExprTry {
                    span: Span::default(),
                    attrs: Vec::new(),
                    expr: Box::new(expr),
                    question_punct,
                }));
                continue;
            }

            break;
        }

        Ok(expr)
    }
}
