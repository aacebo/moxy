use super::*;

pub(crate) fn single(parser: &Parser) -> Result<Pattern, ParseError> {
    let _depth = moxy_token::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;

    // One call site for every form keeps one return slot in this frame
    let parse: fn(&Parser) -> Result<Pattern, ParseError> = if PatWild::peek(parser.cursor()) {
        parse_into::<PatWild, Pattern>
    } else if PatRange::peek(parser.cursor()) {
        parse_into::<PatRange, Pattern>
    } else if PatRest::peek(parser.cursor()) {
        parse_into::<PatRest, Pattern>
    } else if PatBox::peek(parser.cursor()) {
        parse_into::<PatBox, Pattern>
    } else if PatConst::peek(parser.cursor()) {
        parse_into::<PatConst, Pattern>
    } else if PatReference::peek(parser.cursor()) {
        parse_into::<PatReference, Pattern>
    } else if PatGroup::peek(parser.cursor()) {
        parse_into::<PatGroup, Pattern>
    } else if PatSlice::peek(parser.cursor()) {
        parse_into::<PatSlice, Pattern>
    } else if Attributes::skip(parser.cursor()).is_some_and(|cursor| cursor.is_delimited(moxy_token::Delim::Paren)) {
        paren_or_tuple
    } else if PatMacro::peek(parser.cursor()) {
        parse_into::<PatMacro, Pattern>
    } else if PatTupleStruct::peek(parser.cursor()) {
        parse_into::<PatTupleStruct, Pattern>
    } else if PatStruct::peek(parser.cursor()) {
        parse_into::<PatStruct, Pattern>
    } else if PatIdent::peek(parser.cursor()) {
        parse_into::<PatIdent, Pattern>
    } else if PatPath::peek(parser.cursor()) {
        parse_into::<PatPath, Pattern>
    } else if PatLit::peek(parser.cursor()) {
        parse_into::<PatLit, Pattern>
    } else {
        return parser.error("expected pattern").into();
    };

    parse(parser)
}

// Parses the parenthesized patterns once and then tells a tuple from a
// parenthesized pattern, so nested parentheses parse in linear time. One
// pattern without a trailing comma, other than `..`, is parenthesized.
fn paren_or_tuple(parser: &Parser) -> Result<Pattern, ParseError> {
    let attrs = <_ as Parse>::parse(parser)?;
    let (span, inner) = parser.parse_group_spanned(moxy_token::Delim::Paren)?;
    let mut elems: List<Pattern, Token![,]> = List::parse_all(&inner)?;

    if elems.len() == 1 && !elems.is_trailing() && !elems.first().is_some_and(Pattern::is_rest) {
        let content = elems.pop().map(|pair| pair.into_value()).expect("one pattern");

        return Ok(Pattern::Paren(PatParen {
            attrs,
            content: Delimited::paren(span, Box::new(content)),
        }));
    }

    Ok(Pattern::Tuple(PatTuple {
        attrs,
        elems: Delimited::paren(span, elems),
    }))
}
