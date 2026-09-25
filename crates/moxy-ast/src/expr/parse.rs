use moxy_token::{Ident, Lit};

use super::peek;
use super::*;

pub(super) fn expr(parser: &Parser, context: ExprContext) -> Result<Expr, ParseError> {
    let attrs = <_ as Parse>::parse(parser)?;
    assignment(parser, attrs, context)
}

pub(crate) fn const_generic_default(parser: &Parser) -> Result<Expr, ParseError> {
    expr(parser, ExprContext::CONST_GENERIC_DEFAULT)
}

fn optional_expr(parser: &Parser, context: ExprContext) -> Result<Option<Box<Expr>>, ParseError> {
    if peek::expr(parser.cursor(), context) {
        Ok(Some(Box::new(expr(parser, context)?)))
    } else {
        Ok(None)
    }
}

fn assignment(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    let left = range(parser, attrs.clone(), context)?;

    if <Token![=]>::peek(parser.cursor()) && !<Token![==]>::peek(parser.cursor()) && !<Token![=>]>::peek(parser.cursor()) {
        return Ok(ExprAssign {
            attrs: attrs.clone(),
            left: Box::new(left),
            eq: <_ as Parse>::parse(parser)?,
            right: Box::new(assignment(parser, attrs, context)?),
        }
        .into());
    }

    Ok(left)
}

fn range(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    if RangeLimits::peek(parser.cursor()) {
        let limits = <_ as Parse>::parse(parser)?;
        let mut end = None;

        if !context.is_end(parser.cursor()) && Expr::peek(parser.cursor()) {
            end = Some(Box::new(binary(parser, attrs.clone(), context)?));
        }

        return Ok(ExprRange {
            attrs: Default::default(),
            start: None,
            limits,
            end,
        }
        .into());
    }

    let left = binary(parser, attrs.clone(), context)?;

    if RangeLimits::peek(parser.cursor()) {
        let limits = <_ as Parse>::parse(parser)?;
        let mut end = None;

        if !context.is_end(parser.cursor()) && Expr::peek(parser.cursor()) {
            end = Some(Box::new(binary(parser, attrs.clone(), context)?));
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

fn binary(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    let mut left = cast(parser, attrs.clone(), context)?;

    while BinOp::peek(parser.cursor()) && !context.is_end_before_infix(parser.cursor()) {
        left = ExprBinary {
            attrs: attrs.clone(),
            left: Box::new(left),
            op: <_ as Parse>::parse(parser)?,
            right: Box::new(cast(parser, attrs.clone(), context)?),
        }
        .into();
    }

    Ok(left)
}

fn cast(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    let mut expr = unary_with(parser, attrs, context)?;

    while <Token![as]>::peek(parser.cursor()) {
        expr = ExprCast {
            attrs: Default::default(),
            expr: Box::new(expr),
            as_keyword: <_ as Parse>::parse(parser)?,
            ty: <_ as Parse>::parse(parser)?,
        }
        .into();
    }

    Ok(expr)
}

pub(crate) fn unary(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    unary_with(parser, attrs, ExprContext::PATTERN_BOUND)
}

fn unary_with(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    if <Token![&]>::peek(parser.cursor()) {
        if <Token![raw]>::peek(parser.cursor().offset(1)) && PointerMutability::peek(parser.cursor().offset(2)) {
            return Ok(ExprRawAddr {
                attrs: Default::default(),
                and: <_ as Parse>::parse(parser)?,
                raw: <_ as Parse>::parse(parser)?,
                mutability: <_ as Parse>::parse(parser)?,
                expr: Box::new(unary_with(parser, attrs, context)?),
            }
            .into());
        }

        return Ok(ExprReference {
            attrs: Default::default(),
            and: <_ as Parse>::parse(parser)?,
            mutability: <_ as Parse>::parse(parser)?,
            expr: Box::new(unary_with(parser, attrs, context)?),
        }
        .into());
    }

    if UnOp::peek(parser.cursor()) {
        return Ok(ExprUnary {
            attrs: Default::default(),
            op: <_ as Parse>::parse(parser)?,
            expr: Box::new(unary_with(parser, attrs, context)?),
        }
        .into());
    }

    postfix(parser, attrs, context)
}

fn postfix(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    let mut expr = primary(parser, attrs, context)?;

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
                index: Delimited::parse_bracket_with(parser, |inner| Ok(Box::new(<_ as Parse>::parse(inner)?)))?,
            }
            .into();

            continue;
        }

        if <Token![.]>::peek(parser.cursor()) && !<Token![..]>::peek(parser.cursor()) && !<Token![..=]>::peek(parser.cursor()) {
            if <Token![await]>::peek(parser.cursor().offset(1)) {
                expr = ExprAwait {
                    attrs: Default::default(),
                    base: Box::new(expr),
                    dot: <_ as Parse>::parse(parser)?,
                    await_keyword: <_ as Parse>::parse(parser)?,
                }
                .into();
            } else if Ident::peek(parser.cursor().offset(1)) {
                let method = parser.cursor().offset(2);
                let args = Option::<AngleArguments>::skip(method).unwrap_or(method);

                if !args.is_delimited(Delim::Paren) {
                    expr = ExprField {
                        attrs: Default::default(),
                        base: Box::new(expr),
                        dot: <_ as Parse>::parse(parser)?,
                        member: <_ as Parse>::parse(parser)?,
                    }
                    .into();
                } else {
                    expr = ExprMethodCall {
                        attrs: Default::default(),
                        receiver: Box::new(expr),
                        dot: <_ as Parse>::parse(parser)?,
                        method: <_ as Parse>::parse(parser)?,
                        turbofish: <_ as Parse>::parse(parser)?,
                        args: Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?,
                    }
                    .into();
                }
            } else {
                expr = ExprField {
                    attrs: Default::default(),
                    base: Box::new(expr),
                    dot: <_ as Parse>::parse(parser)?,
                    member: <_ as Parse>::parse(parser)?,
                }
                .into();
            }

            continue;
        }

        if <Token![?]>::peek(parser.cursor()) {
            expr = ExprTry {
                attrs: Default::default(),
                expr: Box::new(expr),
                question_punct: <_ as Parse>::parse(parser)?,
            }
            .into();

            continue;
        }

        break;
    }

    Ok(expr)
}

fn primary(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    let mut closure_cursor = parser.cursor();
    closure_cursor = BoundLifetimes::skip(closure_cursor).unwrap_or(closure_cursor);
    closure_cursor = Option::<Token![const]>::skip(closure_cursor).unwrap_or(closure_cursor);
    closure_cursor = Option::<Token![static]>::skip(closure_cursor).unwrap_or(closure_cursor);
    closure_cursor = Option::<Token![async]>::skip(closure_cursor).unwrap_or(closure_cursor);
    closure_cursor = Option::<Token![move]>::skip(closure_cursor).unwrap_or(closure_cursor);

    if <Token![||]>::peek(closure_cursor) || <Token![|]>::peek(closure_cursor) {
        return closure(parser, attrs, context);
    }

    if Lit::peek(parser.cursor()) {
        return Ok(ExprLit {
            attrs,
            lit: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![_]>::peek(parser.cursor()) {
        return Ok(ExprInfer {
            attrs,
            underscore: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if parser.is_delimited(Delim::Paren) {
        return paren_or_tuple(parser, attrs);
    }

    if parser.is_delimited(Delim::Bracket) {
        return array_or_repeat(parser, attrs);
    }

    if parser.is_delimited(Delim::Brace) {
        return Ok(ExprBlock {
            attrs,
            label: None,
            block: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if parser.is_delimited(Delim::None) {
        let inner = parser.parse_group(Delim::None)?;
        return Ok(ExprGroup {
            attrs,
            expr: Box::new(<_ as Parse>::parse(&inner)?),
        }
        .into());
    }

    if <Token![let]>::peek(parser.cursor()) {
        return Ok(ExprLet {
            attrs,
            let_keyword: <_ as Parse>::parse(parser)?,
            pat: <_ as Parse>::parse(parser)?,
            eq: <_ as Parse>::parse(parser)?,
            expr: Box::new(expr(parser, context)?),
        }
        .into());
    }

    if <Token![if]>::peek(parser.cursor()) {
        let if_keyword = <_ as Parse>::parse(parser)?;
        let cond = Box::new(expr(parser, ExprContext::EARLY)?);
        let then_branch = <_ as Parse>::parse(parser)?;
        let (else_keyword, else_branch) = if <Token![else]>::peek(parser.cursor()) {
            (Some(<_ as Parse>::parse(parser)?), Some(<_ as Parse>::parse(parser)?))
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

    if <Token![while]>::peek(parser.cursor()) || label_cursor.is_some_and(|cursor| <Token![while]>::peek(cursor)) {
        return Ok(ExprWhile {
            attrs,
            label: <_ as Parse>::parse(parser)?,
            while_keyword: <_ as Parse>::parse(parser)?,
            cond: Box::new(expr(parser, ExprContext::EARLY)?),
            body: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![for]>::peek(parser.cursor()) || label_cursor.is_some_and(|cursor| <Token![for]>::peek(cursor)) {
        return Ok(ExprForLoop {
            attrs,
            label: <_ as Parse>::parse(parser)?,
            for_keyword: <_ as Parse>::parse(parser)?,
            pat: <_ as Parse>::parse(parser)?,
            in_keyword: <_ as Parse>::parse(parser)?,
            expr: Box::new(expr(parser, ExprContext::EARLY)?),
            body: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![loop]>::peek(parser.cursor()) || label_cursor.is_some_and(|cursor| <Token![loop]>::peek(cursor)) {
        return Ok(ExprLoop {
            attrs,
            label: <_ as Parse>::parse(parser)?,
            loop_keyword: <_ as Parse>::parse(parser)?,
            body: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if let Some(cursor) = label_cursor
        && cursor.is_delimited(Delim::Brace)
    {
        return Ok(ExprBlock {
            attrs,
            label: <_ as Parse>::parse(parser)?,
            block: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![match]>::peek(parser.cursor()) {
        let match_keyword = <_ as Parse>::parse(parser)?;
        let expr = Box::new(expr(parser, ExprContext::EARLY)?);
        let (span, arms) = parser.parse_group_spanned(Delim::Brace)?;

        return Ok(ExprMatch {
            attrs,
            match_keyword,
            expr,
            arms: Delimited::brace(span, arms.parse_until_empty()?),
        }
        .into());
    }

    if <Token![unsafe]>::peek(parser.cursor()) {
        return Ok(ExprUnsafe {
            attrs,
            unsafe_keyword: <_ as Parse>::parse(parser)?,
            block: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![const]>::peek(parser.cursor()) {
        return Ok(ExprConst {
            attrs,
            const_keyword: <_ as Parse>::parse(parser)?,
            block: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![async]>::peek(parser.cursor()) {
        return Ok(ExprAsync {
            attrs,
            async_keyword: <_ as Parse>::parse(parser)?,
            move_keyword: <_ as Parse>::parse(parser)?,
            block: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![try]>::peek(parser.cursor()) {
        return Ok(ExprTryBlock {
            attrs,
            try_keyword: <_ as Parse>::parse(parser)?,
            block: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![return]>::peek(parser.cursor()) {
        return Ok(ExprReturn {
            attrs,
            return_keyword: <_ as Parse>::parse(parser)?,
            expr: optional_expr(parser, context)?,
        }
        .into());
    }

    if <Token![break]>::peek(parser.cursor()) {
        return Ok(ExprBreak {
            attrs,
            break_keyword: <_ as Parse>::parse(parser)?,
            label: <_ as Parse>::parse(parser)?,
            expr: optional_expr(parser, context)?,
        }
        .into());
    }

    if <Token![continue]>::peek(parser.cursor()) {
        return Ok(ExprContinue {
            attrs,
            continue_keyword: <_ as Parse>::parse(parser)?,
            label: <_ as Parse>::parse(parser)?,
        }
        .into());
    }

    if <Token![yield]>::peek(parser.cursor()) {
        return Ok(ExprYield {
            attrs,
            yield_keyword: <_ as Parse>::parse(parser)?,
            expr: optional_expr(parser, context)?,
        }
        .into());
    }

    if <Token![<]>::peek(parser.cursor()) || Path::peek(parser.cursor()) {
        let (qself, path) = if <Token![<]>::peek(parser.cursor()) {
            let (qself, path) = QSelf::parse_qualified(parser)?;
            (Some(qself), path)
        } else {
            (None, <_ as Parse>::parse(parser)?)
        };

        if qself.is_none() && <Token![!]>::peek(parser.cursor()) && !<Token![!=]>::peek(parser.cursor()) {
            let mac = MacroCall {
                path,
                bang: <_ as Parse>::parse(parser)?,
                body: <_ as Parse>::parse(parser)?,
            };

            return Ok(ExprMacro { attrs, mac }.into());
        }

        if context.allow_struct && parser.is_delimited(Delim::Brace) {
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

pub(crate) fn paren_or_tuple(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;

    if parser.is_empty() {
        return Ok(ExprTuple {
            attrs,
            elems: Delimited::paren(span, Default::default()),
        }
        .into());
    }

    let first = <_ as Parse>::parse(&parser)?;

    if <Token![,]>::peek(parser.cursor()) {
        let mut elems = Punctuated::new();
        elems.push_value(first);

        while <Token![,]>::peek(parser.cursor()) {
            elems.push_punct(<_ as Parse>::parse(&parser)?);

            if !parser.is_empty() {
                elems.push_value(<_ as Parse>::parse(&parser)?);
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

pub(crate) fn array_or_repeat(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    let (span, parser) = parser.parse_group_spanned(Delim::Bracket)?;

    if parser.is_empty() {
        return Ok(ExprArray {
            attrs,
            elems: Delimited::bracket(span, Default::default()),
        }
        .into());
    }

    let first = <_ as Parse>::parse(&parser)?;

    if <Token![;]>::peek(parser.cursor()) {
        return Ok(ExprRepeat {
            attrs,
            content: Delimited::bracket(
                span,
                RepeatInner {
                    elem: Box::new(first),
                    semi: <_ as Parse>::parse(&parser)?,
                    len: Box::new(<_ as Parse>::parse(&parser)?),
                },
            ),
        }
        .into());
    }

    let mut elems = Punctuated::new();
    elems.push_value(first);

    while <Token![,]>::peek(parser.cursor()) {
        elems.push_punct(<_ as Parse>::parse(&parser)?);

        if !parser.is_empty() {
            elems.push_value(<_ as Parse>::parse(&parser)?);
        }
    }

    Ok(ExprArray {
        attrs,
        elems: Delimited::bracket(span, elems),
    }
    .into())
}

fn closure(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    let lifetimes = <_ as Parse>::parse(parser)?;
    let constness = <_ as Parse>::parse(parser)?;
    let movability = <_ as Parse>::parse(parser)?;
    let asyncness = <_ as Parse>::parse(parser)?;
    let capture = <_ as Parse>::parse(parser)?;
    let (pipes, inputs) = if <Token![||]>::peek(parser.cursor()) {
        let oror = <_ as Parse>::parse(parser)?;
        (ClosurePipes::Empty(oror), Punctuated::new())
    } else {
        let open = <_ as Parse>::parse(parser)?;
        let mut params = Punctuated::new();

        while !<Token![|]>::peek(parser.cursor()) && !parser.is_empty() {
            params.push_value(<_ as Parse>::parse(parser)?);

            if <Token![,]>::peek(parser.cursor()) {
                params.push_punct(<_ as Parse>::parse(parser)?);
            } else {
                break;
            }
        }

        let close = <_ as Parse>::parse(parser)?;
        (ClosurePipes::Params(open, close), params)
    };

    let output = <_ as Parse>::parse(parser)?;
    let body = Box::new(expr(parser, context)?);

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
