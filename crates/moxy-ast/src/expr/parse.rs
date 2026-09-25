use super::peek;
use super::*;

pub(super) fn expr(parser: &Parser, context: ExprContext) -> Result<Expr, ParseError> {
    let attrs = parser.parse()?;
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

    if parser.peek::<Token![=]>() && !parser.peek::<Token![==]>() && !parser.peek::<Token![=>]>() {
        return Ok(ExprAssign {
            attrs: attrs.clone(),
            left: Box::new(left),
            eq: parser.parse()?,
            right: Box::new(assignment(parser, attrs, context)?),
        }
        .into());
    }

    Ok(left)
}

fn range(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    if parser.peek::<RangeLimits>() {
        let limits = parser.parse()?;
        let mut end = None;

        if !context.is_end(parser.cursor()) && parser.peek::<Expr>() {
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

    if parser.peek::<RangeLimits>() {
        let limits = parser.parse()?;
        let mut end = None;

        if !context.is_end(parser.cursor()) && parser.peek::<Expr>() {
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

    while parser.peek::<BinOp>() && !context.is_end_before_infix(parser.cursor()) {
        left = ExprBinary {
            attrs: attrs.clone(),
            left: Box::new(left),
            op: parser.parse()?,
            right: Box::new(cast(parser, attrs.clone(), context)?),
        }
        .into();
    }

    Ok(left)
}

fn cast(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    let mut expr = unary_with(parser, attrs, context)?;

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

pub(crate) fn unary(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    unary_with(parser, attrs, ExprContext::PATTERN_BOUND)
}

fn unary_with(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    if parser.peek::<Token![&]>() {
        if parser.cursor().offset(1).peek::<Token![raw]>() && parser.cursor().offset(2).peek::<PointerMutability>() {
            return Ok(ExprRawAddr {
                attrs: Default::default(),
                and: parser.parse()?,
                raw: parser.parse()?,
                mutability: parser.parse()?,
                expr: Box::new(unary_with(parser, attrs, context)?),
            }
            .into());
        }

        return Ok(ExprReference {
            attrs: Default::default(),
            and: parser.parse()?,
            mutability: parser.parse()?,
            expr: Box::new(unary_with(parser, attrs, context)?),
        }
        .into());
    }

    if parser.peek::<UnOp>() {
        return Ok(ExprUnary {
            attrs: Default::default(),
            op: parser.parse()?,
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
                index: Delimited::parse_bracket_with(parser, |inner| Ok(Box::new(inner.parse()?)))?,
            }
            .into();

            continue;
        }

        if parser.peek::<Token![.]>() && !parser.peek::<Token![..]>() && !parser.peek::<Token![..=]>() {
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

fn primary(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
    let mut closure_cursor = parser.cursor();
    closure_cursor = BoundLifetimes::skip(closure_cursor).unwrap_or(closure_cursor);
    closure_cursor = closure_cursor.skip::<Option<Token![const]>>().unwrap_or(closure_cursor);
    closure_cursor = closure_cursor.skip::<Option<Token![static]>>().unwrap_or(closure_cursor);
    closure_cursor = closure_cursor.skip::<Option<Token![async]>>().unwrap_or(closure_cursor);
    closure_cursor = Option::<Token![move]>::skip(closure_cursor).unwrap_or(closure_cursor);

    if closure_cursor.peek::<Token![||]>() || closure_cursor.peek::<Token![|]>() {
        return closure(parser, attrs, context);
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
        return paren_or_tuple(parser, attrs);
    }

    if parser.is_delimited(Delim::Bracket) {
        return array_or_repeat(parser, attrs);
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
            expr: Box::new(expr(parser, context)?),
        }
        .into());
    }

    if parser.peek::<Token![if]>() {
        let if_keyword = parser.parse()?;
        let cond = Box::new(expr(parser, ExprContext::EARLY)?);
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
            cond: Box::new(expr(parser, ExprContext::EARLY)?),
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
            expr: Box::new(expr(parser, ExprContext::EARLY)?),
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
            expr: optional_expr(parser, context)?,
        }
        .into());
    }

    if parser.peek::<Token![break]>() {
        return Ok(ExprBreak {
            attrs,
            break_keyword: parser.parse()?,
            label: parser.parse()?,
            expr: optional_expr(parser, context)?,
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
            expr: optional_expr(parser, context)?,
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

        if qself.is_none() && parser.peek::<Token![!]>() && !parser.peek::<Token![!=]>() {
            let mac = MacroCall {
                path,
                bang: parser.parse()?,
                body: parser.parse()?,
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

pub(crate) fn array_or_repeat(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
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

fn closure(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Expr, ParseError> {
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
