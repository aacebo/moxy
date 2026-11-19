use super::*;

// Mirrors the dispatch of `parse::single`, peek for peek
pub(crate) fn single(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    let _depth = moxy_token::Depth::enter()?;
    moxy_token::Depth::enter()?;

    if <Token![#]>::peek(cursor) {
        return None;
    }

    // The start tells the form, except of a path, which reads once below
    let after = match super::Form::of_start(cursor) {
        super::Form::Wild => return PatWild::skip(cursor),
        super::Form::RangeWithoutStart => return PatRange::skip(cursor),
        super::Form::Rest => return PatRest::skip(cursor),
        super::Form::Box => return PatBox::skip(cursor),
        super::Form::Reference => return PatReference::skip(cursor),
        super::Form::Group => return PatGroup::skip(cursor),
        super::Form::Slice => return PatSlice::skip(cursor),
        // A parenthesized pattern skips as a one-element tuple, so nested
        // parentheses skip in linear time
        super::Form::ParenOrTuple => return PatTuple::skip(cursor),
        super::Form::Macro => return PatMacro::skip(cursor),
        super::Form::None => return None,
        super::Form::Const => PatConst::skip(cursor)?,
        super::Form::Lit => PatLit::skip(cursor)?,
        super::Form::Ident => {
            let plain = moxy_token::Ident::peek(cursor);
            let after = PatIdent::skip(cursor)?;

            // Only a plain name starts a range
            if !plain || <Token![@]>::peek(cursor.offset(1)) {
                return Some(after);
            }

            after
        }
        // The path reads once and the token after it tells the form
        super::Form::TupleStruct | super::Form::Struct | super::Form::Path => {
            let after = super::Form::path_end(cursor)?;

            if after.is_delimited(moxy_token::Delim::Paren) {
                let mut inner = after.descend(moxy_token::Delim::Paren)?;

                while !inner.is_empty() {
                    inner = Pattern::skip(inner)?;

                    if inner.is_empty() {
                        break;
                    }

                    inner = <Token![,]>::skip(inner)?;
                }

                return Some(after.offset(1));
            }

            if after.is_delimited(moxy_token::Delim::Brace) {
                let inner = PatStructBody::skip(after.descend(moxy_token::Delim::Brace)?)?;
                return inner.is_empty().then(|| after.offset(1));
            }

            after
        }
    };

    if PatRangeLimits::peek(after) {
        return PatRange::skip_rest(after, true);
    }

    Some(after)
}
