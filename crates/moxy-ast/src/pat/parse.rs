use super::*;

pub(crate) fn single(parser: &Parser) -> Result<Pattern, ParseError> {
    if parser.peek::<PatWild>() {
        return Ok(Pattern::Wild(parser.parse()?));
    }

    if parser.peek::<PatRange>() {
        return Ok(Pattern::Range(parser.parse()?));
    }

    if parser.peek::<PatRest>() {
        return Ok(Pattern::Rest(parser.parse()?));
    }

    if parser.peek::<PatBox>() {
        return Ok(Pattern::Box(parser.parse()?));
    }

    if parser.peek::<PatConst>() {
        return Ok(Pattern::Const(parser.parse()?));
    }

    if parser.peek::<PatReference>() {
        return Ok(Pattern::Reference(parser.parse()?));
    }

    if parser.peek::<PatGroup>() {
        return Ok(Pattern::Group(parser.parse()?));
    }

    if parser.peek::<PatSlice>() {
        return Ok(Pattern::Slice(parser.parse()?));
    }

    if parser.peek::<PatTuple>() {
        return Ok(Pattern::Tuple(parser.parse()?));
    }

    if parser.peek::<PatParen>() {
        return Ok(Pattern::Paren(parser.parse()?));
    }

    if parser.peek::<PatMacro>() {
        return Ok(Pattern::Macro(parser.parse()?));
    }

    if parser.peek::<PatTupleStruct>() {
        return Ok(Pattern::TupleStruct(parser.parse()?));
    }

    if parser.peek::<PatStruct>() {
        return Ok(Pattern::Struct(parser.parse()?));
    }

    if parser.peek::<PatIdent>() {
        return Ok(Pattern::Ident(parser.parse()?));
    }

    if parser.peek::<PatPath>() {
        return Ok(Pattern::Path(parser.parse()?));
    }

    if parser.peek::<PatLit>() {
        return Ok(Pattern::Lit(parser.parse()?));
    }

    parser.error("expected pattern").into()
}
