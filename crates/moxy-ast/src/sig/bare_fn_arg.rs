use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An argument of a bare function pointer type.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BareFnArg {
    pub attrs: Attributes,
    pub name: Option<(Ident, Token![:])>,
    pub ty: Type,
}

impl Parse for BareFnArg {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        (Ident::peek(cursor) && <Token![:]>::peek(cursor.offset(1))) || Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let name = if Ident::peek(parser.cursor()) && <Token![:]>::peek(parser.cursor().offset(1)) {
            Some((<_ as Parse>::parse(parser)?, <_ as Parse>::parse(parser)?))
        } else {
            None
        };

        let ty = <_ as Parse>::parse(parser)?;

        Ok(Self { attrs, name, ty })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;

        if Ident::peek(cursor) && <Token![:]>::peek(cursor.offset(1)) {
            cursor = Ident::skip(cursor)?;
            cursor = <Token![:]>::skip(cursor)?;
        }

        Type::skip(cursor)
    }
}

impl Spanner for BareFnArg {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for BareFnArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if let Some((n, colon)) = &self.name {
            n.to_tokens(t);
            colon.to_tokens(t);
        }

        self.ty.to_tokens(t);
    }
}
