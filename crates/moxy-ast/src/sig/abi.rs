use moxy_token::{Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// An ABI string (`extern "C"`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Abi {
    pub extern_keyword: Token![extern],
    pub name: Option<String>,
}

impl Parse for Abi {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![extern]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let extern_keyword = parser.parse()?;
        let name = match parser.curr() {
            Some(TokenTree::Literal(lit)) if lit.repr().starts_with('"') => {
                let repr = lit.repr().to_string();
                parser.advance();
                Some(repr.trim_matches('"').to_string())
            }

            _ => None,
        };

        Ok(Self { extern_keyword, name })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Token![extern]>()?;

        if matches!(cursor.curr(), Some(TokenTree::Literal(lit)) if lit.repr().starts_with('"')) {
            cursor = cursor.offset(1);
        }

        Some(cursor)
    }
}

impl Spanner for Abi {
    fn span(&self) -> Span {
        self.extern_keyword.span()
    }
}

impl ToTokens for Abi {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.extern_keyword.to_tokens(t);

        if let Some(name) = &self.name {
            moxy_token::Lit::string(name).to_tokens(t);
        }
    }
}
