use super::*;

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
