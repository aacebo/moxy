use moxy_token::Token;
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
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Delim, LexError, Punctuation, Span, Spanner, ToTokens, TokenStream, TokenTree};

use super::block::{
    ExprAsync, ExprBrace, ExprConst, ExprForLoop, ExprIf, ExprLoop, ExprMatch, ExprTryBlock, ExprUnsafe, ExprWhile,
};
use super::jump::{ExprBreak, ExprContinue, ExprReturn, ExprYield};
use super::{BlockExpr, Expr, JumpExpr};
use crate::{
    Asyncness, Attributes, ClosureParam, Constness, Delimited, FieldValue, Label, Movability, Pattern, Punctuated, ReturnType,
};

/// Primary/leaf expressions (literals, paths, closures, collections, struct literals, macros).
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn attrs(&self) -> &Attributes {
        match self {
            Self::Lit(v) => &v.attrs,
            Self::Path(v) => &v.attrs,
            Self::Struct(v) => &v.attrs,
            Self::Closure(v) => &v.attrs,
            Self::Tuple(v) => &v.attrs,
            Self::Array(v) => &v.attrs,
            Self::Repeat(v) => &v.attrs,
            Self::Let(v) => &v.attrs,
            Self::Paren(v) => &v.attrs,
            Self::Group(v) => &v.attrs,
            Self::Macro(v) => &v.attrs,
        }
    }

    pub fn attrs_mut(&mut self) -> &mut Attributes {
        match self {
            Self::Lit(v) => &mut v.attrs,
            Self::Path(v) => &mut v.attrs,
            Self::Struct(v) => &mut v.attrs,
            Self::Closure(v) => &mut v.attrs,
            Self::Tuple(v) => &mut v.attrs,
            Self::Array(v) => &mut v.attrs,
            Self::Repeat(v) => &mut v.attrs,
            Self::Let(v) => &mut v.attrs,
            Self::Paren(v) => &mut v.attrs,
            Self::Group(v) => &mut v.attrs,
            Self::Macro(v) => &mut v.attrs,
        }
    }

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
            Self::Lit(v) => v.span(),
            Self::Path(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Closure(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::Array(v) => v.span(),
            Self::Repeat(v) => v.span(),
            Self::Let(v) => v.span(),
            Self::Paren(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Macro(v) => v.span(),
        }
    }
}

impl ToTokens for PrimaryExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Lit(v) => v.to_tokens(t),
            Self::Path(v) => v.to_tokens(t),
            Self::Struct(v) => v.to_tokens(t),
            Self::Closure(v) => v.to_tokens(t),
            Self::Tuple(v) => v.to_tokens(t),
            Self::Array(v) => v.to_tokens(t),
            Self::Repeat(v) => v.to_tokens(t),
            Self::Let(v) => v.to_tokens(t),
            Self::Paren(v) => v.to_tokens(t),
            Self::Group(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprLit> for PrimaryExpr {
    fn from(v: ExprLit) -> Self {
        Self::Lit(v)
    }
}

impl From<ExprPath> for PrimaryExpr {
    fn from(v: ExprPath) -> Self {
        Self::Path(v)
    }
}

impl From<ExprStruct> for PrimaryExpr {
    fn from(v: ExprStruct) -> Self {
        Self::Struct(v)
    }
}

impl From<ExprClosure> for PrimaryExpr {
    fn from(v: ExprClosure) -> Self {
        Self::Closure(v)
    }
}

impl From<ExprTuple> for PrimaryExpr {
    fn from(v: ExprTuple) -> Self {
        Self::Tuple(v)
    }
}

impl From<ExprArray> for PrimaryExpr {
    fn from(v: ExprArray) -> Self {
        Self::Array(v)
    }
}

impl From<ExprRepeat> for PrimaryExpr {
    fn from(v: ExprRepeat) -> Self {
        Self::Repeat(v)
    }
}

impl From<ExprLet> for PrimaryExpr {
    fn from(v: ExprLet) -> Self {
        Self::Let(v)
    }
}

impl From<ExprParen> for PrimaryExpr {
    fn from(v: ExprParen) -> Self {
        Self::Paren(v)
    }
}

impl From<ExprGroup> for PrimaryExpr {
    fn from(v: ExprGroup) -> Self {
        Self::Group(v)
    }
}

impl From<ExprMacro> for PrimaryExpr {
    fn from(v: ExprMacro) -> Self {
        Self::Macro(v)
    }
}

// Parser

impl ExprClosure {
    pub fn parse_from(stream: &mut ParseStream, attrs: Attributes) -> Result<Self, ParseError> {
        let constness = stream.parse::<Constness>()?;
        let asyncness = stream.parse::<Asyncness>()?;
        let capture = stream.parse_if::<Token![move]>();
        let (pipes, inputs) = if stream.peek::<Token![||]>() {
            let oror = stream.parse::<Token![||]>()?;
            (ClosurePipes::Empty(oror), Punctuated::new())
        } else {
            let open = stream.parse::<Token![|]>()?;
            let mut params = Punctuated::new();

            while !stream.peek::<Token![|]>() && !stream.is_empty() {
                params.push_value(stream.parse::<ClosureParam>()?);
                if stream.peek::<Token![,]>() {
                    params.push_punct(stream.parse::<Token![,]>()?);
                } else {
                    break;
                }
            }

            let close = stream.parse::<Token![|]>()?;
            (ClosurePipes::Params(open, close), params)
        };

        let output = stream.parse::<ReturnType>()?;
        let body = Box::new(super::parse_expr(stream, true)?);

        Ok(Self {
            attrs,
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
    ) -> Result<(Punctuated<FieldValue, Token![,]>, Option<(Token![..], Box<Expr>)>), ParseError> {
        let mut fields = Punctuated::new();
        let mut rest = None;

        while !stream.is_empty() {
            if stream.peek::<Token![..]>() {
                let dotdot = stream.parse::<Token![..]>()?;
                rest = Some((dotdot, Box::new(super::parse_expr(stream, true)?)));
                break;
            }

            fields.push_value(stream.parse::<FieldValue>()?);

            if stream.peek::<Token![,]>() {
                fields.push_punct(stream.parse::<Token![,]>()?);
            } else {
                break;
            }
        }

        Ok((fields, rest))
    }
}

impl ExprRepeat {
    pub fn try_parse(
        stream: &mut ParseStream,
        bracket_span: moxy_token::span::DelimSpan,
        attrs: Attributes,
    ) -> Result<Option<Self>, ParseError> {
        let mut fork = stream.fork();
        let Ok(elem) = super::parse_expr(&mut fork, true) else {
            return Ok(None);
        };

        if !fork.peek::<Token![;]>() {
            return Ok(None);
        }

        let semi = fork.parse::<Token![;]>()?;
        let len = super::parse_expr(&mut fork, true)?;
        stream.seek(&fork);

        Ok(Some(Self {
            attrs,
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
        if stream.is_empty() || stream.peek::<Token![;]>() || stream.peek::<Token![,]>() {
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
    pub fn parse_from(stream: &mut ParseStream, allow_struct: bool, attrs: Attributes) -> Result<Expr, ParseError> {
        let at = stream.span();

        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let (paren_span, group_tokens) = stream.parse_group_spanned(Delim::Paren)?;
            let mut inner = group_tokens.parse();
            let elems: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(&mut inner)?;

            return Ok(if elems.len() == 1 && !elems.is_trailing() {
                let expr = Box::new(elems.into_iter().next().unwrap());
                Expr::Primary(Self::Paren(ExprParen {
                    attrs,
                    content: Delimited::paren(paren_span, expr),
                }))
            } else {
                Expr::Primary(Self::Tuple(ExprTuple {
                    attrs,
                    elems: Delimited::paren(paren_span, elems),
                }))
            });
        }

        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
            let (bracket_span, group_tokens) = stream.parse_group_spanned(Delim::Bracket)?;
            let mut inner = group_tokens.parse();

            if let Some(rep) = ExprRepeat::try_parse(&mut inner, bracket_span, attrs.clone())? {
                return Ok(Expr::Primary(Self::Repeat(rep)));
            }

            let elems = Punctuated::parse_terminated(&mut inner)?;

            return Ok(Expr::Primary(Self::Array(ExprArray {
                attrs,
                elems: Delimited::bracket(bracket_span, elems),
            })));
        }

        // Labeled block / loop: `'a: { }`, `'a: loop { }`, etc.
        if Label::is_prefix(stream) {
            let label = Some(stream.parse::<Label>()?);

            if stream.peek::<Token![while]>() {
                return Ok(Expr::Block(BlockExpr::While(ExprWhile::parse_from(stream, label, attrs)?)));
            }

            if stream.peek::<Token![for]>() {
                return Ok(Expr::Block(BlockExpr::ForLoop(ExprForLoop::parse_from(
                    stream, label, attrs,
                )?)));
            }

            if stream.peek::<Token![loop]>() {
                return Ok(Expr::Block(BlockExpr::Loop(ExprLoop::parse_from(stream, label, attrs)?)));
            }

            return Ok(Expr::Block(BlockExpr::Brace(ExprBrace {
                attrs,
                label,
                block: stream.parse()?,
            })));
        }

        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
            return Ok(Expr::Block(BlockExpr::Brace(ExprBrace {
                attrs,
                label: None,
                block: stream.parse()?,
            })));
        }

        if stream.peek::<Token![if]>() {
            return ExprIf::parse_from(stream, attrs);
        }

        if stream.peek::<Token![while]>() {
            return Ok(Expr::Block(BlockExpr::While(ExprWhile::parse_from(stream, None, attrs)?)));
        }

        if stream.peek::<Token![for]>() {
            return Ok(Expr::Block(BlockExpr::ForLoop(ExprForLoop::parse_from(stream, None, attrs)?)));
        }

        if stream.peek::<Token![loop]>() {
            return Ok(Expr::Block(BlockExpr::Loop(ExprLoop::parse_from(stream, None, attrs)?)));
        }

        if stream.peek::<Token![match]>() {
            return ExprMatch::parse_from(stream, attrs);
        }

        if stream.peek::<Token![unsafe]>() {
            return Ok(Expr::Block(BlockExpr::Unsafe(ExprUnsafe::parse_from(stream, attrs)?)));
        }

        if stream.peek::<Token![const]>() && ExprBrace::is_next(stream) {
            return Ok(Expr::Block(BlockExpr::Const(ExprConst::parse_from(stream, attrs)?)));
        }

        if stream.peek::<Token![async]>() && ExprAsync::is_block(stream) {
            return Ok(Expr::Block(BlockExpr::Async(ExprAsync::parse_from(stream, attrs)?)));
        }

        if stream.peek::<Token![try]>() && ExprBrace::is_next(stream) {
            return Ok(Expr::Block(BlockExpr::TryBlock(ExprTryBlock::parse_from(stream, attrs)?)));
        }

        if stream.peek::<Token![return]>() {
            let return_keyword = stream.parse::<Token![return]>()?;

            return Ok(Expr::Jump(JumpExpr::Return(ExprReturn {
                attrs,
                return_keyword,
                expr: Expr::parse_if(stream)?,
            })));
        }

        if stream.peek::<Token![yield]>() {
            let yield_keyword = stream.parse::<Token![yield]>()?;

            return Ok(Expr::Jump(JumpExpr::Yield(ExprYield {
                attrs,
                yield_keyword,
                expr: Expr::parse_if(stream)?,
            })));
        }

        if stream.peek::<Token![break]>() {
            let break_keyword = stream.parse::<Token![break]>()?;
            let label = Label::parse_opt_break(stream);

            return Ok(Expr::Jump(JumpExpr::Break(ExprBreak {
                attrs,
                break_keyword,
                label,
                expr: Expr::parse_if(stream)?,
            })));
        }

        if stream.peek::<Token![continue]>() {
            let continue_keyword = stream.parse::<Token![continue]>()?;
            let label = Label::parse_opt_break(stream);

            return Ok(Expr::Jump(JumpExpr::Continue(ExprContinue {
                attrs,
                continue_keyword,
                label,
            })));
        }

        if stream.peek::<Token![let]>() {
            let let_keyword = stream.parse::<Token![let]>()?;
            let pat = Box::new(stream.parse::<Pattern>()?);
            let eq = stream.parse::<Token![=]>()?;
            let expr = Box::new(super::parse_expr(stream, false)?);

            return Ok(Expr::Primary(Self::Let(ExprLet {
                attrs,
                let_keyword,
                pat,
                eq,
                expr,
            })));
        }

        if ExprClosure::is_start(stream) {
            return Ok(Expr::Primary(Self::Closure(ExprClosure::parse_from(stream, attrs)?)));
        }

        if matches!(stream.curr(), Some(tt) if ExprLit::is_literal(tt)) {
            return Ok(Expr::Primary(Self::Lit(ExprLit {
                attrs,
                lit: stream.parse()?,
            })));
        }

        if let Some(mac) = stream.parse_if::<crate::MacroCall>() {
            return Ok(Expr::Primary(Self::Macro(ExprMacro { attrs, mac })));
        }

        // Qualified path `<T as Trait>::assoc` in expression position.
        if stream.peek::<Token![<]>() {
            let (qself, path) = crate::ty::QSelf::parse_qualified(stream)?;

            return Ok(Expr::Primary(Self::Path(ExprPath {
                attrs,
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
                return Ok(Expr::Primary(Self::Struct(ExprStruct {
                    attrs,
                    qself: None,
                    path,
                    body,
                })));
            }

            return Ok(Expr::Primary(Self::Path(ExprPath {
                attrs,
                qself: None,
                path,
            })));
        }

        Err(LexError::new(at).message("expected expression").into())
    }
}
