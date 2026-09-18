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
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);

        !cursor.is_empty()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        parse_assignment(parser, attrs)
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        skip_expr(cursor)
    }
}

fn skip_expr(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    skip_expr_with(cursor, false)
}

pub(crate) fn skip_pattern_bound(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    skip_expr_with(cursor, true)
}

fn skip_expr_with(cursor: Cursor<'_>, pattern_bound: bool) -> Option<Cursor<'_>> {
    fn skip_list(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while !cursor.is_empty() {
            cursor = cursor.skip::<Expr>()?;

            if cursor.is_empty() {
                break;
            }

            cursor = cursor.skip::<Token![,]>()?;
        }

        Some(cursor)
    }

    fn skip_pattern_single(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<Pattern>()
    }

    fn skip_closure_param(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = skip_pattern_single(cursor)?;

        if cursor.peek::<Token![:]>() {
            cursor = cursor.skip::<Token![:]>()?;
            cursor = cursor.skip::<Type>()?;
        }

        Some(cursor)
    }

    fn skip_primary(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut closure = cursor;
        closure = BoundLifetimes::skip(closure).unwrap_or(closure);
        closure = Constness::skip(closure)?;
        closure = Movability::skip(closure)?;
        closure = Asyncness::skip(closure)?;
        closure = closure.skip::<Option<Token![move]>>()?;

        if closure.peek::<Token![||]>() || closure.peek::<Token![|]>() {
            cursor = BoundLifetimes::skip(cursor).unwrap_or(cursor);
            cursor = Constness::skip(cursor)?;
            cursor = Movability::skip(cursor)?;
            cursor = Asyncness::skip(cursor)?;
            cursor = cursor.skip::<Option<Token![move]>>()?;

            if cursor.peek::<Token![||]>() {
                cursor = cursor.skip::<Token![||]>()?;
            } else {
                cursor = cursor.skip::<Token![|]>()?;

                while !cursor.peek::<Token![|]>() {
                    cursor = skip_closure_param(cursor)?;

                    if cursor.peek::<Token![,]>() {
                        cursor = cursor.skip::<Token![,]>()?;
                    } else {
                        break;
                    }
                }

                cursor = cursor.skip::<Token![|]>()?;
            }

            cursor = ReturnType::skip(cursor)?;
            return cursor.skip::<Expr>();
        }

        if cursor.peek::<Lit>() {
            return cursor.skip::<Lit>();
        }

        if cursor.peek::<Token![_]>() {
            return cursor.skip::<Token![_]>();
        }

        if cursor.is_delimited(Delim::Paren) {
            let inner = cursor.descend(Delim::Paren)?;
            let inner = skip_list(inner)?;
            return inner.is_empty().then(|| cursor.offset(1));
        }

        if cursor.is_delimited(Delim::Bracket) {
            let mut inner = cursor.descend(Delim::Bracket)?;

            if inner.is_empty() {
                return Some(cursor.offset(1));
            }

            inner = inner.skip::<Expr>()?;

            if inner.peek::<Token![;]>() {
                inner = inner.skip::<Token![;]>()?;
                inner = inner.skip::<Expr>()?;
            } else {
                while !inner.is_empty() {
                    inner = inner.skip::<Token![,]>()?;

                    if !inner.is_empty() {
                        inner = inner.skip::<Expr>()?;
                    }
                }
            }

            return inner.is_empty().then(|| cursor.offset(1));
        }

        if cursor.is_delimited(Delim::Brace) {
            return cursor.skip::<StmtBlock>();
        }

        if cursor.is_delimited(Delim::None) {
            let inner = cursor.descend(Delim::None)?.skip::<Expr>()?;
            return inner.is_empty().then(|| cursor.offset(1));
        }

        if cursor.peek::<Token![let]>() {
            cursor = cursor.skip::<Token![let]>()?;
            cursor = cursor.skip::<Pattern>()?;
            cursor = cursor.skip::<Token![=]>()?;
            return cursor.skip::<Expr>();
        }

        if cursor.peek::<Token![if]>() {
            cursor = cursor.skip::<Token![if]>()?;
            cursor = cursor.skip::<Expr>()?;
            cursor = cursor.skip::<StmtBlock>()?;

            if cursor.peek::<Token![else]>() {
                cursor = cursor.skip::<Token![else]>()?;
                cursor = cursor.skip::<Expr>()?;
            }

            return Some(cursor);
        }

        let label = Label::skip(cursor);

        if cursor.peek::<Token![while]>() || label.is_some_and(|cursor| cursor.peek::<Token![while]>()) {
            cursor = cursor.skip::<Option<Label>>()?;
            cursor = cursor.skip::<Token![while]>()?;
            cursor = cursor.skip::<Expr>()?;
            return cursor.skip::<StmtBlock>();
        }

        if cursor.peek::<Token![for]>() || label.is_some_and(|cursor| cursor.peek::<Token![for]>()) {
            cursor = cursor.skip::<Option<Label>>()?;
            cursor = cursor.skip::<Token![for]>()?;
            cursor = cursor.skip::<Pattern>()?;
            cursor = cursor.skip::<Token![in]>()?;
            cursor = cursor.skip::<Expr>()?;
            return cursor.skip::<StmtBlock>();
        }

        if cursor.peek::<Token![loop]>() || label.is_some_and(|cursor| cursor.peek::<Token![loop]>()) {
            cursor = cursor.skip::<Option<Label>>()?;
            cursor = cursor.skip::<Token![loop]>()?;
            return cursor.skip::<StmtBlock>();
        }

        if label.is_some_and(|cursor| cursor.is_delimited(Delim::Brace)) {
            cursor = cursor.skip::<Label>()?;
            return cursor.skip::<StmtBlock>();
        }

        if cursor.peek::<Token![match]>() {
            cursor = cursor.skip::<Token![match]>()?;
            cursor = cursor.skip::<Expr>()?;
            let mut inner = cursor.descend(Delim::Brace)?;

            while !inner.is_empty() {
                inner = inner.skip::<MatchArm>()?;
            }

            return Some(cursor.offset(1));
        }

        if cursor.peek::<Token![unsafe]>() {
            return cursor.skip::<Token![unsafe]>()?.skip::<StmtBlock>();
        }

        if cursor.peek::<Token![const]>() {
            return cursor.skip::<Token![const]>()?.skip::<StmtBlock>();
        }

        if cursor.peek::<Token![async]>() {
            cursor = cursor.skip::<Token![async]>()?;
            cursor = cursor.skip::<Option<Token![move]>>()?;
            return cursor.skip::<StmtBlock>();
        }

        if cursor.peek::<Token![try]>() {
            return cursor.skip::<Token![try]>()?.skip::<StmtBlock>();
        }

        if cursor.peek::<Token![return]>() {
            return cursor.skip::<Token![return]>()?.skip::<Option<Box<Expr>>>();
        }

        if cursor.peek::<Token![break]>() {
            cursor = cursor.skip::<Token![break]>()?;
            cursor = cursor.skip::<Option<Label>>()?;
            return cursor.skip::<Option<Box<Expr>>>();
        }

        if cursor.peek::<Token![continue]>() {
            return cursor.skip::<Token![continue]>()?.skip::<Option<Label>>();
        }

        if cursor.peek::<Token![yield]>() {
            return cursor.skip::<Token![yield]>()?.skip::<Option<Box<Expr>>>();
        }

        let path_start = cursor;
        let qualified = cursor.peek::<Token![<]>();

        if !qualified && !cursor.peek::<Path>() {
            return Some(cursor.offset(cursor.remaining()));
        }

        cursor = if qualified {
            cursor.skip::<ty::TypePath>()?
        } else {
            cursor.skip::<Path>()?
        };

        if !qualified && cursor.peek::<Token![!]>() {
            return MacroCall::skip(path_start);
        }

        if cursor.is_delimited(Delim::Brace) {
            let inner = cursor.descend(Delim::Brace)?.skip::<StructBody>()?;
            return inner.is_empty().then(|| cursor.offset(1));
        }

        Some(cursor)
    }

    fn skip_postfix(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = skip_primary(cursor)?;

        loop {
            if cursor.is_delimited(Delim::Paren) {
                let inner = skip_list(cursor.descend(Delim::Paren)?)?;

                if !inner.is_empty() {
                    return None;
                }

                cursor = cursor.offset(1);
                continue;
            }

            if cursor.is_delimited(Delim::Bracket) {
                let inner = cursor.descend(Delim::Bracket)?.skip::<Expr>()?;

                if !inner.is_empty() {
                    return None;
                }

                cursor = cursor.offset(1);
                continue;
            }

            if cursor.peek::<Token![.]>() {
                cursor = cursor.skip::<Token![.]>()?;

                if cursor.peek::<Token![await]>() {
                    cursor = cursor.skip::<Token![await]>()?;
                    continue;
                }

                if cursor.peek::<Ident>() {
                    let after_method = cursor.skip::<Ident>()?;
                    let args = after_method.skip::<Option<AngleArguments>>()?;

                    if args.is_delimited(Delim::Paren) {
                        let inner = skip_list(args.descend(Delim::Paren)?)?;

                        if !inner.is_empty() {
                            return None;
                        }

                        cursor = args.offset(1);
                        continue;
                    }
                }

                cursor = cursor.skip::<Member>()?;
                continue;
            }

            if cursor.peek::<Token![?]>() {
                cursor = cursor.skip::<Token![?]>()?;
                continue;
            }

            break;
        }

        Some(cursor)
    }

    fn skip_unary(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Token![&]>() && cursor.offset(1).peek::<Token![raw]>() {
            return cursor
                .skip::<Token![&]>()?
                .skip::<Token![raw]>()?
                .skip::<PointerMutability>()
                .and_then(skip_unary);
        }

        if cursor.peek::<Token![&]>() {
            return cursor.skip::<Token![&]>()?.skip::<Mutability>().and_then(skip_unary);
        }

        if cursor.peek::<UnOp>() {
            return cursor.skip::<UnOp>().and_then(skip_unary);
        }

        skip_postfix(cursor)
    }

    fn skip_cast(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = skip_unary(cursor)?;

        while cursor.peek::<Token![as]>() {
            cursor = cursor.skip::<Token![as]>()?;
            cursor = cursor.skip::<Type>()?;
        }

        Some(cursor)
    }

    fn skip_binary(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = skip_cast(cursor)?;

        while cursor.peek::<BinOp>() {
            cursor = cursor.skip::<BinOp>()?;
            cursor = skip_cast(cursor)?;
        }

        Some(cursor)
    }

    fn skip_range(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<RangeLimits>() {
            cursor = cursor.skip::<RangeLimits>()?;

            if !cursor.is_empty() && !cursor.peek::<Token![,]>() && !cursor.peek::<Token![;]>() && cursor.peek::<Expr>() {
                cursor = skip_binary(cursor)?;
            }

            return Some(cursor);
        }

        cursor = skip_binary(cursor)?;

        if cursor.peek::<RangeLimits>() {
            cursor = cursor.skip::<RangeLimits>()?;

            if !cursor.is_empty() && !cursor.peek::<Token![,]>() && !cursor.peek::<Token![;]>() && cursor.peek::<Expr>() {
                cursor = skip_binary(cursor)?;
            }
        }

        Some(cursor)
    }

    fn skip_assignment(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = skip_range(cursor)?;

        if cursor.peek::<Token![=]>() {
            cursor = cursor.skip::<Token![=]>()?;
            cursor = skip_assignment(cursor)?;
        }

        Some(cursor)
    }

    if pattern_bound {
        skip_unary(cursor)
    } else {
        skip_assignment(Attributes::skip(cursor)?)
    }
}

pub(crate) fn parse_assignment(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let left = parse_range(parser, attrs.clone())?;

    if parser.peek::<Token![=]>() {
        return Ok(ExprAssign {
            attrs: attrs.clone(),
            left: Box::new(left),
            eq: parser.parse()?,
            right: Box::new(parse_assignment(parser, attrs)?),
        }
        .into());
    }

    Ok(left)
}

pub(crate) fn parse_range(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    if parser.peek::<RangeLimits>() {
        let limits = parser.parse()?;
        let mut end = None;

        if !parser.is_empty() && !parser.peek::<Token![,]>() && !parser.peek::<Token![;]>() && parser.peek::<Expr>() {
            end = Some(Box::new(parse_binary(parser, attrs.clone())?));
        }

        return Ok(ExprRange {
            attrs: Default::default(),
            start: None,
            limits,
            end,
        }
        .into());
    }

    let left = parse_binary(parser, attrs.clone())?;

    if parser.peek::<RangeLimits>() {
        let limits = parser.parse()?;
        let mut end = None;

        if !parser.is_empty() && !parser.peek::<Token![,]>() && !parser.peek::<Token![;]>() && parser.peek::<Expr>() {
            end = Some(Box::new(parse_binary(parser, attrs.clone())?));
        }

        return Ok(ExprRange {
            attrs: Default::default(),
            start: Some(Box::new(left)),
            limits,
            end,
        }
        .into());
    }

    Ok(left)
}

pub(crate) fn parse_binary(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let mut left = parse_cast(parser, attrs.clone())?;

    while parser.peek::<BinOp>() {
        left = ExprBinary {
            attrs: attrs.clone(),
            left: Box::new(left),
            op: parser.parse()?,
            right: Box::new(parse_cast(parser, attrs.clone())?),
        }
        .into();
    }

    Ok(left)
}

pub(crate) fn parse_cast(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let mut expr = parse_unary(parser, attrs)?;

    while parser.peek::<Token![as]>() {
        expr = ExprCast {
            attrs: Default::default(),
            expr: Box::new(expr),
            as_keyword: parser.parse()?,
            ty: parser.parse()?,
        }
        .into();
    }

    Ok(expr)
}

pub(crate) fn parse_unary(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    if parser.peek::<Token![&]>() {
        if parser.cursor().offset(1).peek::<Token![raw]>() {
            return Ok(ExprRawAddr {
                attrs: Default::default(),
                and: parser.parse()?,
                raw: parser.parse()?,
                mutability: parser.parse()?,
                expr: Box::new(parse_unary(parser, attrs)?),
            }
            .into());
        }

        return Ok(ExprReference {
            attrs: Default::default(),
            and: parser.parse()?,
            mutability: parser.parse()?,
            expr: Box::new(parse_unary(parser, attrs)?),
        }
        .into());
    }

    if parser.peek::<UnOp>() {
        return Ok(ExprUnary {
            attrs: Default::default(),
            op: parser.parse()?,
            expr: Box::new(parse_unary(parser, attrs)?),
        }
        .into());
    }

    parse_postfix(parser, attrs)
}

pub(crate) fn parse_postfix(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let mut expr = parse_primary(parser, attrs)?;

    loop {
        if parser.is_delimited(Delim::Paren) {
            expr = ExprCall {
                attrs: Default::default(),
                func: Box::new(expr),
                args: Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?,
            }
            .into();

            continue;
        }

        if parser.is_delimited(Delim::Bracket) {
            expr = ExprIndex {
                attrs: Default::default(),
                base: Box::new(expr),
                index: Delimited::parse_bracket_with(parser, |inner| Ok(Box::new(inner.parse()?)))?,
            }
            .into();

            continue;
        }

        if parser.peek::<Token![.]>() {
            if parser.cursor().offset(1).peek::<Token![await]>() {
                expr = ExprAwait {
                    attrs: Default::default(),
                    base: Box::new(expr),
                    dot: parser.parse()?,
                    await_keyword: parser.parse()?,
                }
                .into();
            } else if parser.cursor().offset(1).peek::<Ident>() {
                let method = parser.cursor().offset(2);
                let args = Option::<AngleArguments>::skip(method).unwrap_or(method);

                if !args.is_delimited(Delim::Paren) {
                    expr = ExprField {
                        attrs: Default::default(),
                        base: Box::new(expr),
                        dot: parser.parse()?,
                        member: parser.parse()?,
                    }
                    .into();
                } else {
                    expr = ExprMethodCall {
                        attrs: Default::default(),
                        receiver: Box::new(expr),
                        dot: parser.parse()?,
                        method: parser.parse()?,
                        turbofish: parser.parse()?,
                        args: Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?,
                    }
                    .into();
                }
            } else {
                expr = ExprField {
                    attrs: Default::default(),
                    base: Box::new(expr),
                    dot: parser.parse()?,
                    member: parser.parse()?,
                }
                .into();
            }

            continue;
        }

        if parser.peek::<Token![?]>() {
            expr = ExprTry {
                attrs: Default::default(),
                expr: Box::new(expr),
                question_punct: parser.parse()?,
            }
            .into();

            continue;
        }

        break;
    }

    Ok(expr)
}

pub(crate) fn parse_primary(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let mut closure = parser.cursor();
    closure = BoundLifetimes::skip(closure).unwrap_or(closure);
    closure = Constness::skip(closure).unwrap_or(closure);
    closure = Movability::skip(closure).unwrap_or(closure);
    closure = Asyncness::skip(closure).unwrap_or(closure);
    closure = Option::<Token![move]>::skip(closure).unwrap_or(closure);

    if closure.peek::<Token![||]>() || closure.peek::<Token![|]>() {
        return parse_closure(parser, attrs);
    }

    if parser.peek::<Lit>() {
        return Ok(ExprLit {
            attrs,
            lit: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![_]>() {
        return Ok(ExprInfer {
            attrs,
            underscore: parser.parse()?,
        }
        .into());
    }

    if parser.is_delimited(Delim::Paren) {
        return parse_paren_or_tuple(parser, attrs);
    }

    if parser.is_delimited(Delim::Bracket) {
        return parse_array_or_repeat(parser, attrs);
    }

    if parser.is_delimited(Delim::Brace) {
        return Ok(ExprBlock {
            attrs,
            label: None,
            block: parser.parse()?,
        }
        .into());
    }

    if parser.is_delimited(Delim::None) {
        let inner = parser.parse_group(Delim::None)?;
        return Ok(ExprGroup {
            attrs,
            expr: Box::new(inner.parse()?),
        }
        .into());
    }

    if parser.peek::<Token![let]>() {
        return Ok(ExprLet {
            attrs,
            let_keyword: parser.parse()?,
            pat: parser.parse()?,
            eq: parser.parse()?,
            expr: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![if]>() {
        let if_keyword = parser.parse()?;
        let cond = parser.parse()?;
        let then_branch = parser.parse()?;
        let (else_keyword, else_branch) = if parser.peek::<Token![else]>() {
            (Some(parser.parse()?), Some(parser.parse()?))
        } else {
            (None, None)
        };

        return Ok(ExprIf {
            attrs,
            if_keyword,
            cond,
            then_branch,
            else_keyword,
            else_branch,
        }
        .into());
    }

    let label_cursor = Label::skip(parser.cursor());

    if parser.peek::<Token![while]>() || label_cursor.is_some_and(|cursor| cursor.peek::<Token![while]>()) {
        return Ok(ExprWhile {
            attrs,
            label: parser.parse()?,
            while_keyword: parser.parse()?,
            cond: parser.parse()?,
            body: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![for]>() || label_cursor.is_some_and(|cursor| cursor.peek::<Token![for]>()) {
        return Ok(ExprForLoop {
            attrs,
            label: parser.parse()?,
            for_keyword: parser.parse()?,
            pat: parser.parse()?,
            in_keyword: parser.parse()?,
            expr: parser.parse()?,
            body: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![loop]>() || label_cursor.is_some_and(|cursor| cursor.peek::<Token![loop]>()) {
        return Ok(ExprLoop {
            attrs,
            label: parser.parse()?,
            loop_keyword: parser.parse()?,
            body: parser.parse()?,
        }
        .into());
    }

    if let Some(cursor) = label_cursor
        && cursor.is_delimited(Delim::Brace)
    {
        return Ok(ExprBlock {
            attrs,
            label: parser.parse()?,
            block: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![match]>() {
        let match_keyword = parser.parse()?;
        let expr = parser.parse()?;
        let (span, arms) = parser.parse_group_spanned(Delim::Brace)?;

        return Ok(ExprMatch {
            attrs,
            match_keyword,
            expr,
            arms: Delimited::brace(span, arms.parse_until_empty()?),
        }
        .into());
    }

    if parser.peek::<Token![unsafe]>() {
        return Ok(ExprUnsafe {
            attrs,
            unsafe_keyword: parser.parse()?,
            block: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![const]>() {
        return Ok(ExprConst {
            attrs,
            const_keyword: parser.parse()?,
            block: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![async]>() {
        return Ok(ExprAsync {
            attrs,
            async_keyword: parser.parse()?,
            move_keyword: parser.parse()?,
            block: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![try]>() {
        return Ok(ExprTryBlock {
            attrs,
            try_keyword: parser.parse()?,
            block: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![return]>() {
        return Ok(ExprReturn {
            attrs,
            return_keyword: parser.parse()?,
            expr: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![break]>() {
        return Ok(ExprBreak {
            attrs,
            break_keyword: parser.parse()?,
            label: parser.parse()?,
            expr: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![continue]>() {
        return Ok(ExprContinue {
            attrs,
            continue_keyword: parser.parse()?,
            label: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![yield]>() {
        return Ok(ExprYield {
            attrs,
            yield_keyword: parser.parse()?,
            expr: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![<]>() || parser.peek::<Path>() {
        let (qself, path) = if parser.peek::<Token![<]>() {
            let (qself, path) = QSelf::parse_qualified(parser)?;
            (Some(qself), path)
        } else {
            (None, parser.parse()?)
        };

        if qself.is_none() && parser.peek::<Token![!]>() {
            let mac = MacroCall {
                path,
                bang: parser.parse()?,
                body: parser.parse()?,
                semi: parser.parse()?,
            };

            return Ok(ExprMacro { attrs, mac }.into());
        }

        if parser.is_delimited(Delim::Brace) {
            return Ok(ExprStruct {
                attrs,
                qself,
                path,
                body: Delimited::parse_brace(parser)?,
            }
            .into());
        }

        return Ok(ExprPath { attrs, qself, path }.into());
    }

    let remaining = parser.remaining();
    let tokens = parser.to_token_stream();
    parser.advance_by(remaining);
    Ok(Expr::Verbatim(tokens))
}

pub(crate) fn parse_paren_or_tuple(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;

    if parser.is_empty() {
        return Ok(ExprTuple {
            attrs,
            elems: Delimited::paren(span, Default::default()),
        }
        .into());
    }

    let first = parser.parse()?;

    if parser.peek::<Token![,]>() {
        let mut elems = Punctuated::new();
        elems.push_value(first);

        while parser.peek::<Token![,]>() {
            elems.push_punct(parser.parse()?);

            if !parser.is_empty() {
                elems.push_value(parser.parse()?);
            }
        }

        return Ok(ExprTuple {
            attrs,
            elems: Delimited::paren(span, elems),
        }
        .into());
    }

    Ok(ExprParen {
        attrs,
        content: Delimited::paren(span, Box::new(first)),
    }
    .into())
}

pub(crate) fn parse_array_or_repeat(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let (span, parser) = parser.parse_group_spanned(Delim::Bracket)?;

    if parser.is_empty() {
        return Ok(ExprArray {
            attrs,
            elems: Delimited::bracket(span, Default::default()),
        }
        .into());
    }

    let first = parser.parse()?;

    if parser.peek::<Token![;]>() {
        return Ok(ExprRepeat {
            attrs,
            content: Delimited::bracket(
                span,
                RepeatInner {
                    elem: Box::new(first),
                    semi: parser.parse()?,
                    len: Box::new(parser.parse()?),
                },
            ),
        }
        .into());
    }

    let mut elems = Punctuated::new();
    elems.push_value(first);

    while parser.peek::<Token![,]>() {
        elems.push_punct(parser.parse()?);

        if !parser.is_empty() {
            elems.push_value(parser.parse()?);
        }
    }

    Ok(ExprArray {
        attrs,
        elems: Delimited::bracket(span, elems),
    }
    .into())
}

pub(crate) fn parse_closure(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let lifetimes = parser.parse()?;
    let constness = parser.parse()?;
    let movability = parser.parse()?;
    let asyncness = parser.parse()?;
    let capture = parser.parse()?;
    let (pipes, inputs) = if parser.peek::<Token![||]>() {
        let oror = parser.parse()?;
        (ClosurePipes::Empty(oror), Punctuated::new())
    } else {
        let open = parser.parse()?;
        let mut params = Punctuated::new();

        while !parser.peek::<Token![|]>() && !parser.is_empty() {
            params.push_value(parser.parse()?);

            if parser.peek::<Token![,]>() {
                params.push_punct(parser.parse()?);
            } else {
                break;
            }
        }

        let close = parser.parse()?;
        (ClosurePipes::Params(open, close), params)
    };

    let output = parser.parse()?;
    let body = Box::new(parser.parse()?);

    Ok(ExprClosure {
        attrs,
        lifetimes,
        constness,
        movability,
        asyncness,
        capture,
        pipes,
        inputs,
        output,
        body,
    }
    .into())
}
