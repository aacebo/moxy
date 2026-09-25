use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A `use<'a, T>` bound (precise capturing).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseBound {
    pub use_keyword: Token![use],
    pub lt_punct: Token![<],
    pub params: Punctuated<UseBoundParam, Token![,]>,
    pub gt_punct: Token![>],
}

impl Parse for UseBound {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![use]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            use_keyword: parser.parse()?,
            lt_punct: parser.parse()?,
            params: Punctuated::parse_separated_nonempty(parser)?,
            gt_punct: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = <Token![use]>::skip(cursor)?;
        cursor = <Token![<]>::skip(cursor)?;
        cursor = UseBoundParam::skip(cursor)?;

        while <Token![,]>::peek(cursor) {
            cursor = <Token![,]>::skip(cursor)?;
            cursor = UseBoundParam::skip(cursor)?;
        }

        <Token![>]>::skip(cursor)
    }
}

impl Spanner for UseBound {
    fn span(&self) -> Span {
        self.use_keyword.span().join(self.gt_punct.span())
    }
}

impl ToTokens for UseBound {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.use_keyword.to_tokens(t);
        self.lt_punct.to_tokens(t);
        self.params.to_tokens(t);
        self.gt_punct.to_tokens(t);
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UseBoundParam {
    Ident(Ident),
    Lifetime(Lifetime),
}

impl Spanner for UseBoundParam {
    fn span(&self) -> Span {
        match self {
            Self::Ident(v) => v.span(),
            Self::Lifetime(v) => v.span(),
        }
    }
}

impl ToTokens for UseBoundParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(v) => v.to_tokens(tokens),
            Self::Lifetime(v) => v.to_tokens(tokens),
        }
    }
}

impl Parse for UseBoundParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        Lifetime::peek(cursor) || Ident::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if Lifetime::peek(parser.cursor()) {
            Ok(Self::Lifetime(parser.parse()?))
        } else {
            Ok(Self::Ident(parser.parse()?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if Lifetime::peek(cursor) {
            Lifetime::skip(cursor)
        } else {
            Ident::skip(cursor)
        }
    }
}
