use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream, TokenTree};

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
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            mac: <_ as Parse>::parse(parser)?,
            semi: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = MacroCall::skip(cursor)?;
        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for StmtMacro {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.mac.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for StmtMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
