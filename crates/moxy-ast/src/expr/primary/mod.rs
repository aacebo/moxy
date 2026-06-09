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
use moxy_token::keyword::{Break, Const, Continue, Let, Return, Try, Unsafe, Yield};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, DotDot, Eq, Or, OrOr, Semi};
use moxy_token::{Delim, LexError, Punctuation, Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

use super::block::{
    ExprAsync, ExprBrace, ExprConst, ExprForLoop, ExprIf, ExprLoop, ExprMatch, ExprTryBlock, ExprUnsafe, ExprWhile,
};
use super::jump::{ExprBreak, ExprContinue, ExprReturn, ExprYield};
use super::{BlockExpr, Expr, JumpExpr};
use crate::{
    Asyncness, ClosureParam, Constness, Delimited, FieldValue, Label, Movability, Pattern, Punctuated, QSelf, ReturnType,
};

/// Primary/leaf expressions (literals, paths, closures, collections, struct literals, macros).
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

impl PrimaryExpr {
    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Lit(_))
    }

    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_closure(&self) -> bool {
        matches!(self, Self::Closure(_))
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_repeat(&self) -> bool {
        matches!(self, Self::Repeat(_))
    }

    pub fn is_let(&self) -> bool {
        matches!(self, Self::Let(_))
    }

    pub fn is_paren(&self) -> bool {
        matches!(self, Self::Paren(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn as_lit(&self) -> Option<&ExprLit> {
        if let Self::Lit(v) = self { Some(v) } else { None }
    }

    pub fn as_path(&self) -> Option<&ExprPath> {
        if let Self::Path(v) = self { Some(v) } else { None }
    }

    pub fn as_struct(&self) -> Option<&ExprStruct> {
        if let Self::Struct(v) = self { Some(v) } else { None }
    }

    pub fn as_closure(&self) -> Option<&ExprClosure> {
        if let Self::Closure(v) = self { Some(v) } else { None }
    }

    pub fn as_tuple(&self) -> Option<&ExprTuple> {
        if let Self::Tuple(v) = self { Some(v) } else { None }
    }

    pub fn as_array(&self) -> Option<&ExprArray> {
        if let Self::Array(v) = self { Some(v) } else { None }
    }

    pub fn as_repeat(&self) -> Option<&ExprRepeat> {
        if let Self::Repeat(v) = self { Some(v) } else { None }
    }

    pub fn as_let(&self) -> Option<&ExprLet> {
        if let Self::Let(v) = self { Some(v) } else { None }
    }

    pub fn as_paren(&self) -> Option<&ExprParen> {
        if let Self::Paren(v) = self { Some(v) } else { None }
    }

    pub fn as_group(&self) -> Option<&ExprGroup> {
        if let Self::Group(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&ExprMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }

    pub fn into_expr(self) -> super::Expr {
        super::Expr::from(self)
    }
}

impl Spanner for PrimaryExpr {
    fn span(&self) -> Span {
        match self {
            PrimaryExpr::Lit(v) => v.span(),
            PrimaryExpr::Path(v) => v.span(),
            PrimaryExpr::Struct(v) => v.span(),
            PrimaryExpr::Closure(v) => v.span(),
            PrimaryExpr::Tuple(v) => v.span(),
            PrimaryExpr::Array(v) => v.span(),
            PrimaryExpr::Repeat(v) => v.span(),
            PrimaryExpr::Let(v) => v.span(),
            PrimaryExpr::Paren(v) => v.span(),
            PrimaryExpr::Group(v) => v.span(),
            PrimaryExpr::Macro(v) => v.span(),
        }
    }
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
        use moxy_token::keyword::Move;
        let constness = stream.parse::<Constness>()?;
        let asyncness = stream.parse::<Asyncness>()?;
        let capture = stream.parse_if::<Move>();

        let (pipes, inputs) = if stream.peek::<OrOr>() {
            let oror = stream.parse::<OrOr>()?;
            (ClosurePipes::Empty(oror), Punctuated::new())
        } else {
            let open = stream.parse::<Or>()?;
            let mut params = Punctuated::new();

            while !stream.peek::<Or>() && !stream.is_empty() {
                params.push_value(stream.parse::<ClosureParam>()?);
                if stream.peek::<Comma>() {
                    params.push_punct(stream.parse::<Comma>()?);
                } else {
                    break;
                }
            }

            let close = stream.parse::<Or>()?;
            (ClosurePipes::Params(open, close), params)
        };

        let output = stream.parse::<ReturnType>()?;
        let body = Box::new(super::parse_expr(stream, true)?);

        Ok(Self {
            attrs: Vec::new(),
            lifetimes: None,
            constness,
            movability: Movability::Movable,
            asyncness,
            capture,
            pipes,
            inputs,
            output,
            body,
        })
    }
}

impl ExprStruct {
    pub fn parse_body(
        stream: &mut ParseStream,
    ) -> Result<(Punctuated<FieldValue, Comma>, Option<(DotDot, Box<Expr>)>), ParseError> {
        let mut fields = Punctuated::new();
        let mut rest = None;

        while !stream.is_empty() {
            if stream.peek::<DotDot>() {
                let dotdot = stream.parse::<DotDot>()?;
                rest = Some((dotdot, Box::new(super::parse_expr(stream, true)?)));
                break;
            }
            fields.push_value(stream.parse::<FieldValue>()?);
            if stream.peek::<Comma>() {
                fields.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        Ok((fields, rest))
    }
}

impl ExprRepeat {
    pub fn try_parse(stream: &mut ParseStream, bracket_span: moxy_token::span::DelimSpan) -> Result<Option<Self>, ParseError> {
        let mut fork = stream.fork();
        let Ok(elem) = super::parse_expr(&mut fork, true) else {
            return Ok(None);
        };

        if !fork.peek::<Semi>() {
            return Ok(None);
        }

        let semi = fork.parse::<Semi>()?;
        let len = super::parse_expr(&mut fork, true)?;
        stream.seek(&fork);
        Ok(Some(Self {
            attrs: Vec::new(),
            content: Delimited::bracket(
                bracket_span,
                RepeatInner {
                    elem: Box::new(elem),
                    semi,
                    len: Box::new(len),
                },
            ),
        }))
    }
}

impl Expr {
    pub fn parse_if(stream: &mut ParseStream) -> Result<Option<Box<Self>>, ParseError> {
        if stream.is_empty() || stream.peek::<Semi>() || stream.peek::<Comma>() {
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
            let (paren_span, group_tokens) = stream.parse_group_spanned(Delim::Paren)?;
            let mut inner = group_tokens.parse();
            let elems: Punctuated<Expr, Comma> = Punctuated::parse_terminated(&mut inner)?;
            return Ok(if elems.len() == 1 && !elems.is_trailing() {
                let expr = Box::new(elems.into_iter().next().unwrap());
                Expr::Primary(PrimaryExpr::Paren(ExprParen {
                    attrs: Vec::new(),
                    content: Delimited::paren(paren_span, expr),
                }))
            } else {
                Expr::Primary(PrimaryExpr::Tuple(ExprTuple {
                    attrs: Vec::new(),
                    elems: Delimited::paren(paren_span, elems),
                }))
            });
        }

        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
            let (bracket_span, group_tokens) = stream.parse_group_spanned(Delim::Bracket)?;
            let mut inner = group_tokens.parse();
            if let Some(rep) = ExprRepeat::try_parse(&mut inner, bracket_span)? {
                return Ok(Expr::Primary(PrimaryExpr::Repeat(rep)));
            }
            let elems = Punctuated::parse_terminated(&mut inner)?;
            return Ok(Expr::Primary(PrimaryExpr::Array(ExprArray {
                attrs: Vec::new(),
                elems: Delimited::bracket(bracket_span, elems),
            })));
        }

        // Labeled block / loop: `'a: { }`, `'a: loop { }`, etc.
        if Label::is_prefix(stream) {
            let label = Some(stream.parse::<Label>()?);

            if stream.peek::<moxy_token::keyword::While>() {
                return Ok(Expr::Block(BlockExpr::While(ExprWhile::parse_from(stream, label)?)));
            }

            if stream.peek::<moxy_token::keyword::For>() {
                return Ok(Expr::Block(BlockExpr::ForLoop(ExprForLoop::parse_from(stream, label)?)));
            }

            if stream.peek::<moxy_token::keyword::Loop>() {
                return Ok(Expr::Block(BlockExpr::Loop(ExprLoop::parse_from(stream, label)?)));
            }

            return Ok(Expr::Block(BlockExpr::Brace(ExprBrace {
                attrs: Vec::new(),
                label,
                block: stream.parse()?,
            })));
        }

        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
            return Ok(Expr::Block(BlockExpr::Brace(ExprBrace {
                attrs: Vec::new(),
                label: None,
                block: stream.parse()?,
            })));
        }

        if stream.peek::<moxy_token::keyword::If>() {
            return ExprIf::parse_from(stream);
        }

        if stream.peek::<moxy_token::keyword::While>() {
            return Ok(Expr::Block(BlockExpr::While(ExprWhile::parse_from(stream, None)?)));
        }

        if stream.peek::<moxy_token::keyword::For>() {
            return Ok(Expr::Block(BlockExpr::ForLoop(ExprForLoop::parse_from(stream, None)?)));
        }

        if stream.peek::<moxy_token::keyword::Loop>() {
            return Ok(Expr::Block(BlockExpr::Loop(ExprLoop::parse_from(stream, None)?)));
        }

        if stream.peek::<moxy_token::keyword::Match>() {
            return ExprMatch::parse_from(stream);
        }

        if stream.peek::<Unsafe>() {
            return Ok(Expr::Block(BlockExpr::Unsafe(ExprUnsafe::parse_from(stream)?)));
        }

        if stream.peek::<Const>() && ExprBrace::is_next(stream) {
            return Ok(Expr::Block(BlockExpr::Const(ExprConst::parse_from(stream)?)));
        }

        if stream.peek::<moxy_token::keyword::Async>() && ExprAsync::is_block(stream) {
            return Ok(Expr::Block(BlockExpr::Async(ExprAsync::parse_from(stream)?)));
        }

        if stream.peek::<Try>() && ExprBrace::is_next(stream) {
            return Ok(Expr::Block(BlockExpr::TryBlock(ExprTryBlock::parse_from(stream)?)));
        }

        if stream.peek::<Return>() {
            let return_keyword = stream.parse::<Return>()?;
            return Ok(Expr::Jump(JumpExpr::Return(ExprReturn {
                attrs: Vec::new(),
                return_keyword,
                expr: Expr::parse_if(stream)?,
            })));
        }

        if stream.peek::<Yield>() {
            let yield_keyword = stream.parse::<Yield>()?;
            return Ok(Expr::Jump(JumpExpr::Yield(ExprYield {
                attrs: Vec::new(),
                yield_keyword,
                expr: Expr::parse_if(stream)?,
            })));
        }

        if stream.peek::<Break>() {
            let break_keyword = stream.parse::<Break>()?;
            let label = Label::parse_opt_break(stream);
            return Ok(Expr::Jump(JumpExpr::Break(ExprBreak {
                attrs: Vec::new(),
                break_keyword,
                label,
                expr: Expr::parse_if(stream)?,
            })));
        }

        if stream.peek::<Continue>() {
            let continue_keyword = stream.parse::<Continue>()?;
            let label = Label::parse_opt_break(stream);
            return Ok(Expr::Jump(JumpExpr::Continue(ExprContinue {
                attrs: Vec::new(),
                continue_keyword,
                label,
            })));
        }

        if stream.peek::<Let>() {
            let let_keyword = stream.parse::<Let>()?;
            let pat = Box::new(stream.parse::<Pattern>()?);
            let eq = stream.parse::<Eq>()?;
            let expr = Box::new(super::parse_expr(stream, false)?);
            return Ok(Expr::Primary(PrimaryExpr::Let(ExprLet {
                attrs: Vec::new(),
                let_keyword,
                pat,
                eq,
                expr,
            })));
        }

        if ExprClosure::is_start(stream) {
            return Ok(Expr::Primary(PrimaryExpr::Closure(ExprClosure::parse_from(stream)?)));
        }

        if matches!(stream.curr(), Some(tt) if ExprLit::is_literal(tt)) || ExprLit::is_bool_ident(stream) {
            return Ok(Expr::Primary(PrimaryExpr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: stream.parse()?,
            })));
        }

        if let Some(mac) = stream.parse_if::<crate::MacroCall>() {
            return Ok(Expr::Primary(PrimaryExpr::Macro(ExprMacro { attrs: Vec::new(), mac })));
        }

        // Qualified path `<T as Trait>::assoc` in expression position.
        if stream.peek::<moxy_token::punct::Lt>() {
            let (qself, path) = crate::ty::QSelf::parse_qualified(stream)?;
            return Ok(Expr::Primary(PrimaryExpr::Path(ExprPath {
                attrs: Vec::new(),
                qself: Some(qself),
                path,
            })));
        }

        if matches!(
            stream.curr(),
            Some(TokenTree::Ident(_) | TokenTree::Keyword(_) | TokenTree::Punct(Punctuation::PathSep(_)))
        ) {
            use crate::Path;
            let path = stream.parse::<Path>()?;

            if allow_struct && matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
                let body = Delimited::parse_brace_with(stream, |inner| {
                    let (fields, rest) = ExprStruct::parse_body(inner)?;
                    Ok(StructBody { fields, rest })
                })?;
                return Ok(Expr::Primary(PrimaryExpr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path,
                    body,
                })));
            }

            return Ok(Expr::Primary(PrimaryExpr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path,
            })));
        }

        Err(LexError::new(at).message("expected expression").into())
    }
}
