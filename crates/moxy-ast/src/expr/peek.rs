use moxy_token::{Keyword, Punct, Spacing, TokenTree};

use super::*;

// Whether an expression can start here. An optional operand, as of `return`
// or `break`, is absent otherwise: `match a { _ => return, b => b }`, `return.x`
pub(super) fn expr(cursor: Cursor<'_>, _context: ExprContext) -> bool {
    let cursor = Attributes::skip(cursor).unwrap_or(cursor);

    match cursor.curr() {
        None => false,
        Some(TokenTree::Literal(_) | TokenTree::Ident(_) | TokenTree::Group(_)) => true,
        Some(TokenTree::Keyword(keyword)) => matches!(
            keyword,
            Keyword::If(_)
                | Keyword::Match(_)
                | Keyword::While(_)
                | Keyword::For(_)
                | Keyword::Loop(_)
                | Keyword::Unsafe(_)
                | Keyword::Const(_)
                | Keyword::Async(_)
                | Keyword::Move(_)
                | Keyword::Static(_)
                | Keyword::Return(_)
                | Keyword::Break(_)
                | Keyword::Continue(_)
                | Keyword::Yield(_)
                | Keyword::Let(_)
                | Keyword::Try(_)
                | Keyword::Crate(_)
                | Keyword::SelfValue(_)
                | Keyword::SelfType(_)
                | Keyword::Super(_)
                // Weak keywords, which name variables and paths too
                | Keyword::Auto(_)
                | Keyword::Default(_)
                | Keyword::MacroRules(_)
                | Keyword::Raw(_)
                | Keyword::Safe(_)
                | Keyword::Union(_)
        ),
        Some(TokenTree::Punct(punct)) => match punct {
            Punct::Minus(_) | Punct::Not(_) | Punct::Star(_) | Punct::And(_) | Punct::Or(_) | Punct::Lt(_) => true,
            Punct::Underscore(_) => true,
            Punct::Dot(_) => RangeLimits::peek(cursor),
            Punct::Colon(_) => <Token![::]>::peek(cursor),
            // A label, as in `'a: loop {}`
            Punct::Quote(_) => Label::skip(cursor).is_some(),
            _ => false,
        },
    }
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

// Whether a range without a start begins here, after the limits of another
// range and apart from them: `a.. ..b`, while `a....b` lexes in rustc as `...`
pub(super) fn nested_range(cursor: Cursor<'_>) -> bool {
    let joined = matches!(cursor.prev(), Some(TokenTree::Punct(punct)) if punct.spacing() == Spacing::Joint);
    RangeLimits::peek(cursor) && !joined
}
