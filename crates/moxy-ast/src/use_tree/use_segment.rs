use moxy_token::{Crate, Ident, SelfValue, Span, Spanner, Super, ToTokens, TokenStream};

use crate::{Cursor, Parse, ParseError, Parser};

/// A segment of an `use` path.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UseSegment {
    Ident(Ident),
    Super(Super),
    Crate(Crate),
    SelfValue(SelfValue),
}

impl UseSegment {
    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(_))
    }

    pub fn is_super(&self) -> bool {
        matches!(self, Self::Super(_))
    }

    pub fn is_crate(&self) -> bool {
        matches!(self, Self::Crate(_))
    }

    pub fn is_self_value(&self) -> bool {
        matches!(self, Self::SelfValue(_))
    }

    pub fn as_ident(&self) -> Option<&Ident> {
        if let Self::Ident(v) = self { Some(v) } else { None }
    }

    pub fn as_super(&self) -> Option<&Super> {
        if let Self::Super(v) = self { Some(v) } else { None }
    }

    pub fn as_crate(&self) -> Option<&Crate> {
        if let Self::Crate(v) = self { Some(v) } else { None }
    }

    pub fn as_self_value(&self) -> Option<&SelfValue> {
        if let Self::SelfValue(v) = self { Some(v) } else { None }
    }
}

impl Spanner for UseSegment {
    fn span(&self) -> Span {
        match self {
            Self::Ident(v) => v.span(),
            Self::Super(v) => v.span(),
            Self::Crate(v) => v.span(),
            Self::SelfValue(v) => v.span(),
        }
    }
}

impl Parse for UseSegment {
    fn peek(cursor: Cursor<'_>) -> bool {
        Ident::peek(cursor) || Super::peek(cursor) || Crate::peek(cursor) || SelfValue::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if Ident::peek(parser.cursor()) {
            Ok(Self::Ident(<_ as Parse>::parse(parser)?))
        } else if Super::peek(parser.cursor()) {
            Ok(Self::Super(<_ as Parse>::parse(parser)?))
        } else if Crate::peek(parser.cursor()) {
            Ok(Self::Crate(<_ as Parse>::parse(parser)?))
        } else if SelfValue::peek(parser.cursor()) {
            Ok(Self::SelfValue(<_ as Parse>::parse(parser)?))
        } else {
            Err(parser.error("expected identifier, 'super', 'crate' or 'self'"))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Ident::skip(cursor)
            .or_else(|| Super::skip(cursor))
            .or_else(|| SelfValue::skip(cursor))
            .or_else(|| Crate::skip(cursor))
    }
}

impl ToTokens for UseSegment {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Ident(v) => v.to_tokens(t),
            Self::Super(v) => v.to_tokens(t),
            Self::SelfValue(v) => v.to_tokens(t),
            Self::Crate(v) => v.to_tokens(t),
        }
    }
}
