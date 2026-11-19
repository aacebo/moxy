use super::*;

// Mirrors the dispatch of `parse::single`, peek for peek
pub(crate) fn single(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    let _depth = moxy_token::Depth::enter()?;

    if PatWild::peek(cursor) {
        PatWild::skip(cursor)
    } else if PatRange::peek(cursor) {
        PatRange::skip(cursor)
    } else if PatRest::peek(cursor) {
        PatRest::skip(cursor)
    } else if PatBox::peek(cursor) {
        PatBox::skip(cursor)
    } else if PatConst::peek(cursor) {
        PatConst::skip(cursor)
    } else if PatReference::peek(cursor) {
        PatReference::skip(cursor)
    } else if PatGroup::peek(cursor) {
        PatGroup::skip(cursor)
    } else if PatSlice::peek(cursor) {
        PatSlice::skip(cursor)
    } else if Attributes::skip(cursor).is_some_and(|cursor| cursor.is_delimited(moxy_token::Delim::Paren)) {
        // A parenthesized pattern skips as a one-element tuple, so nested
        // parentheses skip in linear time
        PatTuple::skip(cursor)
    } else if PatMacro::peek(cursor) {
        PatMacro::skip(cursor)
    } else if PatTupleStruct::peek(cursor) {
        PatTupleStruct::skip(cursor)
    } else if PatStruct::peek(cursor) {
        PatStruct::skip(cursor)
    } else if PatIdent::peek(cursor) {
        PatIdent::skip(cursor)
    } else if PatPath::peek(cursor) {
        PatPath::skip(cursor)
    } else if PatLit::peek(cursor) {
        PatLit::skip(cursor)
    } else {
        None
    }
}
