use super::*;

pub(super) fn expr(cursor: Cursor<'_>, _context: ExprContext) -> bool {
    let cursor = Attributes::skip(cursor).unwrap_or(cursor);
    !cursor.is_empty()
}
