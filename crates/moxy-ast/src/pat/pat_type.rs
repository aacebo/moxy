use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A type-ascription pattern, e.g. `x: i32`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatType {
    pub attrs: Attributes,
    pub pat: Box<Pattern>,
    pub colon: Token![:],
    pub ty: Box<Type>,
}

impl Spanner for PatType {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl Parse for PatType {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Pattern>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            pat: parser.parse()?,
            colon: parser.parse()?,
            ty: parser.parse()?,
        })
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
