use moxy_token::{Ident, Lit, TokenTree};

use super::parse::{Precedence, peek_binary};
use super::peek;
use super::*;

pub(super) fn expr(cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    assignment(Attributes::skip(cursor)?, context)
}

fn optional_expr(cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    if peek::expr(cursor, context) {
        expr(cursor, context)
    } else {
        Some(cursor)
    }
}

pub(crate) fn pattern_bound(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    unary(cursor, ExprContext::NORMAL)
}

pub(crate) fn early(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    let start = Attributes::skip(cursor)?;

    if peek::block_like(start)
        && let Some(end) = primary(start, ExprContext::NORMAL)
        && !peek::continues_block_like(end)
    {
        return Some(end);
    }

    expr(cursor, ExprContext::NORMAL)
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

        // With a return type the body is a block, as in the parser
        if <Token![->]>::peek(cursor) {
            return StmtBlock::skip(ReturnType::skip(cursor)?);
        }

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

            if !cursor.is_delimited(Delim::Brace) && !<Token![if]>::peek(cursor) {
                return None;
            }

            let _depth = moxy_token::Depth::enter()?;
            cursor = primary(cursor, ExprContext::NORMAL)?;
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
        cursor = Option::<Lifetime>::skip(cursor)?;
        return optional_expr(cursor, context);
    }

    if <Token![continue]>::peek(cursor) {
        return Option::<Lifetime>::skip(<Token![continue]>::skip(cursor)?);
    }

    if <Token![yield]>::peek(cursor) {
        cursor = <Token![yield]>::skip(cursor)?;
        return optional_expr(cursor, context);
    }

    let path_start = cursor;
    let qualified = <Token![<]>::peek(cursor);

    if !qualified && !Path::peek(cursor) {
        return None;
    }

    cursor = if qualified {
        ty::TypePath::skip_expr(cursor)?
    } else {
        Path::skip_expr(cursor)?
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

            // Arguments or a turbofish make a method call, as in the parser
            if Ident::peek(cursor)
                && let Some(after_method) = Ident::skip(cursor)
                && (after_method.is_delimited(Delim::Paren) || <Token![::]>::peek(after_method))
            {
                let args = Option::<AngleArguments>::skip(after_method)?;

                {
                    let inner = list(args.descend(Delim::Paren)?)?;

                    if !inner.is_empty() {
                        return None;
                    }

                    cursor = args.offset(1);
                    continue;
                }
            }

            // `0.1` in `x.0.1` is two tuple indices in one float literal, and `0.`
            // in `x.0. 1` one and the dot before the next
            if let Some(TokenTree::Literal(Lit::Float(float))) = cursor.curr()
                && let Some((_, _, second)) = float.split_tuple_index()
            {
                cursor = cursor.offset(1);

                if second.is_none() {
                    cursor = Member::skip(cursor)?;
                }

                continue;
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
    let _depth = moxy_token::Depth::enter()?;
    let cursor = Attributes::skip(cursor)?;

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
        cursor = Type::skip_without_plus(cursor)?;
    }

    Some(cursor)
}

fn binary(cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    binary_rest(cast(cursor, context)?, context, Precedence::Or)
}

// Mirrors the precedence climbing of the parser, which rejects chained
// comparisons and leaves compound assignment to the assignment level
fn binary_rest(mut cursor: Cursor<'_>, context: ExprContext, min: Precedence) -> Option<Cursor<'_>> {
    while let Some(op) = peek_binary(cursor) {
        let precedence = Precedence::of(&op);

        if precedence < min || precedence == Precedence::Assign {
            break;
        }

        cursor = cast(BinOp::skip(cursor)?, context)?;

        if let Some(stronger) = precedence.next() {
            cursor = binary_rest(cursor, context, stronger)?;
        }

        if precedence == Precedence::Compare && peek_binary(cursor).is_some_and(|next| next.is_comparison()) {
            return None;
        }
    }

    Some(cursor)
}

// Skips a range or the operand of one, and tells whether a range ended it
fn range(mut cursor: Cursor<'_>, context: ExprContext) -> Option<(Cursor<'_>, bool)> {
    if !RangeLimits::peek(cursor) {
        cursor = binary(cursor, context)?;

        if !RangeLimits::peek(cursor) {
            return Some((cursor, false));
        }
    }

    // An inclusive range needs an end, as in the parser
    let closed = <Token![..=]>::peek(cursor);
    cursor = RangeLimits::skip(cursor)?;

    if peek::nested_range(cursor) {
        return range(cursor, context);
    }

    if !context.is_end(cursor) && peek::expr(cursor, context) {
        return Some((binary(cursor, context)?, true));
    }

    (!closed).then_some((cursor, true))
}

fn assignment(cursor: Cursor<'_>, context: ExprContext) -> Option<Cursor<'_>> {
    assignment_ranged(cursor, context).map(|(cursor, _)| cursor)
}

// Skips an assignment as the parser parses it, and tells whether a range
// ended it
fn assignment_ranged(cursor: Cursor<'_>, context: ExprContext) -> Option<(Cursor<'_>, bool)> {
    let (mut cursor, ranged) = range(cursor, context)?;

    if ranged {
        return Some((cursor, true));
    }

    loop {
        let assign = <Token![=]>::peek(cursor) && !<Token![==]>::peek(cursor) && !<Token![=>]>::peek(cursor);
        let compound = peek_binary(cursor).is_some_and(|op| Precedence::of(&op) == Precedence::Assign);

        if !assign && !compound {
            return Some((cursor, false));
        }

        cursor = if assign {
            <Token![=]>::skip(cursor)?
        } else {
            BinOp::skip(cursor)?
        };
        let _depth = moxy_token::Depth::enter()?;
        let (next, ranged) = assignment_ranged(cursor, context)?;
        cursor = next;

        if !ranged {
            return Some((cursor, false));
        }
    }
}
