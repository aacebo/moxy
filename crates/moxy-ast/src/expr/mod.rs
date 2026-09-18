mod expr_array;
mod expr_assign;
mod expr_async;
mod expr_await;
mod expr_binary;
mod expr_block;
mod expr_break;
mod expr_call;
mod expr_cast;
mod expr_closure;
mod expr_const;
mod expr_continue;
mod expr_field;
mod expr_for_loop;
mod expr_group;
mod expr_if;
mod expr_index;
mod expr_infer;
mod expr_let;
mod expr_lit;
mod expr_loop;
mod expr_macro;
mod expr_match;
mod expr_method_call;
mod expr_paren;
mod expr_path;
mod expr_range;
mod expr_raw_addr;
mod expr_reference;
mod expr_repeat;
mod expr_return;
mod expr_struct;
mod expr_try;
mod expr_try_block;
mod expr_tuple;
mod expr_unary;
mod expr_unsafe;
mod expr_while;
mod expr_yield;

pub(crate) mod parse;
pub(crate) mod peek;
pub(crate) mod skip;

pub use expr_array::*;
pub use expr_assign::*;
pub use expr_async::*;
pub use expr_await::*;
pub use expr_binary::*;
pub use expr_block::*;
pub use expr_break::*;
pub use expr_call::*;
pub use expr_cast::*;
pub use expr_closure::*;
pub use expr_const::*;
pub use expr_continue::*;
pub use expr_field::*;
pub use expr_for_loop::*;
pub use expr_group::*;
pub use expr_if::*;
pub use expr_index::*;
pub use expr_infer::*;
pub use expr_let::*;
pub use expr_lit::*;
pub use expr_loop::*;
pub use expr_macro::*;
pub use expr_match::*;
pub use expr_method_call::*;
pub use expr_paren::*;
pub use expr_path::*;
pub use expr_range::*;
pub use expr_raw_addr::*;
pub use expr_reference::*;
pub use expr_repeat::*;
pub use expr_return::*;
pub use expr_struct::*;
pub use expr_try::*;
pub use expr_try_block::*;
pub use expr_tuple::*;
pub use expr_unary::*;
pub use expr_unsafe::*;
pub use expr_while::*;
pub use expr_yield::*;

use moxy_token::{Delim, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A Rust expression. The primary recursive node covering all expression forms.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Const(ExprConst),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Infer(ExprInfer),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    RawAddr(ExprRawAddr),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
}

impl Expr {
    pub fn attrs(&self) -> Option<&Attributes> {
        match self {
            Self::Array(v) => Some(&v.attrs),
            Self::Assign(v) => Some(&v.attrs),
            Self::Async(v) => Some(&v.attrs),
            Self::Await(v) => Some(&v.attrs),
            Self::Binary(v) => Some(&v.attrs),
            Self::Block(v) => Some(&v.attrs),
            Self::Break(v) => Some(&v.attrs),
            Self::Call(v) => Some(&v.attrs),
            Self::Cast(v) => Some(&v.attrs),
            Self::Closure(v) => Some(&v.attrs),
            Self::Const(v) => Some(&v.attrs),
            Self::Continue(v) => Some(&v.attrs),
            Self::Field(v) => Some(&v.attrs),
            Self::ForLoop(v) => Some(&v.attrs),
            Self::Group(v) => Some(&v.attrs),
            Self::If(v) => Some(&v.attrs),
            Self::Index(v) => Some(&v.attrs),
            Self::Infer(v) => Some(&v.attrs),
            Self::Let(v) => Some(&v.attrs),
            Self::Lit(v) => Some(&v.attrs),
            Self::Loop(v) => Some(&v.attrs),
            Self::Macro(v) => Some(&v.attrs),
            Self::Match(v) => Some(&v.attrs),
            Self::MethodCall(v) => Some(&v.attrs),
            Self::Paren(v) => Some(&v.attrs),
            Self::Path(v) => Some(&v.attrs),
            Self::Range(v) => Some(&v.attrs),
            Self::RawAddr(v) => Some(&v.attrs),
            Self::Reference(v) => Some(&v.attrs),
            Self::Repeat(v) => Some(&v.attrs),
            Self::Return(v) => Some(&v.attrs),
            Self::Struct(v) => Some(&v.attrs),
            Self::Try(v) => Some(&v.attrs),
            Self::TryBlock(v) => Some(&v.attrs),
            Self::Tuple(v) => Some(&v.attrs),
            Self::Unary(v) => Some(&v.attrs),
            Self::Unsafe(v) => Some(&v.attrs),
            Self::While(v) => Some(&v.attrs),
            Self::Yield(v) => Some(&v.attrs),
            _ => None,
        }
    }

    pub fn attrs_mut(&mut self) -> Option<&mut Attributes> {
        match self {
            Self::Array(v) => Some(&mut v.attrs),
            Self::Assign(v) => Some(&mut v.attrs),
            Self::Async(v) => Some(&mut v.attrs),
            Self::Await(v) => Some(&mut v.attrs),
            Self::Binary(v) => Some(&mut v.attrs),
            Self::Block(v) => Some(&mut v.attrs),
            Self::Break(v) => Some(&mut v.attrs),
            Self::Call(v) => Some(&mut v.attrs),
            Self::Cast(v) => Some(&mut v.attrs),
            Self::Closure(v) => Some(&mut v.attrs),
            Self::Const(v) => Some(&mut v.attrs),
            Self::Continue(v) => Some(&mut v.attrs),
            Self::Field(v) => Some(&mut v.attrs),
            Self::ForLoop(v) => Some(&mut v.attrs),
            Self::Group(v) => Some(&mut v.attrs),
            Self::If(v) => Some(&mut v.attrs),
            Self::Index(v) => Some(&mut v.attrs),
            Self::Infer(v) => Some(&mut v.attrs),
            Self::Let(v) => Some(&mut v.attrs),
            Self::Lit(v) => Some(&mut v.attrs),
            Self::Loop(v) => Some(&mut v.attrs),
            Self::Macro(v) => Some(&mut v.attrs),
            Self::Match(v) => Some(&mut v.attrs),
            Self::MethodCall(v) => Some(&mut v.attrs),
            Self::Paren(v) => Some(&mut v.attrs),
            Self::Path(v) => Some(&mut v.attrs),
            Self::Range(v) => Some(&mut v.attrs),
            Self::RawAddr(v) => Some(&mut v.attrs),
            Self::Reference(v) => Some(&mut v.attrs),
            Self::Repeat(v) => Some(&mut v.attrs),
            Self::Return(v) => Some(&mut v.attrs),
            Self::Struct(v) => Some(&mut v.attrs),
            Self::Try(v) => Some(&mut v.attrs),
            Self::TryBlock(v) => Some(&mut v.attrs),
            Self::Tuple(v) => Some(&mut v.attrs),
            Self::Unary(v) => Some(&mut v.attrs),
            Self::Unsafe(v) => Some(&mut v.attrs),
            Self::While(v) => Some(&mut v.attrs),
            Self::Yield(v) => Some(&mut v.attrs),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_assign(&self) -> bool {
        matches!(self, Self::Assign(_))
    }

    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async(_))
    }

    pub fn is_await(&self) -> bool {
        matches!(self, Self::Await(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_break(&self) -> bool {
        matches!(self, Self::Break(_))
    }

    pub fn is_call(&self) -> bool {
        matches!(self, Self::Call(_))
    }

    pub fn is_cast(&self) -> bool {
        matches!(self, Self::Cast(_))
    }

    pub fn is_closure(&self) -> bool {
        matches!(self, Self::Closure(_))
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }

    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue(_))
    }

    pub fn is_field(&self) -> bool {
        matches!(self, Self::Field(_))
    }

    pub fn is_for_loop(&self) -> bool {
        matches!(self, Self::ForLoop(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_if(&self) -> bool {
        matches!(self, Self::If(_))
    }

    pub fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }

    pub fn is_infer(&self) -> bool {
        matches!(self, Self::Infer(_))
    }

    pub fn is_let(&self) -> bool {
        matches!(self, Self::Let(_))
    }

    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Lit(_))
    }

    pub fn is_loop(&self) -> bool {
        matches!(self, Self::Loop(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn is_match(&self) -> bool {
        matches!(self, Self::Match(_))
    }

    pub fn is_method_call(&self) -> bool {
        matches!(self, Self::MethodCall(_))
    }

    pub fn is_paren(&self) -> bool {
        matches!(self, Self::Paren(_))
    }

    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_range(&self) -> bool {
        matches!(self, Self::Range(_))
    }

    pub fn is_raw_addr(&self) -> bool {
        matches!(self, Self::RawAddr(_))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub fn is_repeat(&self) -> bool {
        matches!(self, Self::Repeat(_))
    }

    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_try(&self) -> bool {
        matches!(self, Self::Try(_))
    }

    pub fn is_try_block(&self) -> bool {
        matches!(self, Self::TryBlock(_))
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(_))
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Unary(_))
    }

    pub fn is_unsafe(&self) -> bool {
        matches!(self, Self::Unsafe(_))
    }

    pub fn is_verbatim(&self) -> bool {
        matches!(self, Self::Verbatim(_))
    }

    pub fn is_while(&self) -> bool {
        matches!(self, Self::While(_))
    }

    pub fn is_yield(&self) -> bool {
        matches!(self, Self::Yield(_))
    }

    pub fn as_array(&self) -> Option<&ExprArray> {
        if let Self::Array(v) = self { Some(v) } else { None }
    }

    pub fn as_assign(&self) -> Option<&ExprAssign> {
        if let Self::Assign(v) = self { Some(v) } else { None }
    }

    pub fn as_async(&self) -> Option<&ExprAsync> {
        if let Self::Async(v) = self { Some(v) } else { None }
    }

    pub fn as_await(&self) -> Option<&ExprAwait> {
        if let Self::Await(v) = self { Some(v) } else { None }
    }

    pub fn as_binary(&self) -> Option<&ExprBinary> {
        if let Self::Binary(v) = self { Some(v) } else { None }
    }

    pub fn as_block(&self) -> Option<&ExprBlock> {
        if let Self::Block(v) = self { Some(v) } else { None }
    }

    pub fn as_break(&self) -> Option<&ExprBreak> {
        if let Self::Break(v) = self { Some(v) } else { None }
    }

    pub fn as_call(&self) -> Option<&ExprCall> {
        if let Self::Call(v) = self { Some(v) } else { None }
    }

    pub fn as_cast(&self) -> Option<&ExprCast> {
        if let Self::Cast(v) = self { Some(v) } else { None }
    }

    pub fn as_closure(&self) -> Option<&ExprClosure> {
        if let Self::Closure(v) = self { Some(v) } else { None }
    }

    pub fn as_const(&self) -> Option<&ExprConst> {
        if let Self::Const(v) = self { Some(v) } else { None }
    }

    pub fn as_continue(&self) -> Option<&ExprContinue> {
        if let Self::Continue(v) = self { Some(v) } else { None }
    }

    pub fn as_field(&self) -> Option<&ExprField> {
        if let Self::Field(v) = self { Some(v) } else { None }
    }

    pub fn as_for_loop(&self) -> Option<&ExprForLoop> {
        if let Self::ForLoop(v) = self { Some(v) } else { None }
    }

    pub fn as_group(&self) -> Option<&ExprGroup> {
        if let Self::Group(v) = self { Some(v) } else { None }
    }

    pub fn as_if(&self) -> Option<&ExprIf> {
        if let Self::If(v) = self { Some(v) } else { None }
    }

    pub fn as_index(&self) -> Option<&ExprIndex> {
        if let Self::Index(v) = self { Some(v) } else { None }
    }

    pub fn as_infer(&self) -> Option<&ExprInfer> {
        if let Self::Infer(v) = self { Some(v) } else { None }
    }

    pub fn as_let(&self) -> Option<&ExprLet> {
        if let Self::Let(v) = self { Some(v) } else { None }
    }

    pub fn as_lit(&self) -> Option<&ExprLit> {
        if let Self::Lit(v) = self { Some(v) } else { None }
    }

    pub fn as_loop(&self) -> Option<&ExprLoop> {
        if let Self::Loop(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&ExprMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }

    pub fn as_match(&self) -> Option<&ExprMatch> {
        if let Self::Match(v) = self { Some(v) } else { None }
    }

    pub fn as_method_call(&self) -> Option<&ExprMethodCall> {
        if let Self::MethodCall(v) = self { Some(v) } else { None }
    }

    pub fn as_paren(&self) -> Option<&ExprParen> {
        if let Self::Paren(v) = self { Some(v) } else { None }
    }

    pub fn as_path(&self) -> Option<&ExprPath> {
        if let Self::Path(v) = self { Some(v) } else { None }
    }

    pub fn as_range(&self) -> Option<&ExprRange> {
        if let Self::Range(v) = self { Some(v) } else { None }
    }

    pub fn as_raw_addr(&self) -> Option<&ExprRawAddr> {
        if let Self::RawAddr(v) = self { Some(v) } else { None }
    }

    pub fn as_reference(&self) -> Option<&ExprReference> {
        if let Self::Reference(v) = self { Some(v) } else { None }
    }

    pub fn as_repeat(&self) -> Option<&ExprRepeat> {
        if let Self::Repeat(v) = self { Some(v) } else { None }
    }

    pub fn as_return(&self) -> Option<&ExprReturn> {
        if let Self::Return(v) = self { Some(v) } else { None }
    }

    pub fn as_struct(&self) -> Option<&ExprStruct> {
        if let Self::Struct(v) = self { Some(v) } else { None }
    }

    pub fn as_try(&self) -> Option<&ExprTry> {
        if let Self::Try(v) = self { Some(v) } else { None }
    }

    pub fn as_try_block(&self) -> Option<&ExprTryBlock> {
        if let Self::TryBlock(v) = self { Some(v) } else { None }
    }

    pub fn as_tuple(&self) -> Option<&ExprTuple> {
        if let Self::Tuple(v) = self { Some(v) } else { None }
    }

    pub fn as_unary(&self) -> Option<&ExprUnary> {
        if let Self::Unary(v) = self { Some(v) } else { None }
    }

    pub fn as_unsafe(&self) -> Option<&ExprUnsafe> {
        if let Self::Unsafe(v) = self { Some(v) } else { None }
    }

    pub fn as_verbatim(&self) -> Option<&TokenStream> {
        if let Self::Verbatim(v) = self { Some(v) } else { None }
    }

    pub fn as_while(&self) -> Option<&ExprWhile> {
        if let Self::While(v) = self { Some(v) } else { None }
    }

    pub fn as_yield(&self) -> Option<&ExprYield> {
        if let Self::Yield(v) = self { Some(v) } else { None }
    }
}

impl Spanner for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Array(v) => v.span(),
            Self::Assign(v) => v.span(),
            Self::Async(v) => v.span(),
            Self::Await(v) => v.span(),
            Self::Binary(v) => v.span(),
            Self::Block(v) => v.span(),
            Self::Break(v) => v.span(),
            Self::Call(v) => v.span(),
            Self::Cast(v) => v.span(),
            Self::Closure(v) => v.span(),
            Self::Const(v) => v.span(),
            Self::Continue(v) => v.span(),
            Self::Field(v) => v.span(),
            Self::ForLoop(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::If(v) => v.span(),
            Self::Index(v) => v.span(),
            Self::Infer(v) => v.span(),
            Self::Let(v) => v.span(),
            Self::Lit(v) => v.span(),
            Self::Loop(v) => v.span(),
            Self::Macro(v) => v.span(),
            Self::Match(v) => v.span(),
            Self::MethodCall(v) => v.span(),
            Self::Paren(v) => v.span(),
            Self::Path(v) => v.span(),
            Self::Range(v) => v.span(),
            Self::RawAddr(v) => v.span(),
            Self::Reference(v) => v.span(),
            Self::Repeat(v) => v.span(),
            Self::Return(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Try(v) => v.span(),
            Self::TryBlock(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::Unary(v) => v.span(),
            Self::Unsafe(v) => v.span(),
            Self::While(v) => v.span(),
            Self::Yield(v) => v.span(),
            Self::Verbatim(v) => v.span(),
        }
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Array(v) => v.to_tokens(t),
            Self::Assign(v) => v.to_tokens(t),
            Self::Async(v) => v.to_tokens(t),
            Self::Await(v) => v.to_tokens(t),
            Self::Binary(v) => v.to_tokens(t),
            Self::Block(v) => v.to_tokens(t),
            Self::Break(v) => v.to_tokens(t),
            Self::Call(v) => v.to_tokens(t),
            Self::Cast(v) => v.to_tokens(t),
            Self::Closure(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
            Self::Continue(v) => v.to_tokens(t),
            Self::Field(v) => v.to_tokens(t),
            Self::ForLoop(v) => v.to_tokens(t),
            Self::Group(v) => v.to_tokens(t),
            Self::If(v) => v.to_tokens(t),
            Self::Index(v) => v.to_tokens(t),
            Self::Infer(v) => v.to_tokens(t),
            Self::Let(v) => v.to_tokens(t),
            Self::Lit(v) => v.to_tokens(t),
            Self::Loop(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
            Self::Match(v) => v.to_tokens(t),
            Self::MethodCall(v) => v.to_tokens(t),
            Self::Paren(v) => v.to_tokens(t),
            Self::Path(v) => v.to_tokens(t),
            Self::Range(v) => v.to_tokens(t),
            Self::RawAddr(v) => v.to_tokens(t),
            Self::Reference(v) => v.to_tokens(t),
            Self::Repeat(v) => v.to_tokens(t),
            Self::Return(v) => v.to_tokens(t),
            Self::Struct(v) => v.to_tokens(t),
            Self::Try(v) => v.to_tokens(t),
            Self::TryBlock(v) => v.to_tokens(t),
            Self::Tuple(v) => v.to_tokens(t),
            Self::Unary(v) => v.to_tokens(t),
            Self::Unsafe(v) => v.to_tokens(t),
            Self::While(v) => v.to_tokens(t),
            Self::Yield(v) => v.to_tokens(t),
            Self::Verbatim(v) => v.to_tokens(t),
        }
    }
}

impl Parse for Expr {
    fn peek(cursor: Cursor<'_>) -> bool {
        peek::expr(cursor, ExprContext::NORMAL)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        parse::expr(parser, ExprContext::NORMAL)
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        skip::expr(cursor, ExprContext::NORMAL)
    }
}

#[derive(Copy, Clone)]
struct ExprContext {
    allow_struct: bool,
    pattern_bound: bool,
}

impl ExprContext {
    const NORMAL: Self = Self {
        allow_struct: true,
        pattern_bound: false,
    };

    const EARLY: Self = Self {
        allow_struct: false,
        pattern_bound: false,
    };

    const PATTERN_BOUND: Self = Self {
        allow_struct: true,
        pattern_bound: true,
    };
}
