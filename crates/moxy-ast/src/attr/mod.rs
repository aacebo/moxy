mod meta;
mod style;

pub use meta::*;
pub use style::*;

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// Provides read-only access to the attributes attached to an AST node.
///
/// Implementations return the node's attributes in source order. This trait is
/// implemented for concrete AST nodes that store attributes and for wrapper
/// enums whose variants all store attributes.
///
/// Use [`Self::parse_meta`] to find the first attribute with a matching
/// single-segment path and convert its metadata with [`FromMeta`].
pub trait Attributed {
    /// Returns the attributes attached directly to this node.
    fn attrs(&self) -> &[Attribute];

    /// Parses the metadata of the first attribute named `name`.
    ///
    /// Returns `Ok(None)` when this node has no matching attribute. If a
    /// matching attribute is present, conversion is delegated to
    /// [`FromMeta::from_meta`].
    fn parse_meta<T: FromMeta>(&self, name: &str) -> Result<Option<T>, ParseError> {
        for attr in self.attrs() {
            if attr.path.is_ident(name) {
                return Ok(Some(T::from_meta(&attr.meta)?));
            }
        }

        Ok(None)
    }
}

/// A Rust attribute (`#[...]` or `#![...]`) applied to an item, expression, or statement.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.style.to_tokens(tokens);
        self.meta.to_tokens(tokens);
    }
}

impl Parse for Attribute {
    fn peek(cursor: Cursor<'_>) -> bool {
        AttrStyle::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            style: <_ as Parse>::parse(parser)?,
            meta: Delimited::parse_bracket(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = AttrStyle::skip(cursor)?;
        let meta = Meta::skip(cursor.descend(moxy_token::Delim::Bracket)?)?;
        meta.is_empty().then(|| cursor.offset(1))
    }
}

impl std::ops::Deref for Attribute {
    type Target = Meta;

    fn deref(&self) -> &Self::Target {
        &self.meta
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
#[derive(Clone, Default)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct Attributes(Vec<Attribute>);

impl Attributes {
    pub fn for_each<P>(&self, mut parse: P) -> Result<(), ParseError>
    where
        P: FnMut(&Attribute) -> Result<(), ParseError>,
    {
        for attr in &self.0 {
            parse(attr)?;
        }

        Ok(())
    }
}

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
    fn peek(cursor: Cursor<'_>) -> bool {
        Attribute::peek(cursor)
    }

    // Outer attributes; an inner attribute here is an error, as rustc reports
    // it, since only the start of a file or of a body holds inner ones
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs: Vec<Attribute> = parser.parse_while();
        let mut outer = false;

        for attr in &attrs {
            if attr.style.is_inner() {
                let message = if outer {
                    "an inner attribute is not permitted following an outer attribute"
                } else {
                    "an inner attribute is not permitted in this context"
                };

                return Err(ParseError::new(attr.span(), message));
            }

            outer = true;
        }

        Ok(Self(attrs))
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while let Some(next) = Attribute::skip(cursor) {
            if <Token![!]>::peek(cursor.offset(1)) {
                return None;
            }

            cursor = next;
        }

        Some(cursor)
    }
}

impl Attributes {
    // The inner attributes that open a file or a body, as `#![a]` in
    // `mod m { #![a] }`
    pub(crate) fn parse_inner(parser: &Parser) -> Result<Self, ParseError> {
        let mut attrs = Self::default();

        while Self::peek_inner(parser.cursor()) {
            attrs.push(<_ as Parse>::parse(parser)?);
        }

        Ok(attrs)
    }

    pub(crate) fn skip_inner(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while Self::peek_inner(cursor) {
            cursor = Attribute::skip(cursor)?;
        }

        Some(cursor)
    }

    fn peek_inner(cursor: Cursor<'_>) -> bool {
        <Token![#]>::peek(cursor)
            && <Token![!]>::peek(cursor.offset(1))
            && cursor.offset(2).is_delimited(moxy_token::Delim::Bracket)
    }
}
