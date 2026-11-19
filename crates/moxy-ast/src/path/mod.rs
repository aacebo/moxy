mod arguments;
mod segment;

pub use arguments::*;
pub use segment::*;

use moxy_token::{Ident, Keyword, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A path expression or type path (e.g. `std::collections::HashMap`, `crate::Foo`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path {
    colon: Option<Token![::]>,
    segments: List<PathSegment, Token![::]>,
}

impl Path {
    pub fn try_from_str(value: impl AsRef<str>) -> Result<Self, ParseError> {
        std::str::FromStr::from_str(value.as_ref())
    }

    pub fn colon(&self) -> Option<Token![::]> {
        self.colon
    }

    pub fn as_ident(&self) -> Option<&Ident> {
        if self.colon.is_none() && self.segments.len() == 1 {
            return self.segments.first().map(|s| &s.ident);
        }

        None
    }

    pub fn is_ident(&self, ident: impl AsRef<str>) -> bool {
        let Some(v) = self.as_ident() else {
            return false;
        };

        v == ident.as_ref()
    }

    pub(crate) fn parse_rest(parser: &Parser, first: PathSegment) -> Result<Self, ParseError> {
        let mut segments = List::new();
        segments.push_value(first);

        while <Token![::]>::skip(parser.cursor()).is_some_and(|cursor| PathSegment::peek(cursor)) {
            segments.push_punct(<_ as Parse>::parse(parser)?);
            segments.push_value(<_ as Parse>::parse(parser)?);
        }

        Ok(Self { colon: None, segments })
    }

    pub(crate) fn skip_rest(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while <Token![::]>::skip(cursor).is_some_and(|cursor| PathSegment::peek(cursor)) {
            cursor = <Token![::]>::skip(cursor)?;
            cursor = PathSegment::skip(cursor)?;
        }

        Some(cursor)
    }
}

impl Path {
    /// Parses a path whose segments take no arguments, as the path of an
    /// attribute: `#[derive(Clone)]` names `derive`, not `derive(Clone)`.
    pub(crate) fn parse_mod_style(parser: &Parser) -> Result<Self, ParseError> {
        let colon = <_ as Parse>::parse(parser)?;
        let mut segments = List::new();
        segments.push_value(PathSegment {
            ident: parser.parse_ident_any()?,
            args: PathArguments::None,
        });

        while <Token![::]>::skip(parser.cursor()).is_some_and(|cursor| Ident::peek(cursor) || Keyword::peek(cursor)) {
            segments.push_punct(<_ as Parse>::parse(parser)?);
            segments.push_value(PathSegment {
                ident: parser.parse_ident_any()?,
                args: PathArguments::None,
            });
        }

        Ok(Self { colon, segments })
    }

    pub(crate) fn skip_mod_style(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        // Any keyword names a segment, as `parse_mod_style` reads one
        let name = |cursor: Cursor<'_>| Ident::peek(cursor) || Keyword::peek(cursor);
        cursor = Option::<Token![::]>::skip(cursor)?;
        cursor = name(cursor).then(|| cursor.offset(1))?;

        while <Token![::]>::skip(cursor).is_some_and(name) {
            cursor = <Token![::]>::skip(cursor)?.offset(1);
        }

        Some(cursor)
    }

    /// Parses a path in expression position, where generic arguments need the
    /// turbofish (`f::<T>`), so `a < b` stays a comparison.
    pub(crate) fn parse_expr(parser: &Parser) -> Result<Self, ParseError> {
        let colon = <_ as Parse>::parse(parser)?;
        let mut segments = List::new();
        segments.push_value(PathSegment::parse_expr(parser)?);

        while <Token![::]>::skip(parser.cursor()).is_some_and(|cursor| PathSegment::peek(cursor)) {
            segments.push_punct(<_ as Parse>::parse(parser)?);
            segments.push_value(PathSegment::parse_expr(parser)?);
        }

        Ok(Self { colon, segments })
    }

    pub(crate) fn skip_expr(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![::]>::skip(cursor)?;
        cursor = PathSegment::skip_expr(cursor)?;

        while <Token![::]>::skip(cursor).is_some_and(|cursor| PathSegment::peek(cursor)) {
            cursor = <Token![::]>::skip(cursor)?;
            cursor = PathSegment::skip_expr(cursor)?;
        }

        Some(cursor)
    }
}

impl Parse for Path {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![::]>::peek(cursor) || PathSegment::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let colon = <_ as Parse>::parse(parser)?;
        let first = <_ as Parse>::parse(parser)?;
        let mut path = Self::parse_rest(parser, first)?;
        path.colon = colon;
        Ok(path)
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![::]>::skip(cursor)?;
        cursor = PathSegment::skip(cursor)?;
        Self::skip_rest(cursor)
    }
}

impl Spanner for Path {
    fn span(&self) -> Span {
        // The first segment starts with its name, so its arguments need no
        // span, and a path of one segment spans its arguments once
        let first = self
            .colon
            .map(|colon| colon.span())
            .or_else(|| self.segments.first().map(|segment| segment.ident.span()));
        let last = self.segments.last().map(Spanner::span);

        match (first, last) {
            (Some(first), Some(last)) => first.join(last),
            (Some(first), None) => first,
            _ => Span::default(),
        }
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.colon.to_tokens(tokens);
        self.segments.to_tokens(tokens);
    }
}

impl std::str::FromStr for Path {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = TokenStream::from_str(s)?;
        Self::parse(&Parser::from_tokens(&parser))
    }
}

impl From<Ident> for Path {
    fn from(ident: Ident) -> Self {
        let mut segments = List::new();

        segments.push_value(PathSegment {
            ident,
            args: PathArguments::None,
        });

        Self { colon: None, segments }
    }
}

impl From<Vec<PathSegment>> for Path {
    fn from(value: Vec<PathSegment>) -> Self {
        Self {
            colon: None,
            segments: List::from_iter(value),
        }
    }
}

impl Extend<PathSegment> for Path {
    fn extend<T: IntoIterator<Item = PathSegment>>(&mut self, iter: T) {
        self.segments.extend(iter);
    }
}

impl std::ops::Deref for Path {
    type Target = List<PathSegment, Token![::]>;

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl std::ops::DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.segments
    }
}

impl IntoIterator for Path {
    type IntoIter = IntoIter<PathSegment>;
    type Item = PathSegment;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter()
    }
}

impl std::hash::Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(colon) = &self.colon {
            colon.hash(state);
        }

        for (seg, sep) in self.pairs().map(|v| v.into_tuple()) {
            seg.hash(state);

            if let Some(colon) = sep {
                colon.hash(state);
            }
        }
    }
}
