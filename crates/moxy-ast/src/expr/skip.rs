use moxy_token::{Ident, Lit};

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
        cursor = Expr::skip(cursor)?;

        if cursor.is_empty() {
            break;
        }

        cursor = <Token![,]>::skip(cursor)?;
    }

    Some(cursor)
}

fn primary(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    let mut closure = cursor;
    closure = BoundLifetimes::skip(closure).unwrap_or(closure);
    closure = Option::<Token![const]>::skip(closure)?;
    closure = Option::<Token![static]>::skip(closure)?;
    closure = Option::<Token![async]>::skip(closure)?;
    closure = Option::<Token![move]>::skip(closure)?;

    if <Token![||]>::peek(closure) || <Token![|]>::peek(closure) {
        cursor = BoundLifetimes::skip(cursor).unwrap_or(cursor);
        cursor = Option::<Token![const]>::skip(cursor)?;
        cursor = Option::<Token![static]>::skip(cursor)?;
        cursor = Option::<Token![async]>::skip(cursor)?;
        cursor = Option::<Token![move]>::skip(cursor)?;

        if <Token![||]>::peek(cursor) {
            cursor = <Token![||]>::skip(cursor)?;
        } else {
            cursor = <Token![|]>::skip(cursor)?;

            while !<Token![|]>::peek(cursor) {
                cursor = ClosureParam::skip(cursor)?;

                if <Token![,]>::peek(cursor) {
                    cursor = <Token![,]>::skip(cursor)?;
                } else {
                    break;
                }
            }

            cursor = <Token![|]>::skip(cursor)?;
        }

        cursor = ReturnType::skip(cursor)?;
        return expr(cursor, context);
    }

    if Lit::peek(cursor) {
        return Lit::skip(cursor);
    }

    if <Token![_]>::peek(cursor) {
        return <Token![_]>::skip(cursor);
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

        inner = Expr::skip(inner)?;

        if <Token![;]>::peek(inner) {
            inner = <Token![;]>::skip(inner)?;
            inner = Expr::skip(inner)?;
        } else {
            while !inner.is_empty() {
                inner = <Token![,]>::skip(inner)?;

                if !inner.is_empty() {
                    inner = Expr::skip(inner)?;
                }
            }
        }

        return inner.is_empty().then(|| cursor.offset(1));
    }

    if cursor.is_delimited(Delim::Brace) {
        return StmtBlock::skip(cursor);
    }

    if cursor.is_delimited(Delim::None) {
        let inner = Expr::skip(cursor.descend(Delim::None)?)?;
        return inner.is_empty().then(|| cursor.offset(1));
    }

    if <Token![let]>::peek(cursor) {
        cursor = <Token![let]>::skip(cursor)?;
        cursor = Pattern::skip(cursor)?;
        cursor = <Token![=]>::skip(cursor)?;
        return expr(cursor, context);
    }

    if <Token![if]>::peek(cursor) {
        cursor = <Token![if]>::skip(cursor)?;
        cursor = expr(cursor, ExprContext::EARLY)?;
        cursor = StmtBlock::skip(cursor)?;

        if <Token![else]>::peek(cursor) {
            cursor = <Token![else]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

        return Some(cursor);
    }

    let label = Label::skip(cursor);

    if <Token![while]>::peek(cursor) || label.is_some_and(|cursor| <Token![while]>::peek(cursor)) {
        cursor = Option::<Label>::skip(cursor)?;
        cursor = <Token![while]>::skip(cursor)?;
        cursor = expr(cursor, ExprContext::EARLY)?;
        return StmtBlock::skip(cursor);
    }

    if <Token![for]>::peek(cursor) || label.is_some_and(|cursor| <Token![for]>::peek(cursor)) {
        cursor = Option::<Label>::skip(cursor)?;
        cursor = <Token![for]>::skip(cursor)?;
        cursor = Pattern::skip(cursor)?;
        cursor = <Token![in]>::skip(cursor)?;
        cursor = expr(cursor, ExprContext::EARLY)?;
        return StmtBlock::skip(cursor);
    }

    if <Token![loop]>::peek(cursor) || label.is_some_and(|cursor| <Token![loop]>::peek(cursor)) {
        cursor = Option::<Label>::skip(cursor)?;
        cursor = <Token![loop]>::skip(cursor)?;
        return StmtBlock::skip(cursor);
    }

    if label.is_some_and(|cursor| cursor.is_delimited(Delim::Brace)) {
        cursor = Label::skip(cursor)?;
        return StmtBlock::skip(cursor);
    }

    if <Token![match]>::peek(cursor) {
        cursor = <Token![match]>::skip(cursor)?;
        cursor = expr(cursor, ExprContext::EARLY)?;
        let mut inner = cursor.descend(Delim::Brace)?;

        while !inner.is_empty() {
            inner = MatchArm::skip(inner)?;
        }

        return Some(cursor.offset(1));
    }

    if <Token![unsafe]>::peek(cursor) {
        return StmtBlock::skip(<Token![unsafe]>::skip(cursor)?);
    }

    if <Token![const]>::peek(cursor) {
        return StmtBlock::skip(<Token![const]>::skip(cursor)?);
    }

    if <Token![async]>::peek(cursor) {
        cursor = <Token![async]>::skip(cursor)?;
        cursor = Option::<Token![move]>::skip(cursor)?;
        return StmtBlock::skip(cursor);
    }

    if <Token![try]>::peek(cursor) {
        return StmtBlock::skip(<Token![try]>::skip(cursor)?);
    }

    if <Token![return]>::peek(cursor) {
        cursor = <Token![return]>::skip(cursor)?;
        return optional_expr(cursor, context);
    }

    if <Token![break]>::peek(cursor) {
        cursor = <Token![break]>::skip(cursor)?;
        cursor = Option::<Label>::skip(cursor)?;
        return optional_expr(cursor, context);
    }

    if <Token![continue]>::peek(cursor) {
        return Option::<Label>::skip(<Token![continue]>::skip(cursor)?);
    }

    if <Token![yield]>::peek(cursor) {
        cursor = <Token![yield]>::skip(cursor)?;
        return optional_expr(cursor, context);
    }

    let path_start = cursor;
    let qualified = <Token![<]>::peek(cursor);

    if !qualified && !Path::peek(cursor) {
        return Some(cursor.offset(cursor.remaining()));
    }

    cursor = if qualified {
        ty::TypePath::skip(cursor)?
    } else {
        Path::skip(cursor)?
    };

    if !qualified && <Token![!]>::peek(cursor) && !<Token![!=]>::peek(cursor) {
        return MacroCall::skip(path_start);
    }

    if context.allow_struct && cursor.is_delimited(Delim::Brace) {
        let inner = StructBody::skip(cursor.descend(Delim::Brace)?)?;
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
            let inner = Expr::skip(cursor.descend(Delim::Bracket)?)?;

            if !inner.is_empty() {
                return None;
            }

            cursor = cursor.offset(1);
            continue;
        }

        if <Token![.]>::peek(cursor) && !<Token![..]>::peek(cursor) && !<Token![..=]>::peek(cursor) {
            cursor = <Token![.]>::skip(cursor)?;

            if <Token![await]>::peek(cursor) {
                cursor = <Token![await]>::skip(cursor)?;
                continue;
            }

            if Ident::peek(cursor) {
                let after_method = Ident::skip(cursor)?;
                let args = Option::<AngleArguments>::skip(after_method)?;

                if args.is_delimited(Delim::Paren) {
                    let inner = list(args.descend(Delim::Paren)?)?;

                    if !inner.is_empty() {
                        return None;
                    }

                    cursor = args.offset(1);
                    continue;
                }
            }

            cursor = Member::skip(cursor)?;
            continue;
        }

        if <Token![?]>::peek(cursor) {
            cursor = <Token![?]>::skip(cursor)?;
            continue;
        }

        break;
    }

    Some(cursor)
}

fn unary(cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    if <Token![&]>::peek(cursor) && <Token![raw]>::peek(cursor.offset(1)) && PointerMutability::peek(cursor.offset(2)) {
        return PointerMutability::skip(<Token![raw]>::skip(<Token![&]>::skip(cursor)?)?)
            .and_then(|cursor| unary(cursor, context));
    }

    if <Token![&]>::peek(cursor) {
        return Option::<Token![mut]>::skip(<Token![&]>::skip(cursor)?).and_then(|cursor| unary(cursor, context));
    }

    if UnOp::peek(cursor) {
        return UnOp::skip(cursor).and_then(|cursor| unary(cursor, context));
    }

    postfix(cursor, context)
}

fn cast(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    cursor = unary(cursor, context)?;

    while <Token![as]>::peek(cursor) {
        cursor = <Token![as]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;
    }

    Some(cursor)
}

fn binary(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    cursor = cast(cursor, context)?;

    while BinOp::peek(cursor) && !context.is_end_before_infix(cursor) {
        cursor = BinOp::skip(cursor)?;
        cursor = cast(cursor, context)?;
    }

    Some(cursor)
}

fn range(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    if RangeLimits::peek(cursor) {
        cursor = RangeLimits::skip(cursor)?;

        if !context.is_end(cursor) && peek::expr(cursor, context) {
            cursor = binary(cursor, context)?;
        }

        return Some(cursor);
    }

    cursor = binary(cursor, context)?;

    if RangeLimits::peek(cursor) {
        cursor = RangeLimits::skip(cursor)?;

        if !context.is_end(cursor) && peek::expr(cursor, context) {
            cursor = binary(cursor, context)?;
        }
    }

    Some(cursor)
}

fn assignment(mut cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    cursor = range(cursor, context)?;

    if <Token![=]>::peek(cursor) {
        cursor = <Token![=]>::skip(cursor)?;
        cursor = assignment(cursor, context)?;
    }

    Some(cursor)
}
