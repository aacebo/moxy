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
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        <Token![...]>::peek(cursor)
            || Pattern::skip(cursor)
                .and_then(|cursor| <Token![:]>::skip(cursor))
                .is_some_and(|cursor| <Token![...]>::peek(cursor))
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
