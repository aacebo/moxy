use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A type-ascription pattern, e.g. `x: i32`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatType {
    pub attrs: Attributes,
    pub pat: Box<Pattern>,
    pub colon: Token![:],
    pub ty: Box<Type>,
}

impl Attributed for PatType {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Spanner for PatType {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl Parse for PatType {
    // A pattern and a `:` start a typed pattern; the type is left to the
    // parse, so a type nested past the depth limit reports the depth error
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .and_then(super::skip::single)
            .is_some_and(|cursor| <Token![:]>::peek(cursor))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            // As in rustc, a parameter's pattern has no top-level `|`
            pat: Box::new(super::parse::single(parser)?),
            colon: <_ as Parse>::parse(parser)?,
            ty: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = super::skip::single(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        Type::skip(cursor)
    }
}

impl ToTokens for PatType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.pat.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
    }
}
