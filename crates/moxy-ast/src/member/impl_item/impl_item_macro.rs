use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A macro invocation inside an `impl` block.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
    pub semi: Option<Token![;]>,
}

impl Attributed for ImplItemMacro {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ImplItemMacro {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| MacroCall::peek(cursor))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let mac: MacroCall = <_ as Parse>::parse(parser)?;
        let semi: Option<Token![;]> = <_ as Parse>::parse(parser)?;

        // Only a brace-delimited macro item ends without a semicolon
        if semi.is_none() && !mac.body.delim.is_brace() {
            return parser.expected("`;` after a macro item in parentheses or brackets").into();
        }

        Ok(Self { attrs, mac, semi })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = MacroCall::skip(cursor)?;
        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for ImplItemMacro {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ImplItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
