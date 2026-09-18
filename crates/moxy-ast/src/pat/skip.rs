use super::*;

pub(crate) fn single(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    if cursor.peek::<PatWild>() {
        return cursor.skip::<PatWild>();
    }

    if cursor.peek::<PatRange>() {
        return cursor.skip::<PatRange>();
    }

    if cursor.peek::<PatRest>() {
        return cursor.skip::<PatRest>();
    }

    if cursor.peek::<PatBox>() {
        return cursor.skip::<PatBox>();
    }

    if cursor.peek::<PatConst>() {
        return cursor.skip::<PatConst>();
    }

    if cursor.peek::<PatReference>() {
        return cursor.skip::<PatReference>();
    }

    if cursor.is_delimited(moxy_token::Delim::None) {
        return cursor.skip::<PatGroup>();
    }

    if cursor.is_delimited(moxy_token::Delim::Bracket) {
        return cursor.skip::<PatSlice>();
    }

    if cursor.is_delimited(moxy_token::Delim::Paren) {
        if cursor.peek::<PatTuple>() {
            return cursor.skip::<PatTuple>();
        }

        return cursor.skip::<PatParen>();
    }

    if cursor.peek::<MacroCall>() {
        return cursor.skip::<MacroCall>();
    }

    if cursor.peek::<PatTupleStruct>() {
        return cursor.skip::<PatTupleStruct>();
    }

    if cursor.peek::<PatStruct>() {
        return cursor.skip::<PatStruct>();
    }

    if cursor.peek::<PatIdent>() {
        return cursor.skip::<PatIdent>();
    }

    if cursor.peek::<PatPath>() {
        return cursor.skip::<PatPath>();
    }

    cursor.skip::<PatLit>()
}
