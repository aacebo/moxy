use crate::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Punctuated, TypeBound};

/// A trait object type (e.g. `dyn Iterator<Item = u8>`, `dyn Fn() + 'a`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeTraitObject {
    pub dyn_token: Option<Token![dyn]>,
    pub bounds: Punctuated<TypeBound, Token![+]>,
}

impl Parse for TypeTraitObject {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![dyn]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let dyn_token = parser.parse()?;
        let bounds = crate::TypeBound::parse_bounds(parser)?;
        Ok(Self { dyn_token, bounds })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = cursor.skip::<Option<Token![dyn]>>()?;
        cursor = cursor.skip::<TypeBound>()?;

        while cursor.peek::<Token![+]>() {
            cursor = cursor.skip::<Token![+]>()?;
            cursor = cursor.skip::<TypeBound>()?;
        }

        Some(cursor)
    }
}

impl Spanner for TypeTraitObject {
    fn span(&self) -> Span {
        let start = if let Some(d) = &self.dyn_token {
            d.span()
        } else if let Some(b) = self.bounds.first() {
            b.span()
        } else {
            Span::call_site()
        };

        let end = self.bounds.last().map(|b| b.span()).unwrap_or(start);
        start.join(end)
    }
}

impl ToTokens for TypeTraitObject {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.dyn_token.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
