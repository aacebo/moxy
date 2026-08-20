pub mod meta;
pub mod query;
mod style;

pub use meta::Meta;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};
pub use style::*;

use crate::Delimited;

/// A Rust attribute (`#[...]` or `#![...]`) applied to an item, expression, or statement.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Attribute {
    pub style: AttrStyle,
    pub meta: Delimited<Meta>,
}

impl Spanner for Attribute {
    fn span(&self) -> Span {
        self.style.span().join(self.meta.span())
    }
}

impl Parse for Attribute {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            style: stream.parse()?,
            meta: Delimited::parse_bracket(stream)?,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.style.to_tokens(tokens);
        self.meta.to_tokens(tokens);
    }
}

/// A list of attributes
///
/// # Example
/// ```ignore
/// #[a]
/// #[a(b)]
/// #[a { .. }]
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct Attributes(Vec<Attribute>);

impl Spanner for Attributes {
    fn span(&self) -> Span {
        match (self.0.first(), self.0.last()) {
            (Some(a), Some(b)) => a.span().join(b.span()),
            (Some(a), None) => a.span(),
            (None, Some(b)) => b.span(),
            _ => Span::default(),
        }
    }
}

impl FromIterator<Attribute> for Attributes {
    fn from_iter<I: IntoIterator<Item = Attribute>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<Vec<Attribute>> for Attributes {
    fn from(v: Vec<Attribute>) -> Self {
        Self(v)
    }
}

impl IntoIterator for Attributes {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = &'a Attribute;
    type IntoIter = std::slice::Iter<'a, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Attributes {
    type Item = &'a mut Attribute;
    type IntoIter = std::slice::IterMut<'a, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for Attributes {
    type Target = Vec<Attribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Attributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.0 {
            attr.to_tokens(tokens);
        }
    }
}

impl Parse for Attributes {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self(stream.parse_while::<Attribute>()))
    }
}

#[cfg(test)]
mod tests {
    use moxy_token::{ToTokenStream, parse};

    use super::meta::Meta;
    use super::*;

    #[test]
    fn outer_empty() {
        let a = parse!("#[inline]" as Attribute).unwrap();
        assert!(a.style.is_outer());
        assert!(a.meta.is_none());
        assert_eq!(a.to_token_stream().to_string(), "# [inline]");
    }

    #[test]
    fn outer_list() {
        let a = parse!("#[derive(Clone, Debug)]" as Attribute).unwrap();
        assert!(a.style.is_outer());
        assert!(a.meta.is_list());
        assert_eq!(a.to_token_stream().to_string(), "# [derive (Clone , Debug)]");
    }

    #[test]
    fn inner() {
        let a = parse!("#![no_std]" as Attribute).unwrap();
        assert!(a.style.is_inner());
        assert_eq!(a.to_token_stream().to_string(), "# ! [no_std]");
    }

    #[test]
    fn name_value() {
        let a = parse!("#[path = \"x.rs\"]" as Attribute).unwrap();
        assert!(a.meta.is_alias());
        assert_eq!(a.to_token_stream().to_string(), "# [path = \"x.rs\"]");
    }

    #[test]
    fn attributes_many() {
        let attrs = parse!("#[a] #[b(1)]" as Attributes).unwrap();
        assert_eq!(attrs.len(), 2);
    }

    #[test]
    fn meta_none() {
        assert!(parse!("inline" as Meta).unwrap().is_none());
    }

    #[test]
    fn meta_literal() {
        let meta = parse!("count = 42" as Meta).unwrap();
        assert!(meta.is_alias());
        assert!(meta.as_value().unwrap().is_literal());
        assert_eq!(meta.as_value().unwrap().as_literal().unwrap().as_u64(), Some(42));
    }

    #[test]
    fn meta_alias() {
        let meta = parse!("path = \"x\"" as Meta).unwrap();
        assert!(meta.is_alias());
        assert!(meta.as_value().is_some());
    }

    #[test]
    fn meta_list() {
        let meta = parse!("derive(Clone, Debug)" as Meta).unwrap();
        let items = meta.as_list().unwrap();
        assert_eq!(items.len(), 2);
        assert!(items.first().unwrap().as_meta().unwrap().is_none());
    }

    #[test]
    fn meta_verbatim() {
        let meta = parse!("custom { a + b }" as Meta).unwrap();
        assert!(meta.as_value().unwrap().is_verbatim());
        assert!(!meta.as_value().unwrap().as_verbatim().unwrap().is_empty());
    }

    #[test]
    fn meta_arg_value() {
        let meta = parse!("debug(\"x\")" as Meta).unwrap();
        let arg = meta.as_list().unwrap().first().unwrap().as_value().unwrap();
        assert_eq!(arg.as_literal().unwrap().as_str().unwrap().repr(), "\"x\"");
    }

    #[test]
    fn meta_arg_nested_meta() {
        let meta = parse!("serde(rename = \"x\")" as Meta).unwrap();
        let inner = meta.as_list().unwrap().first().unwrap().as_meta().unwrap();
        assert!(inner.is_alias());
    }

    #[test]
    fn meta_arg_mixed() {
        let meta = parse!("cfg(unix, target = \"x\")" as Meta).unwrap();
        let items = meta.as_list().unwrap();
        assert!(items.get(0).unwrap().as_meta().unwrap().is_none());
        assert!(items.get(1).unwrap().as_meta().unwrap().is_alias());
    }

    #[test]
    fn meta_not_alias_eqeq() {
        let meta = parse!("flag == y" as Meta).unwrap();
        assert!(!meta.is_alias());
    }

    #[test]
    fn meta_not_alias_fatarrow() {
        let meta = parse!("key => v" as Meta).unwrap();
        assert!(!meta.is_alias());
    }

    #[test]
    fn nested_none() {
        let meta = parse!("outer(inner)" as Meta).unwrap();
        let inner = meta.as_list().unwrap().first().unwrap().as_meta().unwrap();
        assert!(inner.is_none());
    }

    #[test]
    fn nested_bare_literal_rejected() {
        // `inner 42` (a bare literal directly after a path) is no longer valid syntax.
        // Inside a list, the terminated parser surfaces the dangling `42` as an error.
        assert!(parse!("outer(inner 42)" as Meta).is_err());
    }

    #[test]
    fn nested_alias() {
        let meta = parse!("outer(inner = \"x\")" as Meta).unwrap();
        let inner = meta.as_list().unwrap().first().unwrap().as_meta().unwrap();
        assert!(inner.is_alias());
        assert!(inner.as_value().is_some());
    }

    #[test]
    fn nested_list() {
        let meta = parse!("outer(inner(a, b))" as Meta).unwrap();
        let inner = meta.as_list().unwrap().first().unwrap().as_meta().unwrap();
        assert_eq!(inner.as_list().unwrap().len(), 2);
    }

    #[test]
    fn nested_verbatim() {
        let meta = parse!("outer(inner { a + b })" as Meta).unwrap();
        let inner = meta.as_list().unwrap().first().unwrap().as_meta().unwrap();
        assert!(!inner.as_value().unwrap().as_verbatim().unwrap().is_empty());
    }

    #[test]
    fn nested_value() {
        let meta = parse!("outer(\"x\", 42)" as Meta).unwrap();
        let items = meta.as_list().unwrap();
        assert!(items.get(0).unwrap().as_value().is_some());
        assert!(items.get(1).unwrap().as_value().is_some());
    }

    #[test]
    fn nested_mixed() {
        let meta = parse!("cfg(unix, all(target = \"x\", \"lit\"))" as Meta).unwrap();
        let items = meta.as_list().unwrap();
        assert!(items.get(0).unwrap().as_meta().unwrap().is_none());

        let inner = items.get(1).unwrap().as_meta().unwrap().as_list().unwrap();
        assert!(inner.get(0).unwrap().as_meta().unwrap().is_alias());
        assert!(inner.get(1).unwrap().as_value().is_some());
    }

    #[test]
    fn nested_deep() {
        let meta = parse!("a(b(c(d)))" as Meta).unwrap();
        let b = meta.as_list().unwrap().first().unwrap().as_meta().unwrap();
        let c = b.as_list().unwrap().first().unwrap().as_meta().unwrap();
        let d = c.as_list().unwrap().first().unwrap().as_meta().unwrap();
        assert!(d.is_none());
    }
}
