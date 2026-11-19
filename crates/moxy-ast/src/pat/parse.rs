use super::*;

pub(crate) fn single(parser: &Parser) -> Result<Pattern, ParseError> {
    let _depth = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;

    // The peeks below skip one level deeper, so without that room they would
    // misread the input, as `1..=2` for `1` at the limit
    moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;

    // As in rustc, a pattern takes no attributes; a parameter, match arm, or
    // struct pattern field holds them
    if <Token![#]>::peek(parser.cursor()) {
        return parser.error("expected pattern").into();
    }

    // One call site for every form keeps one return slot in this frame
    let parse: fn(&Parser) -> Result<Pattern, ParseError> = match super::Form::of(parser.cursor()) {
        super::Form::Wild => parse_into::<PatWild, Pattern>,
        super::Form::RangeWithoutStart => parse_into::<PatRange, Pattern>,
        super::Form::Rest => parse_into::<PatRest, Pattern>,
        super::Form::Box => parse_into::<PatBox, Pattern>,
        super::Form::Const => bound::<PatConst>,
        super::Form::Reference => parse_into::<PatReference, Pattern>,
        super::Form::Group => parse_into::<PatGroup, Pattern>,
        super::Form::Slice => parse_into::<PatSlice, Pattern>,
        super::Form::ParenOrTuple => paren_or_tuple,
        super::Form::Ident => bound::<PatIdent>,
        super::Form::Macro => parse_into::<PatMacro, Pattern>,
        super::Form::TupleStruct => parse_into::<PatTupleStruct, Pattern>,
        super::Form::Struct => parse_into::<PatStruct, Pattern>,
        super::Form::Path => bound::<PatPath>,
        super::Form::Lit => bound::<PatLit>,
        super::Form::None => return parser.error("expected pattern").into(),
    };

    parse(parser)
}

// A form that can start a range parses once, and the limits after it make it
// the range's start, so no peek reads the bound ahead of the parse
fn bound<T: Parse + Into<Pattern>>(parser: &Parser) -> Result<Pattern, ParseError> {
    let pattern: Pattern = T::parse(parser)?.into();

    if !PatRangeLimits::peek(parser.cursor()) {
        return Ok(pattern);
    }

    let start = match pattern {
        Pattern::Lit(lit) => {
            let value = Expr::Lit(ExprLit {
                attrs: Attributes::default(),
                lit: lit.lit,
            });

            match lit.minus {
                Some(minus) => Expr::Unary(ExprUnary {
                    attrs: lit.attrs,
                    op: UnOp::Neg(minus),
                    expr: Box::new(value),
                }),
                None => value,
            }
        }
        Pattern::Path(path) => Expr::Path(ExprPath {
            attrs: path.attrs,
            qself: path.qself,
            path: path.path,
        }),
        Pattern::Ident(ident) if ident.by_ref.is_none() && ident.mutability.is_none() && ident.subpat.is_none() => {
            Expr::Path(ExprPath {
                attrs: ident.attrs,
                qself: None,
                path: ident.ident.into(),
            })
        }
        Pattern::Const(constant) => Expr::Const(ExprConst {
            attrs: constant.attrs,
            const_keyword: constant.keyword,
            block: constant.block,
        }),
        _ => return parser.error("expected a range bound").into(),
    };

    Ok(Pattern::Range(Box::new(PatRange::parse_rest(
        parser,
        Attributes::default(),
        Some(start),
    )?)))
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
