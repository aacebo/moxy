use moxy_token::{Delim, Span, Spanner, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A macro invocation used as a statement (`name!(...);` or `name!(...)`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
    pub semi: Option<Token![;]>,
}

impl Attributed for StmtMacro {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for StmtMacro {
    // As rustc decides, a macro call that a postfix or infix operator follows
    // is an expression statement: `vec![1].len()`, `matches!(a, B) || c`. A
    // braced call ends the statement unless a method call, field access, or
    // `?` follows it.
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(start) = Attributes::skip(cursor) else {
            return false;
        };

        if !MacroCall::peek(start) {
            return false;
        }

        let Some(end) = MacroCall::skip(start) else {
            return true;
        };

        let braced = matches!(end.prev(), Some(TokenTree::Group(group)) if group.delim == Delim::Brace);
        let postfix = (<Token![.]>::peek(end) && !RangeLimits::peek(end)) || <Token![?]>::peek(end);

        if braced {
            return !postfix;
        }

        let call = end.is_delimited(Delim::Paren) || end.is_delimited(Delim::Bracket);
        let infix = BinOp::peek(end) || <Token![=]>::peek(end) || <Token![as]>::peek(end) || RangeLimits::peek(end);
        !postfix && !call && !infix
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let mac: MacroCall = <_ as Parse>::parse(parser)?;
        let semi: Option<Token![;]> = <_ as Parse>::parse(parser)?;

        // A call in parentheses or brackets ends with `;` unless it ends the
        // block, as in rustc
        if semi.is_none() && !mac.body.delim.is_brace() && !parser.is_empty() {
            return parser.error("expected `;`").into();
        }

        Ok(Self { attrs, mac, semi })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = MacroCall::skip(cursor)?;

        if <Token![;]>::peek(cursor) {
            return <Token![;]>::skip(cursor);
        }

        let braced = matches!(cursor.prev(), Some(TokenTree::Group(group)) if group.delim == Delim::Brace);
        (braced || cursor.is_empty()).then_some(cursor)
    }
}

impl Spanner for StmtMacro {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for StmtMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
