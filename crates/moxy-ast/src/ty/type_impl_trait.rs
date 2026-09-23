use crate::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Punctuated, TypeBound};

/// An `impl Trait` type (e.g. `impl Iterator<Item = u8>`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeImplTrait {
    pub impl_keyword: Token![impl],
    pub bounds: Punctuated<TypeBound, Token![+]>,
}

impl Parse for TypeImplTrait {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![impl]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let impl_keyword = parser.parse()?;
        let bounds = crate::TypeBound::parse_bounds(parser)?;
        Ok(Self { impl_keyword, bounds })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = cursor.skip::<Token![impl]>()?;
        cursor = cursor.skip::<TypeBound>()?;

        while cursor.peek::<Token![+]>() {
            cursor = cursor.skip::<Token![+]>()?;
            cursor = cursor.skip::<TypeBound>()?;
        }

        Some(cursor)
    }
}

impl Spanner for TypeImplTrait {
    fn span(&self) -> Span {
        let end = self
            .bounds
            .last()
            .map(|b| b.span())
            .unwrap_or_else(|| self.impl_keyword.span());
        self.impl_keyword.span().join(end)
    }
}

impl ToTokens for TypeImplTrait {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.impl_keyword.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
