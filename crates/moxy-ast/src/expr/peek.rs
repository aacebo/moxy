use super::*;

// An optional operand, as of `return` or `break`, is absent before a token
// that ends the enclosing expression: `match a { _ => return, b => b }`
pub(super) fn expr(cursor: Cursor<'_>, _context: ExprContext) -> bool {
    let cursor = Attributes::skip(cursor).unwrap_or(cursor);
    !cursor.is_empty() && !<Token![;]>::peek(cursor) && !<Token![,]>::peek(cursor) && !<Token![=>]>::peek(cursor)
}

// Whether a block-like expression, which ends a match arm without a comma and
// a statement without a semicolon, starts here
pub(super) fn block_like(cursor: Cursor<'_>) -> bool {
    let cursor = Label::skip(cursor).unwrap_or(cursor);

    cursor.is_delimited(Delim::Brace)
        || <Token![if]>::peek(cursor)
        || <Token![match]>::peek(cursor)
        || <Token![loop]>::peek(cursor)
        || <Token![while]>::peek(cursor)
        || <Token![for]>::peek(cursor)
        || (<Token![unsafe]>::peek(cursor) && cursor.offset(1).is_delimited(Delim::Brace))
        || (<Token![const]>::peek(cursor) && cursor.offset(1).is_delimited(Delim::Brace))
}

// Whether a block-like expression continues as a method call, field access,
// or `?` instead of ending
pub(crate) fn continues_block_like(cursor: Cursor<'_>) -> bool {
    (<Token![.]>::peek(cursor) && !<Token![..]>::peek(cursor)) || <Token![?]>::peek(cursor)
}
