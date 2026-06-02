mod expr_array;
mod expr_closure;
mod expr_group;
mod expr_infer;
mod expr_let;
mod expr_lit;
mod expr_macro;
mod expr_paren;
mod expr_path;
mod expr_repeat;
mod expr_struct;
mod expr_tuple;

pub use expr_array::*;
pub use expr_closure::*;
pub use expr_group::*;
pub use expr_let::*;
pub use expr_lit::*;
pub use expr_macro::*;
pub use expr_paren::*;
pub use expr_path::*;
pub use expr_repeat::*;
pub use expr_struct::*;
pub use expr_tuple::*;

use super::block::{
    ExprAsync, ExprBrace, ExprConst, ExprForLoop, ExprIf, ExprLoop, ExprMatch, ExprTryBlock, ExprUnsafe, ExprWhile,
};
use super::jump::{ExprBreak, ExprContinue, ExprReturn, ExprYield};
use super::{BlockExpr, Expr, JumpExpr};
use crate::Span;
use crate::ast::{Asyncness, ClosureParam, Constness, FieldValue, Label, Movability, Pattern, Punctuated, QSelf, ReturnType};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Break, Const, Continue, Let, Return, Unsafe, Yield};
use crate::token::punct::{Comma, Eq, Or, OrOr, Semi};
use crate::token::{Delim, LexError, Punctuation, ToTokens, Token, TokenStream, TokenTree};

#[doc = "Primary/leaf expressions (literals, paths, closures, collections, struct literals, macros)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PrimaryExpr {
    Lit(ExprLit),
    Path(ExprPath),
    Struct(ExprStruct),
    Closure(ExprClosure),
    Tuple(ExprTuple),
    Array(ExprArray),
    Repeat(ExprRepeat),
    Let(ExprLet),
    Paren(ExprParen),
    Group(ExprGroup),
    Macro(ExprMacro),
}

impl ToTokens for PrimaryExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            PrimaryExpr::Lit(v) => v.to_tokens(t),
            PrimaryExpr::Path(v) => v.to_tokens(t),
            PrimaryExpr::Struct(v) => v.to_tokens(t),
            PrimaryExpr::Closure(v) => v.to_tokens(t),
            PrimaryExpr::Tuple(v) => v.to_tokens(t),
            PrimaryExpr::Array(v) => v.to_tokens(t),
            PrimaryExpr::Repeat(v) => v.to_tokens(t),
            PrimaryExpr::Let(v) => v.to_tokens(t),
            PrimaryExpr::Paren(v) => v.to_tokens(t),
            PrimaryExpr::Group(v) => v.to_tokens(t),
            PrimaryExpr::Macro(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprLit> for PrimaryExpr {
    fn from(v: ExprLit) -> Self {
        PrimaryExpr::Lit(v)
    }
}

impl From<ExprPath> for PrimaryExpr {
    fn from(v: ExprPath) -> Self {
        PrimaryExpr::Path(v)
    }
}

impl From<ExprStruct> for PrimaryExpr {
    fn from(v: ExprStruct) -> Self {
        PrimaryExpr::Struct(v)
    }
}

impl From<ExprClosure> for PrimaryExpr {
    fn from(v: ExprClosure) -> Self {
        PrimaryExpr::Closure(v)
    }
}

impl From<ExprTuple> for PrimaryExpr {
    fn from(v: ExprTuple) -> Self {
        PrimaryExpr::Tuple(v)
    }
}

impl From<ExprArray> for PrimaryExpr {
    fn from(v: ExprArray) -> Self {
        PrimaryExpr::Array(v)
    }
}

impl From<ExprRepeat> for PrimaryExpr {
    fn from(v: ExprRepeat) -> Self {
        PrimaryExpr::Repeat(v)
    }
}

impl From<ExprLet> for PrimaryExpr {
    fn from(v: ExprLet) -> Self {
        PrimaryExpr::Let(v)
    }
}

impl From<ExprParen> for PrimaryExpr {
    fn from(v: ExprParen) -> Self {
        PrimaryExpr::Paren(v)
    }
}

impl From<ExprGroup> for PrimaryExpr {
    fn from(v: ExprGroup) -> Self {
        PrimaryExpr::Group(v)
    }
}

impl From<ExprMacro> for PrimaryExpr {
    fn from(v: ExprMacro) -> Self {
        PrimaryExpr::Macro(v)
    }
}

// Parser

impl ExprClosure {
    pub fn parse_from(stream: &mut ParseStream) -> Result<Self, ParseError> {
        use crate::token::keyword::Move;
        let constness = stream.parse::<Constness>()?;
        let asyncness = stream.parse::<Asyncness>()?;
        let capture = if stream.peek::<Move>().is_some() {
            let _ = stream.parse::<Move>()?;
            true
        } else {
            false
        };

        let inputs = if stream.peek::<OrOr>().is_some() {
            let _ = stream.parse::<OrOr>()?;
            Punctuated::new()
        } else {
            let _ = stream.parse::<Or>()?;
            let mut params = Punctuated::new();

            while stream.peek::<Or>().is_none() && !stream.is_empty() {
                params.push_value(stream.parse::<ClosureParam>()?);
                if stream.peek::<Comma>().is_some() {
                    params.push_punct(stream.parse::<Comma>()?);
                } else {
                    break;
                }
            }

            let _ = stream.parse::<Or>()?;
            params
        };

        let output = stream.parse::<ReturnType>()?;
        let body = Box::new(super::parse_expr(stream, true)?);

        Ok(Self {
            span: Span::default(),
            attrs: Vec::new(),
            lifetimes: None,
            constness,
            movability: Movability::Movable,
            asyncness,
            capture,
            inputs,
            output,
            body,
        })
    }
}

impl ExprStruct {
    #[allow(clippy::type_complexity)]
    pub fn parse_body(stream: &mut ParseStream) -> Result<(Punctuated<FieldValue, Comma>, Option<Box<Expr>>), ParseError> {
        use crate::token::punct::DotDot;
        let mut fields = Punctuated::new();
        let mut rest = None;

        while !stream.is_empty() {
            if stream.peek::<DotDot>().is_some() {
                let _ = stream.parse::<DotDot>()?;
                rest = Some(Box::new(super::parse_expr(stream, true)?));
                break;
            }
            fields.push_value(stream.parse::<FieldValue>()?);
            if stream.peek::<Comma>().is_some() {
                fields.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        Ok((fields, rest))
    }
}

impl ExprRepeat {
    pub fn try_parse(stream: &mut ParseStream) -> Result<Option<Self>, ParseError> {
        let mut fork = stream.fork();
        let Ok(elem) = super::parse_expr(&mut fork, true) else {
            return Ok(None);
        };

        if fork.peek::<Semi>().is_none() {
            return Ok(None);
        }

        let _ = fork.parse::<Semi>()?;
        let len = super::parse_expr(&mut fork, true)?;
        stream.seek(&fork);
        Ok(Some(Self {
            span: Span::default(),
            attrs: Vec::new(),
            elem: Box::new(elem),
            len: Box::new(len),
        }))
    }
}

impl Expr {
    pub fn parse_opt(stream: &mut ParseStream) -> Result<Option<Box<Self>>, ParseError> {
        if stream.is_empty() || stream.peek::<Semi>().is_some() || stream.peek::<Comma>().is_some() {
            return Ok(None);
        }
        let mut fork = stream.fork();

        match super::parse_expr(&mut fork, true) {
            Ok(e) => {
                stream.seek(&fork);
                Ok(Some(Box::new(e)))
            }
            Err(_) => Ok(None),
        }
    }
}

impl PrimaryExpr {
    pub fn parse_from(stream: &mut ParseStream, allow_struct: bool) -> Result<Expr, ParseError> {
        let at = stream.span();

        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let group = stream.parse_group(Delim::Paren)?;
            let mut inner = group.parse();
            let elems: Punctuated<Expr, Comma> = Punctuated::parse_terminated(&mut inner)?;
            return Ok(if elems.len() == 1 && !elems.trailing_punct() {
                Expr::Primary(PrimaryExpr::Paren(ExprParen {
                    span: Span::default(),
                    attrs: Vec::new(),
                    expr: Box::new(elems.into_iter().next().unwrap()),
                }))
            } else {
                Expr::Primary(PrimaryExpr::Tuple(ExprTuple {
                    span: Span::default(),
                    attrs: Vec::new(),
                    elems,
                }))
            });
        }

        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
            let group = stream.parse_group(Delim::Bracket)?;
            let mut inner = group.parse();
            if let Some(rep) = ExprRepeat::try_parse(&mut inner)? {
                return Ok(Expr::Primary(PrimaryExpr::Repeat(rep)));
            }
            let elems = Punctuated::parse_terminated(&mut inner)?;
            return Ok(Expr::Primary(PrimaryExpr::Array(ExprArray {
                span: Span::default(),
                attrs: Vec::new(),
                elems,
            })));
        }

        // Labeled block / loop: `'a: { }`, `'a: loop { }`, etc.
        if Label::is_prefix(stream) {
            let label = Some(stream.parse::<Label>()?);

            if stream.peek::<crate::token::keyword::While>().is_some() {
                return Ok(Expr::Block(BlockExpr::While(ExprWhile::parse_from(stream, label)?)));
            }

            if stream.peek::<crate::token::keyword::For>().is_some() {
                return Ok(Expr::Block(BlockExpr::ForLoop(ExprForLoop::parse_from(stream, label)?)));
            }

            if stream.peek::<crate::token::keyword::Loop>().is_some() {
                return Ok(Expr::Block(BlockExpr::Loop(ExprLoop::parse_from(stream, label)?)));
            }

            return Ok(Expr::Block(BlockExpr::Brace(ExprBrace {
                span: Span::default(),
                attrs: Vec::new(),
                label,
                block: stream.parse()?,
            })));
        }

        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
            return Ok(Expr::Block(BlockExpr::Brace(ExprBrace {
                span: Span::default(),
                attrs: Vec::new(),
                label: None,
                block: stream.parse()?,
            })));
        }

        if stream.peek::<crate::token::keyword::If>().is_some() {
            return ExprIf::parse_from(stream);
        }

        if stream.peek::<crate::token::keyword::While>().is_some() {
            return Ok(Expr::Block(BlockExpr::While(ExprWhile::parse_from(stream, None)?)));
        }

        if stream.peek::<crate::token::keyword::For>().is_some() {
            return Ok(Expr::Block(BlockExpr::ForLoop(ExprForLoop::parse_from(stream, None)?)));
        }

        if stream.peek::<crate::token::keyword::Loop>().is_some() {
            return Ok(Expr::Block(BlockExpr::Loop(ExprLoop::parse_from(stream, None)?)));
        }

        if stream.peek::<crate::token::keyword::Match>().is_some() {
            return ExprMatch::parse_from(stream);
        }

        if stream.peek::<Unsafe>().is_some() {
            let _ = stream.parse::<Unsafe>()?;
            return Ok(Expr::Block(BlockExpr::Unsafe(ExprUnsafe {
                span: Span::default(),
                attrs: Vec::new(),
                block: stream.parse()?,
            })));
        }

        // `const { }` block (vs `const` closure, which has `|`/`move` next).
        if stream.peek::<Const>().is_some() && ExprBrace::is_next(stream) {
            let _ = stream.parse::<Const>()?;
            return Ok(Expr::Block(BlockExpr::Const(ExprConst {
                span: Span::default(),
                attrs: Vec::new(),
                block: stream.parse()?,
            })));
        }

        // `async { }` / `async move { }` block (vs `async` closure).
        if stream.peek::<crate::token::keyword::Async>().is_some() && ExprAsync::is_block(stream) {
            use crate::token::keyword::Move;
            let _ = stream.parse::<crate::token::keyword::Async>()?;

            let capture = if stream.peek::<Move>().is_some() {
                let _ = stream.parse::<Move>()?;
                true
            } else {
                false
            };

            return Ok(Expr::Block(BlockExpr::Async(ExprAsync {
                span: Span::default(),
                attrs: Vec::new(),
                capture,
                block: stream.parse()?,
            })));
        }

        // `try { }` block.
        if matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("try")) && ExprBrace::is_next(stream) {
            stream.advance();
            return Ok(Expr::Block(BlockExpr::TryBlock(ExprTryBlock {
                span: Span::default(),
                attrs: Vec::new(),
                block: stream.parse()?,
            })));
        }

        if stream.peek::<Return>().is_some() {
            let _ = stream.parse::<Return>()?;
            return Ok(Expr::Jump(JumpExpr::Return(ExprReturn {
                span: Span::default(),
                attrs: Vec::new(),
                expr: Expr::parse_opt(stream)?,
            })));
        }

        if stream.peek::<Yield>().is_some() {
            let _ = stream.parse::<Yield>()?;
            return Ok(Expr::Jump(JumpExpr::Yield(ExprYield {
                span: Span::default(),
                attrs: Vec::new(),
                expr: Expr::parse_opt(stream)?,
            })));
        }

        if stream.peek::<Break>().is_some() {
            let _ = stream.parse::<Break>()?;
            let label = Label::parse_opt_break(stream);
            return Ok(Expr::Jump(JumpExpr::Break(ExprBreak {
                span: Span::default(),
                attrs: Vec::new(),
                label,
                expr: Expr::parse_opt(stream)?,
            })));
        }

        if stream.peek::<Continue>().is_some() {
            let _ = stream.parse::<Continue>()?;
            let label = Label::parse_opt_break(stream);
            return Ok(Expr::Jump(JumpExpr::Continue(ExprContinue {
                span: Span::default(),
                attrs: Vec::new(),
                label,
            })));
        }

        if stream.peek::<Let>().is_some() {
            let _ = stream.parse::<Let>()?;
            let pat = Box::new(stream.parse::<Pattern>()?);
            let _ = stream.parse::<Eq>()?;
            let expr = Box::new(super::parse_expr(stream, false)?);
            return Ok(Expr::Primary(PrimaryExpr::Let(ExprLet {
                span: Span::default(),
                attrs: Vec::new(),
                pat,
                expr,
            })));
        }

        if ExprClosure::is_start(stream) {
            return Ok(Expr::Primary(PrimaryExpr::Closure(ExprClosure::parse_from(stream)?)));
        }

        if matches!(stream.curr(), Some(tt) if ExprLit::is_literal(tt)) || ExprLit::is_bool_ident(stream) {
            return Ok(Expr::Primary(PrimaryExpr::Lit(ExprLit {
                span: Span::default(),
                attrs: Vec::new(),
                lit: stream.parse()?,
            })));
        }

        if let Some(mac) = stream.parse_opt::<crate::ast::MacroCall>() {
            return Ok(Expr::Primary(PrimaryExpr::Macro(ExprMacro {
                span: crate::Span::default(),
                attrs: Vec::new(),
                mac,
            })));
        }

        // Qualified path `<T as Trait>::assoc` in expression position.
        if stream.peek::<crate::token::punct::Lt>().is_some() {
            let (qself, path) = crate::ast::ty::QSelf::parse_qualified(stream)?;
            return Ok(Expr::Primary(PrimaryExpr::Path(ExprPath {
                span: Span::default(),
                attrs: Vec::new(),
                qself: Some(qself),
                path,
            })));
        }

        if matches!(
            stream.curr(),
            Some(
                TokenTree::Token(Token::Ident(_))
                    | TokenTree::Token(Token::Keyword(_))
                    | TokenTree::Token(Token::Punct(Punctuation::PathSep(_)))
            )
        ) {
            use crate::ast::Path;
            let path = stream.parse::<Path>()?;

            if allow_struct && matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
                let group = stream.parse_group(Delim::Brace)?;
                let mut inner = group.parse();
                let (fields, rest) = ExprStruct::parse_body(&mut inner)?;
                return Ok(Expr::Primary(PrimaryExpr::Struct(ExprStruct {
                    span: Span::default(),
                    attrs: Vec::new(),
                    qself: None,
                    path,
                    fields,
                    rest,
                })));
            }

            return Ok(Expr::Primary(PrimaryExpr::Path(ExprPath {
                span: Span::default(),
                attrs: Vec::new(),
                qself: None,
                path,
            })));
        }

        Err(LexError::new(at).message("expected expression").into())
    }
}
