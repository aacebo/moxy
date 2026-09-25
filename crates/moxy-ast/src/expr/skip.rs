use super::peek;
use super::*;

pub(super) fn expr(cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    if context.pattern_bound {
        unary(cursor, context)
    } else {
        assignment(Attributes::skip(cursor)?, context)
    }
}

fn optional_expr(cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    if peek::expr(cursor, context) {
        expr(cursor, context)
    } else {
        Some(cursor)
    }
}

pub(crate) fn pattern_bound(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    expr(cursor, ExprContext::PATTERN_BOUND)
}

pub(crate) fn const_generic_default(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    expr(cursor, ExprContext::CONST_GENERIC_DEFAULT)
}

fn list(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    while !cursor.is_empty() {
        cursor = cursor.skip::<Expr>()?;

        if cursor.is_empty() {
            break;
        }

        cursor = cursor.skip::<Token![,]>()?;
    }

    Some(cursor)
}

fn primary(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    let mut closure = cursor;
    closure = BoundLifetimes::skip(closure).unwrap_or(closure);
    closure = closure.skip::<Option<Token![const]>>()?;
    closure = closure.skip::<Option<Token![static]>>()?;
    closure = closure.skip::<Option<Token![async]>>()?;
    closure = closure.skip::<Option<Token![move]>>()?;

    if closure.peek::<Token![||]>() || closure.peek::<Token![|]>() {
        cursor = BoundLifetimes::skip(cursor).unwrap_or(cursor);
        cursor = cursor.skip::<Option<Token![const]>>()?;
        cursor = cursor.skip::<Option<Token![static]>>()?;
        cursor = cursor.skip::<Option<Token![async]>>()?;
        cursor = cursor.skip::<Option<Token![move]>>()?;

        if cursor.peek::<Token![||]>() {
            cursor = cursor.skip::<Token![||]>()?;
        } else {
            cursor = cursor.skip::<Token![|]>()?;

            while !cursor.peek::<Token![|]>() {
                cursor = cursor.skip::<ClosureParam>()?;

                if cursor.peek::<Token![,]>() {
                    cursor = cursor.skip::<Token![,]>()?;
                } else {
                    break;
                }
            }

            cursor = cursor.skip::<Token![|]>()?;
        }

        cursor = ReturnType::skip(cursor)?;
        return expr(cursor, context);
    }

    if cursor.peek::<Lit>() {
        return cursor.skip::<Lit>();
    }

    if cursor.peek::<Token![_]>() {
        return cursor.skip::<Token![_]>();
    }

    if cursor.is_delimited(Delim::Paren) {
        let inner = cursor.descend(Delim::Paren)?;
        let inner = list(inner)?;
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
        return expr(cursor, context);
    }

    if cursor.peek::<Token![if]>() {
        cursor = cursor.skip::<Token![if]>()?;
        cursor = expr(cursor, ExprContext::EARLY)?;
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
        cursor = expr(cursor, ExprContext::EARLY)?;
        return cursor.skip::<StmtBlock>();
    }

    if cursor.peek::<Token![for]>() || label.is_some_and(|cursor| cursor.peek::<Token![for]>()) {
        cursor = cursor.skip::<Option<Label>>()?;
        cursor = cursor.skip::<Token![for]>()?;
        cursor = cursor.skip::<Pattern>()?;
        cursor = cursor.skip::<Token![in]>()?;
        cursor = expr(cursor, ExprContext::EARLY)?;
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
        cursor = expr(cursor, ExprContext::EARLY)?;
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
        cursor = cursor.skip::<Token![return]>()?;
        return optional_expr(cursor, context);
    }

    if cursor.peek::<Token![break]>() {
        cursor = cursor.skip::<Token![break]>()?;
        cursor = cursor.skip::<Option<Label>>()?;
        return optional_expr(cursor, context);
    }

    if cursor.peek::<Token![continue]>() {
        return cursor.skip::<Token![continue]>()?.skip::<Option<Label>>();
    }

    if cursor.peek::<Token![yield]>() {
        cursor = cursor.skip::<Token![yield]>()?;
        return optional_expr(cursor, context);
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

    if !qualified && cursor.peek::<Token![!]>() && !cursor.peek::<Token![!=]>() {
        return MacroCall::skip(path_start);
    }

    if context.allow_struct && cursor.is_delimited(Delim::Brace) {
        let inner = cursor.descend(Delim::Brace)?.skip::<StructBody>()?;
        return inner.is_empty().then(|| cursor.offset(1));
    }

    Some(cursor)
}

fn postfix(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    cursor = primary(cursor, context)?;

    loop {
        if cursor.is_delimited(Delim::Paren) {
            let inner = list(cursor.descend(Delim::Paren)?)?;

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

        if cursor.peek::<Token![.]>() && !cursor.peek::<Token![..]>() && !cursor.peek::<Token![..=]>() {
            cursor = cursor.skip::<Token![.]>()?;

            if cursor.peek::<Token![await]>() {
                cursor = cursor.skip::<Token![await]>()?;
                continue;
            }

            if cursor.peek::<Ident>() {
                let after_method = cursor.skip::<Ident>()?;
                let args = after_method.skip::<Option<AngleArguments>>()?;

                if args.is_delimited(Delim::Paren) {
                    let inner = list(args.descend(Delim::Paren)?)?;

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

fn unary(cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    if cursor.peek::<Token![&]>() && cursor.offset(1).peek::<Token![raw]>() && cursor.offset(2).peek::<PointerMutability>() {
        return cursor
            .skip::<Token![&]>()?
            .skip::<Token![raw]>()?
            .skip::<PointerMutability>()
            .and_then(|cursor| unary(cursor, context));
    }

    if cursor.peek::<Token![&]>() {
        return cursor
            .skip::<Token![&]>()?
            .skip::<Option<Token![mut]>>()
            .and_then(|cursor| unary(cursor, context));
    }

    if cursor.peek::<UnOp>() {
        return cursor.skip::<UnOp>().and_then(|cursor| unary(cursor, context));
    }

    postfix(cursor, context)
}

fn cast(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    cursor = unary(cursor, context)?;

    while cursor.peek::<Token![as]>() {
        cursor = cursor.skip::<Token![as]>()?;
        cursor = cursor.skip::<Type>()?;
    }

    Some(cursor)
}

fn binary(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    cursor = cast(cursor, context)?;

    while cursor.peek::<BinOp>() && !context.is_end_before_infix(cursor) {
        cursor = cursor.skip::<BinOp>()?;
        cursor = cast(cursor, context)?;
    }

    Some(cursor)
}

fn range(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    if cursor.peek::<RangeLimits>() {
        cursor = cursor.skip::<RangeLimits>()?;

        if !context.is_end(cursor) && peek::expr(cursor, context) {
            cursor = binary(cursor, context)?;
        }

        return Some(cursor);
    }

    cursor = binary(cursor, context)?;

    if cursor.peek::<RangeLimits>() {
        cursor = cursor.skip::<RangeLimits>()?;

        if !context.is_end(cursor) && peek::expr(cursor, context) {
            cursor = binary(cursor, context)?;
        }
    }

    Some(cursor)
}

fn assignment(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    cursor = range(cursor, context)?;

    if cursor.peek::<Token![=]>() {
        cursor = cursor.skip::<Token![=]>()?;
        cursor = assignment(cursor, context)?;
    }

    Some(cursor)
}
