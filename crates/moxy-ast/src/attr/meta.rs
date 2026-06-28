use moxy_token::parser::{Parse, ParseError, ParseStream};
use moxy_token::{Comma, Delim, Eq, EqEq, FatArrow, Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Expr, Lit, Path, Punctuated};

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
        match &self.value {
            MetaValue::None => self.path.span(),
            MetaValue::Literal(lit) => self.path.span().join(lit.span()),
            MetaValue::Alias { eq: _, expr } => self.path.span().join(expr.span()),
            MetaValue::List { items } => self.path.span().join(items.span()),
            MetaValue::Verbatim(tokens) => self.path.span().join(tokens.span()),
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
    Literal(Lit),
    /// `#[debug = true]`
    Alias { eq: Eq, expr: Expr },
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
        matches!(self, Self::Alias { eq: _, expr: _ })
    }

    pub fn is_verbatim(&self) -> bool {
        matches!(self, Self::Verbatim(_))
    }

    pub fn as_alias(&self) -> Option<&Expr> {
        match self {
            Self::Alias { eq: _, expr } => Some(expr),
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
}

impl Parse for MetaValue {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Eq>() && !stream.peek::<EqEq>() && !stream.peek::<FatArrow>() {
            return Ok(Self::Alias {
                eq: stream.parse()?,
                expr: stream.parse()?,
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
            Self::Alias { eq, expr } => {
                eq.to_tokens(t);
                expr.to_tokens(t);
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::{ToTokenStream, TokenStream};

    use super::super::*;
    use super::*;

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn outer_empty() {
        let a = moxy_token::parse!("#[inline]" as Attribute).unwrap();
        assert!(a.style.is_outer());
        assert!(a.meta.is_none());
        assert_eq!(render(&a), "# [inline]");
    }

    #[test]
    fn outer_delimited() {
        let a = moxy_token::parse!("#[derive(Clone, Debug)]" as Attribute).unwrap();
        assert!(a.style.is_outer());
        assert!(a.meta.is_list());
        assert_eq!(render(&a), "# [derive (Clone , Debug)]");
    }

    #[test]
    fn inner() {
        let a = moxy_token::parse!("#![no_std]" as Attribute).unwrap();
        assert!(a.style.is_inner());
        assert_eq!(render(&a), "# ! [no_std]");
    }

    #[test]
    fn many() {
        let attrs: Vec<Attribute> = {
            let ts = TokenStream::from_str("#[a] #[b(1)]").unwrap();
            let mut ps = ts.parse();
            let mut out = Vec::new();
            while !ps.is_empty() {
                out.push(ps.parse::<Attribute>().unwrap());
            }
            out
        };

        assert_eq!(attrs.len(), 2);
    }

    #[test]
    fn name_value() {
        let a = moxy_token::parse!("#[path = \"x.rs\"]" as Attribute).unwrap();
        assert!(a.meta.is_alias());
        assert_eq!(render(&a), "# [path = \"x.rs\"]");
    }

    #[test]
    fn meta_forms() {
        assert!(moxy_token::parse!("inline" as Meta).unwrap().is_none());
        assert!(moxy_token::parse!("derive(Clone)" as Meta).unwrap().is_list());
        assert!(moxy_token::parse!("path = \"x\"" as Meta).unwrap().is_alias());

        let meta = moxy_token::parse!("debug(\"x\")" as Meta).unwrap();

        if let MetaValue::List { items } = &meta.value {
            if let Some(MetaArgument::Value(MetaValue::Literal(lit))) = &items.first() {
                assert_eq!(lit.as_str().map(|s| s.repr.as_str()), Some("\"x\""))
            } else {
                panic!("expected lit");
            }
        } else {
            panic!("expected list");
        }
    }
}
