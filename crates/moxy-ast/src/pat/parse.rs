use super::*;

pub(crate) fn single(parser: &Parser) -> Result<Pattern, ParseError> {
    if PatWild::peek(parser.cursor()) {
        return Ok(Pattern::Wild(parser.parse()?));
    }

    if PatRange::peek(parser.cursor()) {
        return Ok(Pattern::Range(parser.parse()?));
    }

    if PatRest::peek(parser.cursor()) {
        return Ok(Pattern::Rest(parser.parse()?));
    }

    if PatBox::peek(parser.cursor()) {
        return Ok(Pattern::Box(parser.parse()?));
    }

    if PatConst::peek(parser.cursor()) {
        return Ok(Pattern::Const(parser.parse()?));
    }

    if PatReference::peek(parser.cursor()) {
        return Ok(Pattern::Reference(parser.parse()?));
    }

    if PatGroup::peek(parser.cursor()) {
        return Ok(Pattern::Group(parser.parse()?));
    }

    if PatSlice::peek(parser.cursor()) {
        return Ok(Pattern::Slice(parser.parse()?));
    }

    if PatTuple::peek(parser.cursor()) {
        return Ok(Pattern::Tuple(parser.parse()?));
    }

    if PatParen::peek(parser.cursor()) {
        return Ok(Pattern::Paren(parser.parse()?));
    }

    if PatMacro::peek(parser.cursor()) {
        return Ok(Pattern::Macro(parser.parse()?));
    }

    if PatTupleStruct::peek(parser.cursor()) {
        return Ok(Pattern::TupleStruct(parser.parse()?));
    }

    if PatStruct::peek(parser.cursor()) {
        return Ok(Pattern::Struct(parser.parse()?));
    }

    if PatIdent::peek(parser.cursor()) {
        return Ok(Pattern::Ident(parser.parse()?));
    }

    if PatPath::peek(parser.cursor()) {
        return Ok(Pattern::Path(parser.parse()?));
    }

    if PatLit::peek(parser.cursor()) {
        return Ok(Pattern::Lit(parser.parse()?));
    }

    parser.error("expected pattern").into()
}
