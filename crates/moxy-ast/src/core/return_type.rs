use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// The optional return type of a function (`-> Type` or nothing).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ReturnType {
    Default,
    Type(Token![->], Box<Type>),
}

impl ReturnType {
    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(..))
    }

    pub fn as_type(&self) -> Option<&Type> {
        if let Self::Type(_, v) = self { Some(v.as_ref()) } else { None }
    }
}

impl ReturnType {
    /// Parses a return type that `+` does not continue, as in `Fn(A) -> B + Send`
    /// and `fn() -> B + Send`, where the bound applies to the whole type.
    pub(crate) fn parse_without_plus(parser: &Parser) -> Result<Self, ParseError> {
        if <Token![->]>::peek(parser.cursor()) {
            let arrow = <_ as Parse>::parse(parser)?;
            Ok(Self::Type(arrow, Box::new(Type::parse_without_plus(parser)?)))
        } else {
            Ok(Self::Default)
        }
    }

    pub(crate) fn skip_without_plus(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![->]>::peek(cursor) {
            Type::skip_without_plus(<Token![->]>::skip(cursor)?)
        } else {
            Some(cursor)
        }
    }

    pub(crate) fn to_tokens_without_plus(&self, tokens: &mut TokenStream) {
        if let Self::Type(arrow, ty) = self {
            arrow.to_tokens(tokens);
            ty.to_tokens_without_plus(tokens);
        }
    }
}

impl Parse for ReturnType {
    fn peek(_cursor: Cursor<'_>) -> bool {
        true
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if <Token![->]>::peek(parser.cursor()) {
            let arrow = <_ as Parse>::parse(parser)?;
            Ok(Self::Type(arrow, <_ as Parse>::parse(parser)?))
        } else {
            Ok(Self::Default)
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![->]>::peek(cursor) {
            Type::skip(<Token![->]>::skip(cursor)?)
        } else {
            Some(cursor)
        }
    }
}

impl Spanner for ReturnType {
    fn span(&self) -> Span {
        match self {
            Self::Default => Span::call_site(),
            Self::Type(arrow, ty) => arrow.span().join(ty.span()),
        }
    }
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let Self::Type(arrow, ty) = self {
            arrow.to_tokens(t);
            ty.to_tokens(t);
        }
    }
}
