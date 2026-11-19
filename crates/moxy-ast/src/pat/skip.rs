use super::*;

pub(crate) fn single(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    let _depth = moxy_token::Depth::enter()?;

    if PatWild::peek(cursor) {
        return PatWild::skip(cursor);
    }

    if PatRange::peek(cursor) {
        return PatRange::skip(cursor);
    }

    if PatRest::peek(cursor) {
        return PatRest::skip(cursor);
    }

    if PatBox::peek(cursor) {
        return PatBox::skip(cursor);
    }

    if PatConst::peek(cursor) {
        return PatConst::skip(cursor);
    }

    if PatReference::peek(cursor) {
        return PatReference::skip(cursor);
    }

    if cursor.is_delimited(moxy_token::Delim::None) {
        return PatGroup::skip(cursor);
    }

    if cursor.is_delimited(moxy_token::Delim::Bracket) {
        return PatSlice::skip(cursor);
    }

    // A parenthesized pattern skips as a one-element tuple, so nested
    // parentheses skip in linear time
    if Attributes::skip(cursor).is_some_and(|cursor| cursor.is_delimited(moxy_token::Delim::Paren)) {
        return PatTuple::skip(cursor);
    }

    if MacroCall::peek(cursor) {
        return MacroCall::skip(cursor);
    }

    if PatTupleStruct::peek(cursor) {
        return PatTupleStruct::skip(cursor);
    }

    if PatStruct::peek(cursor) {
        return PatStruct::skip(cursor);
    }

    if PatIdent::peek(cursor) {
        return PatIdent::skip(cursor);
    }

    if PatPath::peek(cursor) {
        return PatPath::skip(cursor);
    }

    PatLit::skip(cursor)
}
