use moxy_token::{DelimSpan, Ident, Lit, TokenTree};

use super::peek;
use super::*;

pub(super) fn expr(parser: &Parser, context: ExprContext) -> Result<Expr, ParseError> {
    Ok(*boxed_expr(parser, context)?)
}

// The parse functions return boxed expressions, which every parent node
// stores, so a nested expression's stack frames hold pointers, not nodes
fn boxed_expr(parser: &Parser, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let attrs = <_ as Parse>::parse(parser)?;
    assignment(parser, attrs, context)
}

fn boxed(node: impl Into<Expr>) -> Box<Expr> {
    Box::new(node.into())
}

// A match arm body or an expression statement. A block-like expression
// (`{ .. }`, `if`, `match`, a loop, ..) ends there unless a method call,
// field access, or `?` continues it, so `a => { x } (b, c) => y` holds two
// arms and `if a { b } <T>::f()` two statements.
pub(crate) fn early(parser: &Parser) -> Result<Box<Expr>, ParseError> {
    let attrs = <_ as Parse>::parse(parser)?;

    if !peek::block_like(parser.cursor()) {
        return assignment(parser, attrs, ExprContext::NORMAL);
    }

    let mut body = primary(parser, Attributes::default(), ExprContext::NORMAL)?;

    if !peek::continues_block_like(parser.cursor()) {
        if let Some(slot) = body.attrs_mut() {
            *slot = attrs;
        }

        return Ok(body);
    }

    // The block-like expression parsed once and the rest of the expression
    // continues from it, as in `{ x }.len() + 1`
    let operand = cast_rest(parser, postfix_rest(parser, body, attrs)?)?;
    let left = binary_rest(parser, ExprContext::NORMAL, operand, Precedence::Or, false)?;

    if RangeLimits::peek(parser.cursor()) {
        return range_rest(parser, Default::default(), Some(left), ExprContext::NORMAL);
    }

    assignment_rest(parser, left, ExprContext::NORMAL)
}

fn optional_expr(parser: &Parser, context: ExprContext) -> Result<Option<Box<Expr>>, ParseError> {
    if peek::expr(parser.cursor(), context) {
        Ok(Some(boxed_expr(parser, context)?))
    } else {
        Ok(None)
    }
}

// As in rustc, nothing continues a range at its own level: `a..b = c` is an
// error, while `a = b..c = d` assigns `d` to `a = b..c`
fn assignment(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let left = range(parser, attrs, context)?;

    if matches!(*left, Expr::Range(_)) {
        return Ok(left);
    }

    assignment_rest(parser, left, context)
}

// An assignment whose left side has parsed, or the left side alone
fn assignment_rest(parser: &Parser, left: Box<Expr>, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    if <Token![=]>::peek(parser.cursor()) && !<Token![==]>::peek(parser.cursor()) && !<Token![=>]>::peek(parser.cursor()) {
        return assign(parser, left, context);
    }

    // Compound assignment is right-associative and binds looser than a range:
    // `x += y + z` adds first, `a += b = c` assigns `c` to `b` first
    if peek_binary(parser.cursor()).is_some_and(|op| Precedence::of(&op) == Precedence::Assign) {
        return compound_assign(parser, left, context);
    }

    Ok(left)
}

fn assign(parser: &Parser, left: Box<Expr>, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let eq = <_ as Parse>::parse(parser)?;
    let right = assignment_right(parser, context)?;
    let ranged = matches!(*right, Expr::Range(_));
    let assignment = boxed(ExprAssign {
        attrs: Default::default(),
        left,
        eq,
        right,
    });

    continue_after_range(parser, assignment, ranged, context)
}

fn compound_assign(parser: &Parser, left: Box<Expr>, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let op = <_ as Parse>::parse(parser)?;
    let right = assignment_right(parser, context)?;
    let ranged = matches!(*right, Expr::Range(_));
    let assignment = boxed(ExprBinaryChain::new(*left, op, *right));

    continue_after_range(parser, assignment, ranged, context)
}

// A range that ends the right side of an assignment ends that side alone, so
// an assignment after it takes the whole assignment as its left side
fn continue_after_range(
    parser: &Parser,
    assignment: Box<Expr>,
    ranged: bool,
    context: ExprContext,
) -> Result<Box<Expr>, ParseError> {
    if ranged {
        // The assignment that follows nests the tree one level, as in
        // `a = b.. = c`
        let _depth = moxy_token::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
        assignment_rest(parser, assignment, context)
    } else {
        Ok(assignment)
    }
}

// The right side of an assignment, which nests one level per operator as
// assignment associates to the right
fn assignment_right(parser: &Parser, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let _depth = moxy_token::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
    assignment(parser, Default::default(), context)
}

fn range(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    if RangeLimits::peek(parser.cursor()) {
        return range_rest(parser, attrs, None, context);
    }

    let left = binary(parser, attrs, context, Precedence::Or, false)?;

    if RangeLimits::peek(parser.cursor()) {
        return range_rest(parser, Default::default(), Some(left), context);
    }

    Ok(left)
}

// The limits and end of a range, after its start if it has one
fn range_rest(
    parser: &Parser,
    attrs: Attributes,
    start: Option<Box<Expr>>,
    context: ExprContext,
) -> Result<Box<Expr>, ParseError> {
    let limits = <_ as Parse>::parse(parser)?;
    let mut end = None;

    // The end may itself be a range without a start, as in `a.. ..b`
    if RangeLimits::peek(parser.cursor()) {
        // A range in the end of a range nests one level, as in `.. .. a`
        if peek::nested_range(parser.cursor()) {
            let _depth = moxy_token::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
            end = Some(range_rest(parser, Default::default(), None, context)?);
        }
    } else if !context.is_end(parser.cursor()) && Expr::peek(parser.cursor()) {
        end = Some(binary(parser, Default::default(), context, Precedence::Or, false)?);
    }

    if end.is_none() && matches!(limits, RangeLimits::Closed(_)) {
        return parser.error("an inclusive range needs an end").into();
    }

    Ok(boxed(ExprRange {
        attrs,
        start,
        limits,
        end,
    }))
}

// The binding strength of a binary operator, weakest first
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Precedence {
    Assign,
    Or,
    And,
    Compare,
    BitOr,
    BitXor,
    BitAnd,
    Shift,
    Sum,
    Product,
}

impl Precedence {
    pub(super) fn of(op: &BinOp) -> Self {
        match op {
            BinOp::ShlAssign(_)
            | BinOp::ShrAssign(_)
            | BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_)
            | BinOp::BitXorAssign(_)
            | BinOp::BitAndAssign(_)
            | BinOp::BitOrAssign(_) => Self::Assign,
            BinOp::Or(_) => Self::Or,
            BinOp::And(_) => Self::And,
            BinOp::Eq(_) | BinOp::Ne(_) | BinOp::Lt(_) | BinOp::Le(_) | BinOp::Gt(_) | BinOp::Ge(_) => Self::Compare,
            BinOp::BitOr(_) => Self::BitOr,
            BinOp::BitXor(_) => Self::BitXor,
            BinOp::BitAnd(_) => Self::BitAnd,
            BinOp::Shl(_) | BinOp::Shr(_) => Self::Shift,
            BinOp::Add(_) | BinOp::Sub(_) => Self::Sum,
            BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_) => Self::Product,
        }
    }

    pub(super) fn next(self) -> Option<Self> {
        match self {
            Self::Assign => Some(Self::Or),
            Self::Or => Some(Self::And),
            Self::And => Some(Self::Compare),
            Self::Compare => Some(Self::BitOr),
            Self::BitOr => Some(Self::BitXor),
            Self::BitXor => Some(Self::BitAnd),
            Self::BitAnd => Some(Self::Shift),
            Self::Shift => Some(Self::Sum),
            Self::Sum => Some(Self::Product),
            Self::Product => None,
        }
    }
}

// The binary operator that continues the expression here, if any
pub(super) fn peek_binary(cursor: Cursor<'_>) -> Option<BinOp> {
    if !BinOp::peek(cursor) {
        return None;
    }

    BinOp::parse(&Parser::from_cursor(cursor)).ok()
}

fn binary(
    parser: &Parser,
    attrs: Attributes,
    context: ExprContext,
    min: Precedence,
    nested: bool,
) -> Result<Box<Expr>, ParseError> {
    let left = cast(parser, attrs, context)?;
    binary_rest(parser, context, left, min, nested)
}

// Precedence climbing over the operators at least as strong as `min`. The
// operators of one strength form one chain node and associate to the left;
// comparisons do not chain
fn binary_rest(
    parser: &Parser,
    context: ExprContext,
    mut left: Box<Expr>,
    min: Precedence,
    nested: bool,
) -> Result<Box<Expr>, ParseError> {
    while let Some(op) = peek_binary(parser.cursor()) {
        let precedence = Precedence::of(&op);

        if precedence < min || precedence == Precedence::Assign {
            break;
        }

        // A chain inside an operand of a weaker chain nests a level, which the
        // depth counts; the steps of one chain do not nest
        let _depth = match nested {
            true => Some(moxy_token::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?),
            false => None,
        };
        let mut operands = List::new();
        operands.push_value(*left);

        while peek_binary(parser.cursor()).is_some_and(|op| Precedence::of(&op) == precedence) {
            let op: BinOp = <_ as Parse>::parse(parser)?;
            let right = match precedence.next() {
                Some(stronger) => binary(parser, Default::default(), context, stronger, true)?,
                None => cast(parser, Default::default(), context)?,
            };
            operands.push_punct(op);
            operands.push_value(*right);

            if precedence == Precedence::Compare {
                break;
            }
        }

        left = boxed(ExprBinaryChain {
            attrs: Default::default(),
            operands,
        });

        if precedence == Precedence::Compare
            && let Some(next) = peek_binary(parser.cursor())
            && next.is_comparison()
        {
            return Err(ParseError::new(next.span(), "comparison operators cannot be chained"));
        }
    }

    Ok(left)
}

fn cast(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let expr = unary_with(parser, attrs, context)?;
    cast_rest(parser, expr)
}

// The casts that follow an operand, held in one chain node
fn cast_rest(parser: &Parser, expr: Box<Expr>) -> Result<Box<Expr>, ParseError> {
    if !<Token![as]>::peek(parser.cursor()) {
        return Ok(expr);
    }

    let mut casts = Vec::new();

    while <Token![as]>::peek(parser.cursor()) {
        casts.push((<_ as Parse>::parse(parser)?, Type::parse_without_plus(parser)?));
    }

    Ok(boxed(ExprCastChain {
        attrs: Default::default(),
        expr,
        casts,
    }))
}

pub(crate) fn unary(parser: &Parser, attrs: Attributes) -> Result<Expr, ParseError> {
    Ok(*unary_with(parser, attrs, ExprContext::NORMAL)?)
}

// Outer attributes belong to the operand of a binary operator or cast: the
// unary expression, or the whole chain of postfix operators, that follows them
fn unary_with(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let _depth = moxy_token::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
    let attrs = if attrs.is_empty() {
        <_ as Parse>::parse(parser)?
    } else {
        attrs
    };

    // An operand may be a range without a start, as in `f(&..)` and `a + ..b`
    if RangeLimits::peek(parser.cursor()) {
        return range_rest(parser, attrs, None, context);
    }

    if <Token![&]>::peek(parser.cursor()) {
        return reference(parser, attrs, context);
    }

    if UnOp::peek(parser.cursor()) {
        return unary_op(parser, attrs, context);
    }

    postfix(parser, attrs, context)
}

// `&x`, `&mut x`, or `&raw const x`
fn reference(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    if <Token![raw]>::peek(parser.cursor().offset(1)) && PointerMutability::peek(parser.cursor().offset(2)) {
        return Ok(boxed(ExprRawAddr {
            attrs,
            and: <_ as Parse>::parse(parser)?,
            raw: <_ as Parse>::parse(parser)?,
            mutability: <_ as Parse>::parse(parser)?,
            expr: unary_with(parser, Default::default(), context)?,
        }));
    }

    Ok(boxed(ExprReference {
        attrs,
        and: <_ as Parse>::parse(parser)?,
        mutability: <_ as Parse>::parse(parser)?,
        expr: unary_with(parser, Default::default(), context)?,
    }))
}

fn unary_op(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprUnary {
        attrs,
        op: <_ as Parse>::parse(parser)?,
        expr: unary_with(parser, Default::default(), context)?,
    }))
}

fn postfix(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let expr = primary(parser, Default::default(), context)?;
    postfix_rest(parser, expr, attrs)
}

// The postfix operations that follow a primary expression, held in one
// chain node with the outer attributes, which belong to the whole chain
fn postfix_rest(parser: &Parser, base: Box<Expr>, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    let mut ops = Vec::new();

    loop {
        if parser.is_delimited(Delim::Paren) {
            ops.push(Postfix::Call(PostfixCall {
                args: Delimited::parse_paren_with(parser, List::parse_all)?,
            }));
        } else if parser.is_delimited(Delim::Bracket) {
            ops.push(Postfix::Index(PostfixIndex {
                index: Delimited::parse_bracket_with(parser, |inner| boxed_expr(inner, ExprContext::NORMAL))?,
            }));
        } else if <Token![.]>::peek(parser.cursor()) && !RangeLimits::peek(parser.cursor()) {
            dot(parser, &mut ops)?;
        } else if <Token![?]>::peek(parser.cursor()) {
            ops.push(Postfix::Try(PostfixTry {
                question_punct: <_ as Parse>::parse(parser)?,
            }));
        } else {
            break;
        }
    }

    let mut expr = if ops.is_empty() {
        base
    } else {
        boxed(ExprPostfix {
            attrs: Default::default(),
            base,
            ops,
        })
    };

    if let Some(slot) = expr.attrs_mut() {
        *slot = attrs;
    }

    Ok(expr)
}

// `.await`, a method call, or a field access, or two field accesses where the
// compiler lexes `0.1` in `x.0.1` as one float literal
fn dot(parser: &Parser, ops: &mut Vec<Postfix>) -> Result<(), ParseError> {
    if <Token![await]>::peek(parser.cursor().offset(1)) {
        ops.push(Postfix::Await(PostfixAwait {
            dot: <_ as Parse>::parse(parser)?,
            await_keyword: <_ as Parse>::parse(parser)?,
        }));
        return Ok(());
    }

    if Ident::peek(parser.cursor().offset(1)) {
        // Arguments or a turbofish make a method call; a turbofish makes one
        // even when its arguments are invalid, so their error is reported
        let method = parser.cursor().offset(2);

        if method.is_delimited(Delim::Paren) || <Token![::]>::peek(method) {
            ops.push(Postfix::MethodCall(PostfixMethodCall {
                dot: <_ as Parse>::parse(parser)?,
                method: <_ as Parse>::parse(parser)?,
                turbofish: <_ as Parse>::parse(parser)?,
                args: Delimited::parse_paren_with(parser, List::parse_all)?,
            }));
            return Ok(());
        }
    }

    // `0.` in `x.0. 1` holds one index and the dot before the next
    if let Some(TokenTree::Literal(Lit::Float(float))) = parser.cursor().offset(1).curr()
        && let Some((first, inner_dot, second)) = float.split_tuple_index()
    {
        let dot = <_ as Parse>::parse(parser)?;
        parser.advance();
        ops.push(Postfix::Field(PostfixField {
            dot,
            member: Member::Unnamed(first.into()),
        }));

        let member = match second {
            Some(second) => Member::Unnamed(second.into()),
            None => <_ as Parse>::parse(parser)?,
        };

        ops.push(Postfix::Field(PostfixField { dot: inner_dot, member }));
        return Ok(());
    }

    ops.push(Postfix::Field(PostfixField {
        dot: <_ as Parse>::parse(parser)?,
        member: <_ as Parse>::parse(parser)?,
    }));
    Ok(())
}

// Each form parses in its own function, so a nested expression's stack frame
// holds only the form being parsed
fn primary(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
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
        return lit(parser, attrs);
    }

    if <Token![_]>::peek(parser.cursor()) {
        return infer(parser, attrs);
    }

    if parser.is_delimited(Delim::Paren) {
        return paren_or_tuple(parser, attrs);
    }

    if parser.is_delimited(Delim::Bracket) {
        return array_or_repeat(parser, attrs);
    }

    if parser.is_delimited(Delim::Brace) {
        return block(parser, attrs);
    }

    if parser.is_delimited(Delim::None) {
        return group(parser, attrs);
    }

    if <Token![let]>::peek(parser.cursor()) {
        return let_expr(parser, attrs, context);
    }

    if <Token![if]>::peek(parser.cursor()) {
        return if_expr(parser, attrs);
    }

    let label_cursor = Label::skip(parser.cursor());
    let labeled = |keyword: fn(Cursor<'_>) -> bool| keyword(parser.cursor()) || label_cursor.is_some_and(keyword);

    if labeled(<Token![while]>::peek) {
        return while_loop(parser, attrs);
    }

    if labeled(<Token![for]>::peek) {
        return for_loop(parser, attrs);
    }

    if labeled(<Token![loop]>::peek) {
        return loop_expr(parser, attrs);
    }

    if label_cursor.is_some_and(|cursor| cursor.is_delimited(Delim::Brace)) {
        return labeled_block(parser, attrs);
    }

    if <Token![match]>::peek(parser.cursor()) {
        return match_expr(parser, attrs);
    }

    if <Token![unsafe]>::peek(parser.cursor()) {
        return unsafe_block(parser, attrs);
    }

    if <Token![const]>::peek(parser.cursor()) {
        return const_block(parser, attrs);
    }

    if <Token![async]>::peek(parser.cursor()) {
        return async_block(parser, attrs);
    }

    if <Token![try]>::peek(parser.cursor()) {
        return try_block(parser, attrs);
    }

    if <Token![return]>::peek(parser.cursor()) {
        return return_expr(parser, attrs, context);
    }

    if <Token![break]>::peek(parser.cursor()) {
        return break_expr(parser, attrs, context);
    }

    if <Token![continue]>::peek(parser.cursor()) {
        return continue_expr(parser, attrs);
    }

    if <Token![yield]>::peek(parser.cursor()) {
        return yield_expr(parser, attrs, context);
    }

    if <Token![<]>::peek(parser.cursor()) || Path::peek(parser.cursor()) {
        return path_macro_or_struct(parser, attrs, context);
    }

    parser.error("expected expression").into()
}

fn lit(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprLit {
        attrs,
        lit: <_ as Parse>::parse(parser)?,
    }))
}

fn infer(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprInfer {
        attrs,
        underscore: <_ as Parse>::parse(parser)?,
    }))
}

fn block(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprBlock {
        attrs,
        label: None,
        block: <_ as Parse>::parse(parser)?,
    }))
}

fn group(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    let inner = parser.parse_group(Delim::None)?;

    Ok(boxed(ExprGroup {
        attrs,
        expr: boxed_expr(&inner, ExprContext::NORMAL)?,
    }))
}

// The scrutinee binds tighter than `&&` and `||`, which chain conditions:
// `if let Some(x) = a && b {}` tests `a` against the pattern, then `b`
fn let_expr(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprLet {
        attrs,
        let_keyword: <_ as Parse>::parse(parser)?,
        pat: <_ as Parse>::parse(parser)?,
        eq: <_ as Parse>::parse(parser)?,
        expr: binary(parser, Attributes::default(), context, Precedence::Compare, false)?,
    }))
}

fn if_expr(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    let if_keyword = <_ as Parse>::parse(parser)?;
    let cond = boxed_expr(parser, ExprContext::EARLY)?;
    let then_branch: StmtBlock = <_ as Parse>::parse(parser)?;
    if_block_inner(&then_branch)?;
    let mut else_ifs = Vec::new();

    // `else` takes another `if`, held as a branch of this node, or a block,
    // which ends the expression: `if a { 1 } else { 2 } [x]` does not index
    // the else block
    while <Token![else]>::peek(parser.cursor()) && <Token![if]>::peek(parser.cursor().offset(1)) {
        let else_keyword = <_ as Parse>::parse(parser)?;
        let if_keyword = <_ as Parse>::parse(parser)?;
        let cond = boxed_expr(parser, ExprContext::EARLY)?;
        let then_branch: StmtBlock = <_ as Parse>::parse(parser)?;
        if_block_inner(&then_branch)?;
        else_ifs.push(ElseIf {
            else_keyword,
            if_keyword,
            cond,
            then_branch,
        });
    }

    let (else_keyword, else_branch) = if <Token![else]>::peek(parser.cursor()) {
        let else_keyword = <_ as Parse>::parse(parser)?;

        if !parser.is_delimited(Delim::Brace) {
            return parser.error("expected `{` or `if` after `else`").into();
        }

        let else_branch: StmtBlock = <_ as Parse>::parse(parser)?;
        if_block_inner(&else_branch)?;
        (Some(else_keyword), Some(else_branch))
    } else {
        (None, None)
    };

    Ok(boxed(ExprIfChain {
        attrs,
        if_keyword,
        cond,
        then_branch,
        else_ifs,
        else_keyword,
        else_branch,
    }))
}

// As in rustc, the blocks of an `if` take no inner attributes
fn if_block_inner(block: &StmtBlock) -> Result<(), ParseError> {
    match block.inner_attrs.iter().next() {
        Some(attr) => Err(ParseError::new(
            attr.span(),
            "an inner attribute is not permitted in this context",
        )),
        None => Ok(()),
    }
}

fn while_loop(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprWhile {
        attrs,
        label: <_ as Parse>::parse(parser)?,
        while_keyword: <_ as Parse>::parse(parser)?,
        cond: boxed_expr(parser, ExprContext::EARLY)?,
        body: <_ as Parse>::parse(parser)?,
    }))
}

fn for_loop(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprForLoop {
        attrs,
        label: <_ as Parse>::parse(parser)?,
        for_keyword: <_ as Parse>::parse(parser)?,
        pat: <_ as Parse>::parse(parser)?,
        in_keyword: <_ as Parse>::parse(parser)?,
        expr: boxed_expr(parser, ExprContext::EARLY)?,
        body: <_ as Parse>::parse(parser)?,
    }))
}

fn loop_expr(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprLoop {
        attrs,
        label: <_ as Parse>::parse(parser)?,
        loop_keyword: <_ as Parse>::parse(parser)?,
        body: <_ as Parse>::parse(parser)?,
    }))
}

fn labeled_block(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprBlock {
        attrs,
        label: <_ as Parse>::parse(parser)?,
        block: <_ as Parse>::parse(parser)?,
    }))
}

fn match_expr(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    let match_keyword = <_ as Parse>::parse(parser)?;
    let expr = boxed_expr(parser, ExprContext::EARLY)?;
    let (span, arms) = parser.parse_group_spanned(Delim::Brace)?;
    let inner = Attributes::parse_inner(&arms)?;

    Ok(boxed(ExprMatch {
        attrs,
        match_keyword,
        expr,
        inner_attrs: inner,
        arms: Delimited::brace(span, arms.parse_until_empty()?),
    }))
}

fn unsafe_block(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprUnsafe {
        attrs,
        unsafe_keyword: <_ as Parse>::parse(parser)?,
        block: <_ as Parse>::parse(parser)?,
    }))
}

fn const_block(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprConst {
        attrs,
        const_keyword: <_ as Parse>::parse(parser)?,
        block: <_ as Parse>::parse(parser)?,
    }))
}

fn async_block(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprAsync {
        attrs,
        async_keyword: <_ as Parse>::parse(parser)?,
        move_keyword: <_ as Parse>::parse(parser)?,
        block: <_ as Parse>::parse(parser)?,
    }))
}

fn try_block(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprTryBlock {
        attrs,
        try_keyword: <_ as Parse>::parse(parser)?,
        block: <_ as Parse>::parse(parser)?,
    }))
}

fn return_expr(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprReturn {
        attrs,
        return_keyword: <_ as Parse>::parse(parser)?,
        expr: optional_expr(parser, context)?,
    }))
}

fn break_expr(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprBreak {
        attrs,
        break_keyword: <_ as Parse>::parse(parser)?,
        label: <_ as Parse>::parse(parser)?,
        expr: optional_expr(parser, context)?,
    }))
}

fn continue_expr(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprContinue {
        attrs,
        continue_keyword: <_ as Parse>::parse(parser)?,
        label: <_ as Parse>::parse(parser)?,
    }))
}

fn yield_expr(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    Ok(boxed(ExprYield {
        attrs,
        yield_keyword: <_ as Parse>::parse(parser)?,
        expr: optional_expr(parser, context)?,
    }))
}

fn path_macro_or_struct(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let (qself, path) = if <Token![<]>::peek(parser.cursor()) {
        let (qself, path) = QSelf::parse_qualified_expr(parser)?;
        (Some(qself), path)
    } else {
        (None, Path::parse_expr(parser)?)
    };

    if qself.is_none() && <Token![!]>::peek(parser.cursor()) && !<Token![!=]>::peek(parser.cursor()) {
        let mac = MacroCall {
            path,
            bang: <_ as Parse>::parse(parser)?,
            body: <_ as Parse>::parse(parser)?,
        };

        return Ok(boxed(ExprMacro { attrs, mac }));
    }

    if context.allow_struct && parser.is_delimited(Delim::Brace) {
        // A struct literal takes two levels of stack, as in `S { a: S { .. } }`
        let _weight = moxy_token::Depth::enter_by(1).ok_or_else(|| ParseError::too_deep(parser.span()))?;

        return Ok(boxed(ExprStruct {
            attrs,
            qself,
            path,
            body: Delimited::parse_brace(parser)?,
        }));
    }

    Ok(boxed(ExprPath { attrs, qself, path }))
}

pub(crate) fn paren_or_tuple(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;
    let expr = paren_or_tuple_content(&parser, span, attrs)?;
    parser.ensure_empty()?;
    Ok(expr)
}

fn paren_or_tuple_content(parser: &Parser, span: DelimSpan, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    if parser.is_empty() {
        return Ok(boxed(ExprTuple {
            attrs,
            elems: Delimited::paren(span, Default::default()),
        }));
    }

    let first = boxed_expr(parser, ExprContext::NORMAL)?;

    if <Token![,]>::peek(parser.cursor()) {
        return tuple(parser, span, attrs, first);
    }

    Ok(boxed(ExprParen {
        attrs,
        content: Delimited::paren(span, first),
    }))
}

fn tuple(parser: &Parser, span: DelimSpan, attrs: Attributes, first: Box<Expr>) -> Result<Box<Expr>, ParseError> {
    let mut elems = List::new();
    elems.push_value(*first);

    while <Token![,]>::peek(parser.cursor()) {
        elems.push_punct(<_ as Parse>::parse(parser)?);

        if !parser.is_empty() {
            elems.push_value(*boxed_expr(parser, ExprContext::NORMAL)?);
        }
    }

    Ok(boxed(ExprTuple {
        attrs,
        elems: Delimited::paren(span, elems),
    }))
}

pub(crate) fn array_or_repeat(parser: &Parser, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    let (span, parser) = parser.parse_group_spanned(Delim::Bracket)?;
    let expr = array_or_repeat_content(&parser, span, attrs)?;
    parser.ensure_empty()?;
    Ok(expr)
}

fn array_or_repeat_content(parser: &Parser, span: DelimSpan, attrs: Attributes) -> Result<Box<Expr>, ParseError> {
    if parser.is_empty() {
        return Ok(boxed(ExprArray {
            attrs,
            elems: Delimited::bracket(span, Default::default()),
        }));
    }

    let first = boxed_expr(parser, ExprContext::NORMAL)?;

    if <Token![;]>::peek(parser.cursor()) {
        return Ok(boxed(ExprRepeat {
            attrs,
            content: Delimited::bracket(
                span,
                RepeatInner {
                    elem: first,
                    semi: <_ as Parse>::parse(parser)?,
                    len: boxed_expr(parser, ExprContext::NORMAL)?,
                },
            ),
        }));
    }

    array(parser, span, attrs, first)
}

fn array(parser: &Parser, span: DelimSpan, attrs: Attributes, first: Box<Expr>) -> Result<Box<Expr>, ParseError> {
    let mut elems = List::new();
    elems.push_value(*first);

    while <Token![,]>::peek(parser.cursor()) {
        elems.push_punct(<_ as Parse>::parse(parser)?);

        if !parser.is_empty() {
            elems.push_value(*boxed_expr(parser, ExprContext::NORMAL)?);
        }
    }

    Ok(boxed(ExprArray {
        attrs,
        elems: Delimited::bracket(span, elems),
    }))
}

fn closure(parser: &Parser, attrs: Attributes, context: ExprContext) -> Result<Box<Expr>, ParseError> {
    let lifetimes = <_ as Parse>::parse(parser)?;
    let constness = <_ as Parse>::parse(parser)?;
    let movability = <_ as Parse>::parse(parser)?;
    let asyncness = <_ as Parse>::parse(parser)?;
    let capture = <_ as Parse>::parse(parser)?;
    let (pipes, inputs) = closure_params(parser)?;
    let output: ReturnType = <_ as Parse>::parse(parser)?;
    // A closure with a return type and a block takes another level of stack
    let _weight = if output.is_type() {
        Some(moxy_token::Depth::enter_by(1).ok_or_else(|| ParseError::too_deep(parser.span()))?)
    } else {
        None
    };

    // With a return type the body is a block, which ends the closure: in
    // `|| -> u8 { 1 }.x` the field belongs to the closure
    let body = if output.is_type() {
        if !parser.is_delimited(Delim::Brace) {
            return parser.error("expected a block after the closure's return type").into();
        }

        block(parser, Attributes::default())?
    } else {
        boxed_expr(parser, context)?
    };

    Ok(boxed(ExprClosure {
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
    }))
}

// The pipes and parameters of a closure, in their own stack frame, which the
// closure body does not hold while it parses
fn closure_params(parser: &Parser) -> Result<(ClosurePipes, List<ClosureParam, Token![,]>), ParseError> {
    if <Token![||]>::peek(parser.cursor()) {
        let oror = <_ as Parse>::parse(parser)?;
        return Ok((ClosurePipes::Empty(oror), List::new()));
    }

    let open = <_ as Parse>::parse(parser)?;
    let mut params = List::new();

    while !<Token![|]>::peek(parser.cursor()) && !parser.is_empty() {
        params.push_value(<_ as Parse>::parse(parser)?);

        if <Token![,]>::peek(parser.cursor()) {
            params.push_punct(<_ as Parse>::parse(parser)?);
        } else {
            break;
        }
    }

    let close = <_ as Parse>::parse(parser)?;
    Ok((ClosurePipes::Params(open, close), params))
}
