use moxy_token::Token;

use super::*;

/// The shape of a meta item after its path (`name`, `name = v`, `name(..)`, `name { .. }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum MetaLayout {
    /// `#[debug]`
    None,
    /// `#[custom { a + b }]` — a verbatim leaf attached directly to the path
    Value(MetaValue),
    /// `#[debug = true]`
    Alias { eq: Token![=], value: MetaValue },
    /// `#[debug(true, env = "test")]`
    List {
        items: Delimited<Punctuated<MetaArgument, Token![,]>>,
    },
}

impl MetaLayout {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_alias(&self) -> bool {
        matches!(self, Self::Alias { eq: _, value: _ })
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List { items: _ })
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_value(&self) -> Option<&MetaValue> {
        match self {
            Self::Alias { eq: _, value } => Some(value),
            Self::Value(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Delimited<Punctuated<MetaArgument, Token![,]>>> {
        match self {
            Self::List { items } => Some(items),
            _ => None,
        }
    }

    pub fn span(&self) -> Option<Span> {
        match self {
            Self::None => None,
            Self::Alias { eq: _, value } => Some(value.span()),
            Self::List { items } => Some(items.span()),
            Self::Value(value) => Some(value.span()),
        }
    }
}

impl Parse for MetaLayout {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Token![=]>() && !stream.peek::<Token![==]>() && !stream.peek::<Token![=>]>() {
            return Ok(Self::Alias {
                eq: stream.parse()?,
                value: stream.parse()?,
            });
        }

        if let Ok((span, tokens)) = stream.parse_group_spanned(Delim::Paren) {
            let punct = Punctuated::parse_terminated(&mut tokens.parse())?;
            let items = Delimited::new(Delim::Paren, span, punct);
            return Ok(Self::List { items });
        }

        if matches!(stream.curr().and_then(|tt| tt.delim()), Some(d) if d.is_brace()) {
            return Ok(Self::Value(stream.parse()?));
        }

        Ok(Self::None)
    }
}

impl ToTokens for MetaLayout {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Alias { eq, value } => {
                eq.to_tokens(t);
                value.to_tokens(t);
            }
            Self::List { items } => items.to_tokens(t),
            Self::Value(value) => value.to_tokens(t),
            Self::None => {}
        }
    }
}
