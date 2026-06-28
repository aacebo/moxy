use moxy_token::parser::{Parse, ParseError, ParseStream};
use moxy_token::{Comma, Delim, Eq, EqEq, FatArrow, Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Lit, Path, Punctuated};

/// A structured attribute meta item (`name`, `name(...)`, `name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Meta {
    pub path: Path,
    pub value: MetaValue,
}

impl std::ops::Deref for Meta {
    type Target = MetaValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for Meta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Parse for Meta {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            path: stream.parse()?,
            value: stream.parse()?,
        })
    }
}

impl Spanner for Meta {
    fn span(&self) -> Span {
        match self.value.span() {
            None => self.path.span(),
            Some(span) => self.path.span().join(span),
        }
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.value.to_tokens(t);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum MetaArgument {
    Meta(Meta),
    Value(MetaValue),
}

impl MetaArgument {
    pub fn is_meta(&self) -> bool {
        matches!(self, Self::Meta(_))
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_meta(&self) -> Option<&Meta> {
        match self {
            Self::Meta(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_value(&self) -> Option<&MetaValue> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
        }
    }
}

impl Parse for MetaArgument {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Path>() {
            Ok(Self::Meta(stream.parse()?))
        } else {
            Ok(Self::Value(stream.parse()?))
        }
    }
}

impl ToTokens for MetaArgument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Meta(v) => v.to_tokens(tokens),
            Self::Value(v) => v.to_tokens(tokens),
        }
    }
}

/// A structured attribute meta item (`name`, `name(...)`, `name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum MetaValue {
    /// `#[debug]`
    None,
    /// `#[debug(true)]`
    /// `#[debug = true]`
    Literal(Lit),
    /// `#[debug = true]`
    Alias { eq: Eq, value: Box<Self> },
    /// `#[debug(true, env = "test")]`
    List {
        items: Delimited<Punctuated<MetaArgument, Comma>>,
    },
    /// raw tokens for custom syntax parsing
    Verbatim(TokenStream),
}

impl MetaValue {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List { items: _ })
    }

    pub fn is_alias(&self) -> bool {
        matches!(self, Self::Alias { eq: _, value: _ })
    }

    pub fn is_verbatim(&self) -> bool {
        matches!(self, Self::Verbatim(_))
    }

    pub fn as_alias(&self) -> Option<&Self> {
        match self {
            Self::Alias { eq: _, value } => Some(value),
            _ => None,
        }
    }

    pub fn as_literal(&self) -> Option<&Lit> {
        match self {
            Self::Literal(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Delimited<Punctuated<MetaArgument, Comma>>> {
        match self {
            Self::List { items } => Some(items),
            _ => None,
        }
    }

    pub fn as_verbatim(&self) -> Option<&TokenStream> {
        match self {
            Self::Verbatim(tokens) => Some(tokens),
            _ => None,
        }
    }

    pub fn span(&self) -> Option<Span> {
        match self {
            Self::None => None,
            Self::Literal(lit) => Some(lit.span()),
            Self::Alias { eq: _, value } => value.span(),
            Self::List { items } => Some(items.span()),
            Self::Verbatim(tokens) => Some(tokens.span()),
        }
    }
}

impl Parse for MetaValue {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Eq>() && !stream.peek::<EqEq>() && !stream.peek::<FatArrow>() {
            return Ok(Self::Alias {
                eq: stream.parse()?,
                value: stream.parse()?,
            });
        }

        if let Ok(lit) = stream.parse::<Lit>() {
            return Ok(Self::Literal(lit));
        }

        if let Ok((span, tokens)) = stream.parse_group_spanned(Delim::Paren) {
            let punct = Punctuated::parse_terminated(&mut tokens.parse())?;
            let items = Delimited::new(Delim::Paren, span, punct);
            return Ok(Self::List { items });
        }

        if let Ok(tokens) = stream.parse_group(Delim::Brace) {
            return Ok(Self::Verbatim(tokens));
        }

        Ok(Self::None)
    }
}

impl ToTokens for MetaValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Alias { eq, value } => {
                eq.to_tokens(t);
                value.to_tokens(t);
            }
            Self::Literal(lit) => {
                lit.to_tokens(t);
            }
            Self::List { items } => {
                items.to_tokens(t);
            }
            Self::Verbatim(tokens) => {
                tokens.to_tokens(t);
            }
            _ => {}
        }
    }
}
