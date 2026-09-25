use super::*;

pub(crate) fn single(parser: &Parser) -> Result<Pattern, ParseError> {
    if PatWild::peek(parser.cursor()) {
        return Ok(Pattern::Wild(<_ as Parse>::parse(parser)?));
    }

    if PatRange::peek(parser.cursor()) {
        return Ok(Pattern::Range(<_ as Parse>::parse(parser)?));
    }

    if PatRest::peek(parser.cursor()) {
        return Ok(Pattern::Rest(<_ as Parse>::parse(parser)?));
    }

    if PatBox::peek(parser.cursor()) {
        return Ok(Pattern::Box(<_ as Parse>::parse(parser)?));
    }

    if PatConst::peek(parser.cursor()) {
        return Ok(Pattern::Const(<_ as Parse>::parse(parser)?));
    }

    if PatReference::peek(parser.cursor()) {
        return Ok(Pattern::Reference(<_ as Parse>::parse(parser)?));
    }

    if PatGroup::peek(parser.cursor()) {
        return Ok(Pattern::Group(<_ as Parse>::parse(parser)?));
    }

    if PatSlice::peek(parser.cursor()) {
        return Ok(Pattern::Slice(<_ as Parse>::parse(parser)?));
    }

    if PatTuple::peek(parser.cursor()) {
        return Ok(Pattern::Tuple(<_ as Parse>::parse(parser)?));
    }

    if PatParen::peek(parser.cursor()) {
        return Ok(Pattern::Paren(<_ as Parse>::parse(parser)?));
    }

    if PatMacro::peek(parser.cursor()) {
        return Ok(Pattern::Macro(<_ as Parse>::parse(parser)?));
    }

    if PatTupleStruct::peek(parser.cursor()) {
        return Ok(Pattern::TupleStruct(<_ as Parse>::parse(parser)?));
    }

    if PatStruct::peek(parser.cursor()) {
        return Ok(Pattern::Struct(<_ as Parse>::parse(parser)?));
    }

    if PatIdent::peek(parser.cursor()) {
        return Ok(Pattern::Ident(<_ as Parse>::parse(parser)?));
    }

    if PatPath::peek(parser.cursor()) {
        return Ok(Pattern::Path(<_ as Parse>::parse(parser)?));
    }

    if PatLit::peek(parser.cursor()) {
        return Ok(Pattern::Lit(<_ as Parse>::parse(parser)?));
    }

    parser.error("expected pattern").into()
}
