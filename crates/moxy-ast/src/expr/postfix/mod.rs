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
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Dot, Question};
use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use super::unary::ExprTry;
use super::{Expr, UnaryExpr};
use crate::{Delimited, Member, Punctuated};

/// Postfix/suffix expressions (calls, field access, indexing, await, try-propagation).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PostfixExpr {
    Call(ExprCall),
    MethodCall(ExprMethodCall),
    Field(ExprField),
    Index(ExprIndex),
    Await(ExprAwait),
}

impl PostfixExpr {
    pub fn is_call(&self) -> bool {
        matches!(self, Self::Call(_))
    }

    pub fn is_method_call(&self) -> bool {
        matches!(self, Self::MethodCall(_))
    }

    pub fn is_field(&self) -> bool {
        matches!(self, Self::Field(_))
    }

    pub fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }

    pub fn is_await(&self) -> bool {
        matches!(self, Self::Await(_))
    }

    pub fn as_call(&self) -> Option<&ExprCall> {
        if let Self::Call(v) = self { Some(v) } else { None }
    }

    pub fn as_method_call(&self) -> Option<&ExprMethodCall> {
        if let Self::MethodCall(v) = self { Some(v) } else { None }
    }

    pub fn as_field(&self) -> Option<&ExprField> {
        if let Self::Field(v) = self { Some(v) } else { None }
    }

    pub fn as_index(&self) -> Option<&ExprIndex> {
        if let Self::Index(v) = self { Some(v) } else { None }
    }

    pub fn as_await(&self) -> Option<&ExprAwait> {
        if let Self::Await(v) = self { Some(v) } else { None }
    }

    pub fn into_expr(self) -> super::Expr {
        super::Expr::from(self)
    }
}

impl Spanner for PostfixExpr {
    fn span(&self) -> Span {
        match self {
            PostfixExpr::Call(v) => v.span(),
            PostfixExpr::MethodCall(v) => v.span(),
            PostfixExpr::Field(v) => v.span(),
            PostfixExpr::Index(v) => v.span(),
            PostfixExpr::Await(v) => v.span(),
        }
    }
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
            if stream.peek::<Dot>() {
                let dot = stream.parse::<Dot>()?;

                if matches!(stream.curr(), Some(tt) if tt.text() == Some("await")) {
                    let await_span = stream.span();
                    stream.advance();
                    expr = Expr::Postfix(PostfixExpr::Await(ExprAwait {
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
                        let args = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
                        expr = Expr::Postfix(PostfixExpr::MethodCall(ExprMethodCall {
                            attrs: Vec::new(),
                            receiver: Box::new(expr),
                            dot,
                            method,
                            turbofish,
                            args,
                        }));
                        continue;
                    }
                }

                expr = Expr::Postfix(PostfixExpr::Field(ExprField {
                    attrs: Vec::new(),
                    base: Box::new(expr),
                    dot,
                    member,
                }));

                continue;
            }

            if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
                let args = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
                expr = Expr::Postfix(PostfixExpr::Call(ExprCall {
                    attrs: Vec::new(),
                    func: Box::new(expr),
                    args,
                }));

                continue;
            }

            if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
                let index = Delimited::parse_bracket_with(stream, |s| super::parse_expr(s, true).map(Box::new))?;
                expr = Expr::Postfix(PostfixExpr::Index(ExprIndex {
                    attrs: Vec::new(),
                    base: Box::new(expr),
                    index,
                }));

                continue;
            }

            if stream.peek::<Question>() {
                let question_punct = stream.parse::<Question>()?;
                expr = Expr::Unary(UnaryExpr::Try(ExprTry {
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
