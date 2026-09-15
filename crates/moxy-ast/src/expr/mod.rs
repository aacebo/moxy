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

use moxy_token::{Delim, Group, Span, Spanner, ToTokenStream, ToTokens, TokenStream, TokenTree};

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
        match cursor.curr() {
            // literals
            Some(TokenTree::Literal(_)) => true,

            // paths, `_`, etc.
            Some(TokenTree::Ident(_)) => true,

            // grouped primary expressions
            Some(TokenTree::Group(group))
                if matches!(group.delim(), Delim::Paren | Delim::Bracket | Delim::Brace | Delim::None) =>
            {
                true
            }

            _ => {
                // unary expressions
                cursor.peek::<Token![&]>()
                    || cursor.peek::<Token![*]>()
                    || cursor.peek::<Token![!]>()
                    || cursor.peek::<Token![-]>()

                    // paths beginning with ::
                    || cursor.peek::<Token![::]>()

                    // closures
                    || cursor.peek::<Token![|]>()
                    || cursor.peek::<Token![||]>()

                    // block expressions
                    || cursor.peek::<Token![if]>()
                    || cursor.peek::<Token![match]>()
                    || cursor.peek::<Token![while]>()
                    || cursor.peek::<Token![for]>()
                    || cursor.peek::<Token![loop]>()
                    || cursor.peek::<Token![async]>()
                    || cursor.peek::<Token![unsafe]>()
                    || cursor.peek::<Token![const]>()
                    || cursor.peek::<Token![try]>()

                    // jump expressions
                    || cursor.peek::<Token![return]>()
                    || cursor.peek::<Token![break]>()
                    || cursor.peek::<Token![continue]>()
                    || cursor.peek::<Token![yield]>()

                    // let expression
                    || cursor.peek::<Token![let]>()

                    // range with no lhs, if supported
                    || cursor.peek::<Token![..]>()
                    || cursor.peek::<Token![..=]>()
            }
        }
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        parse_assignment(parser, attrs)
    }
}

pub(crate) fn parse_assignment(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let left = parse_range(parser, attrs)?;

    if parser.peek::<Token![=]>() {
        return Ok(ExprAssign {
            attrs,
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
            end = Some(Box::new(parse_binary(parser, attrs)?));
        }

        return Ok(ExprRange {
            attrs: Default::default(),
            start: None,
            limits,
            end,
        }
        .into());
    }

    let left = parser.parse()?;

    if parser.peek::<RangeLimits>() {
        let limits = parser.parse()?;
        let mut end = None;

        if !parser.is_empty() && !parser.peek::<Token![,]>() && !parser.peek::<Token![;]>() && parser.peek::<Expr>() {
            end = Some(Box::new(parse_binary(parser, attrs)?));
        }

        return Ok(ExprRange {
            attrs: Default::default(),
            start: None,
            limits,
            end,
        }
        .into());
    }

    Ok(left)
}

pub(crate) fn parse_binary(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let mut left = parse_cast(parser, attrs)?;

    while parser.peek::<BinOp>() {
        left = ExprBinary {
            attrs,
            left: Box::new(left),
            op: parser.parse()?,
            right: Box::new(parse_cast(parser, attrs)?),
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
                args: parser.parse()?,
            }
            .into();
        }

        if parser.is_delimited(Delim::Bracket) {
            expr = ExprIndex {
                attrs: Default::default(),
                base: Box::new(expr),
                index: parser.parse()?,
            }
            .into();
        }

        if parser.peek::<Token![.]>() {
            if parser.offset(1).peek::<Token![await]>() {
                expr = ExprAwait {
                    attrs: Default::default(),
                    base: Box::new(expr),
                    dot: parser.parse()?,
                    await_keyword: parser.parse()?,
                }
                .into();
            } else if parser.offset(1).peek::<Ident>() && parser.offset(2).is_delimited(Delim::Paren) {
                expr = ExprMethodCall {
                    attrs: Default::default(),
                    receiver: Box::new(expr),
                    dot: parser.parse()?,
                    method: parser.parse()?,
                    turbofish: parser.parse()?,
                    args: parser.parse()?,
                }
                .into();
            } else {
                expr = ExprField {
                    attrs: Default::default(),
                    base: Box::new(expr),
                    dot: parser.parse()?,
                    member: parser.parse()?,
                }
                .into();
            }
        }

        if parser.peek::<Token![?]>() {
            expr = ExprTry {
                attrs: Default::default(),
                expr: Box::new(expr),
                question_punct: parser.parse()?,
            }
            .into();
        }

        break;
    }

    Ok(expr)
}

pub(crate) fn parse_primary(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    if parser.peek::<Lit>() {
        return Ok(ExprLit {
            attrs,
            lit: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Path>() {
        return Ok(ExprPath {
            attrs,
            path: parser.parse()?,
            qself: parser.parse()?,
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
            label: parser.parse()?,
            block: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![|]>()
        || parser.peek::<Token![||]>()
        || parser.peek::<Token![move]>()
        || parser.peek::<Token![async]>()
    {
        return parse_closure(parser, attrs);
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
        return Ok(ExprIf {
            attrs,
            if_keyword: parser.parse()?,
            cond: parser.parse()?,
            then_branch: parser.parse()?,
            else_keyword: parser.parse()?,
            else_branch: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![while]>() {
        return Ok(ExprWhile {
            attrs,
            label: parser.parse()?,
            while_keyword: parser.parse()?,
            cond: parser.parse()?,
            body: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![for]>() {
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

    if parser.peek::<Token![loop]>() {
        return Ok(ExprLoop {
            attrs,
            label: parser.parse()?,
            loop_keyword: parser.parse()?,
            body: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Token![match]>() {
        return Ok(ExprMatch {
            attrs,
            match_keyword: parser.parse()?,
            expr: parser.parse()?,
            arms: parser.parse()?,
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

    if parser.peek::<Token![_]>() {
        return Ok(ExprInfer {
            attrs,
            underscore: parser.parse()?,
        }
        .into());
    }

    if parser.peek::<Group>() {
        return parser.parse();
    }

    Ok(Expr::Verbatim(parser.to_token_stream()))
}

pub(crate) fn parse_paren_or_tuple(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;

    if parser.is_empty() {
        return Ok(ExprTuple {
            attrs,
            elems: Delimited::bracket(span, Default::default()),
        }
        .into());
    }

    let first = parser.parse()?;

    if parser.peek::<Token![,]>() {
        let mut elems = Punctuated::new();
        elems.push_value(first);

        while !parser.peek::<Token![,]>() {
            elems.push_punct(parser.parse()?);

            if !parser.is_empty() {
                elems.push_value(parser.parse()?);
            }
        }

        return Ok(ExprTuple {
            attrs,
            elems: Delimited::bracket(span, elems),
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

    let mut elems = vec![first];
    elems.extend(parser.parse_until_empty()?);

    Ok(ExprArray {
        attrs,
        elems: Delimited::bracket(span, Punctuated::from_iter(elems)),
    }
    .into())
}

pub(crate) fn parse_closure(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let constness = parser.parse()?;
    let asyncness = parser.parse()?;
    let capture = parser.parse()?;
    let (pipes, inputs) = if parser.peek::<Token![||]>() {
        let oror = parser.parse()?;
        (ClosurePipes::Empty(oror), Punctuated::new())
    } else {
        let open = parser.parse::<Token![|]>()?;
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

    return Ok(ExprClosure {
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
    }
    .into());
}
