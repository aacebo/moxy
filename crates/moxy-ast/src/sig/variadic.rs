use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A C-style variadic marker (`...`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variadic {
    pub attrs: Attributes,
    /// The pattern before `: ...`, as `args` in `mut args: ...` or `_`.
    pub pat: Option<Box<Pattern>>,
    pub colon: Option<Token![:]>,
    pub dots: Token![...],
    /// A trailing comma after the dots, as in `fn f(a: u8, ...,);`.
    pub comma: Option<Token![,]>,
}

impl Attributed for Variadic {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for Variadic {
    // `...` alone or after a pattern and `:`, which ends the parameter. The
    // parameter's tokens are scanned at its own level rather than skipped as
    // a pattern, so peeking a parameter does not read a nested one again
    fn peek(cursor: Cursor<'_>) -> bool {
        let start = Attributes::skip(cursor).unwrap_or(cursor);
        let ends = |cursor: Cursor<'_>| cursor.is_empty() || <Token![,]>::peek(cursor);

        if <Token![...]>::skip(start).is_some_and(ends) {
            return true;
        }

        // A `,` inside generic arguments, as in `S::<A, B>(x)`, which are no
        // group, does not end the parameter
        let mut cursor = start;
        let mut angles = 0usize;

        while !cursor.is_empty() && !(angles == 0 && <Token![,]>::peek(cursor)) {
            if angles == 0
                && <Token![:]>::peek(cursor)
                && cursor.remaining() < start.remaining()
                && <Token![...]>::skip(cursor.offset(1)).is_some_and(ends)
            {
                return true;
            }

            angles = if <Token![<<]>::peek(cursor) {
                angles + 2
            } else if <Token![<]>::peek(cursor) {
                angles + 1
            } else if <Token![>>]>::peek(cursor) {
                angles.saturating_sub(2)
            } else if <Token![>]>::peek(cursor) && !<Token![->]>::peek(cursor) {
                angles.saturating_sub(1)
            } else {
                angles
            };
            cursor = cursor.offset(1);
        }

        false
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let (pat, colon) = if <Token![...]>::peek(parser.cursor()) {
            (None, None)
        } else {
            (
                Some(Box::new(<_ as Parse>::parse(parser)?)),
                Some(<_ as Parse>::parse(parser)?),
            )
        };

        Ok(Self {
            attrs,
            pat,
            colon,
            dots: <_ as Parse>::parse(parser)?,
            comma: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        if !<Token![...]>::peek(cursor) {
            cursor = Pattern::skip(cursor)?;
            cursor = <Token![:]>::skip(cursor)?;
        }

        cursor = <Token![...]>::skip(cursor)?;
        Option::<Token![,]>::skip(cursor)
    }
}

impl Spanner for Variadic {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for Variadic {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.pat.to_tokens(t);
        self.colon.to_tokens(t);
        self.dots.to_tokens(t);
        self.comma.to_tokens(t);
    }
}
