use moxy_token::span::DelimSpan;

use super::*;

/// A leaf value in a meta tree (the terminal after `=`, after `{...}`, or as a list item).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum MetaValue {
    /// `42`, `"x"`, `true`
    Literal(Lit),
    /// `{ a + b }` — raw tokens for custom syntax
    Verbatim(Delimited<TokenStream>),
}

impl MetaValue {
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_))
    }

    pub fn is_verbatim(&self) -> bool {
        matches!(self, Self::Verbatim(_))
    }

    pub fn as_literal(&self) -> Option<&Lit> {
        match self {
            Self::Literal(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn as_verbatim(&self) -> Option<&TokenStream> {
        match self {
            Self::Verbatim(tokens) => Some(tokens),
            _ => None,
        }
    }
}

impl Parse for MetaValue {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if let Ok(lit) = stream.parse::<Lit>() {
            return Ok(Self::Literal(lit));
        }

        if let Ok((span, tokens)) = stream.parse_group_spanned(Delim::Brace) {
            return Ok(Self::Verbatim(Delimited::new(Delim::Brace, span, tokens)));
        }

        if let Ok((span, tokens)) = stream.parse_group_spanned(Delim::None) {
            return Ok(Self::Verbatim(Delimited::new(Delim::None, span, tokens)));
        }

        let span = stream.span();
        let tokens = stream.advance_by(stream.remaining()).ok_or(ParseError::new(span, "EOF"))?;

        Ok(Self::Verbatim(Delimited::new(
            Delim::None,
            DelimSpan::new(span, span),
            tokens.into(),
        )))
    }
}

impl ToTokens for MetaValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Literal(lit) => lit.to_tokens(t),
            Self::Verbatim(tokens) => tokens.to_tokens(t),
        }
    }
}

impl Spanner for MetaValue {
    fn span(&self) -> Span {
        match self {
            Self::Literal(lit) => lit.span(),
            Self::Verbatim(tokens) => tokens.span(),
        }
    }
}
