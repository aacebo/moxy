use moxy_token::{Lit, LitStr, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// An ABI string (`extern "C"`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Abi {
    pub extern_keyword: Token![extern],
    /// The ABI string as written, cooked or raw, as `"C"` in `extern "C"`;
    /// [`LitStr::value`] gives its value. It is boxed to keep [`Type`] small.
    pub name: Option<Box<LitStr>>,
}

impl Parse for Abi {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![extern]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let extern_keyword = <_ as Parse>::parse(parser)?;
        let name = match parser.curr() {
            Some(TokenTree::Literal(Lit::Str(lit))) => {
                let lit = Box::new(lit.clone());
                parser.advance();
                Some(lit)
            }

            _ => None,
        };

        Ok(Self { extern_keyword, name })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = <Token![extern]>::skip(cursor)?;

        if matches!(cursor.curr(), Some(TokenTree::Literal(Lit::Str(_)))) {
            cursor = cursor.offset(1);
        }

        Some(cursor)
    }
}

impl Spanner for Abi {
    fn span(&self) -> Span {
        let start = self.extern_keyword.span();
        self.name.as_ref().map_or(start, |name| start.join(name.span()))
    }
}

impl ToTokens for Abi {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.extern_keyword.to_tokens(t);

        self.name.to_tokens(t);
    }
}
