use moxy_token::{Delim, DelimSpan, Group, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A trait bound (`Trait`, `?Sized`, `for<'a> Trait`), possibly in parentheses
/// (`(Fn() -> u8)`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitBound {
    /// The parentheses around the bound, as in `dyn (Fn() -> u8) + Send`.
    /// Serialization writes only whether they are present.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_present"))]
    pub paren: Option<DelimSpan>,
    pub polarity: Option<Token![!]>,
    pub lifetimes: Option<BoundLifetimes>,
    pub modifier: Option<Token![?]>,
    pub path: Path,
}

impl Parse for TraitBound {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.is_delimited(Delim::Paren)
            || <Token![!]>::peek(cursor)
            || <Token![for]>::peek(cursor)
            || <Token![?]>::peek(cursor)
            || Path::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.is_delimited(Delim::Paren) {
            let (span, inner) = parser.parse_group_spanned(Delim::Paren)?;
            let mut bound = Self::parse_bare(&inner)?;
            inner.ensure_empty()?;
            bound.paren = Some(span);
            return Ok(bound);
        }

        Self::parse_bare(parser)
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.is_delimited(Delim::Paren) {
            let inner = Self::skip_bare(cursor.descend(Delim::Paren)?)?;
            return inner.is_empty().then(|| cursor.offset(1));
        }

        Self::skip_bare(cursor)
    }
}

impl TraitBound {
    // A bound without parentheses
    fn parse_bare(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            paren: None,
            polarity: <_ as Parse>::parse(parser)?,
            lifetimes: <_ as Parse>::parse(parser)?,
            modifier: <_ as Parse>::parse(parser)?,
            path: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip_bare(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![!]>::skip(cursor)?;
        cursor = Option::<BoundLifetimes>::skip(cursor)?;
        cursor = Option::<Token![?]>::skip(cursor)?;
        Path::skip(cursor)
    }
}

impl Spanner for TraitBound {
    fn span(&self) -> Span {
        let start = self
            .polarity
            .as_ref()
            .map(Spanner::span)
            .or_else(|| self.lifetimes.as_ref().map(Spanner::span))
            .or_else(|| self.modifier.as_ref().map(Spanner::span))
            .unwrap_or_else(|| self.path.span());

        match self.paren {
            Some(paren) => paren.span(),
            None => start.join(self.path.span()),
        }
    }
}

impl ToTokens for TraitBound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut bound = TokenStream::new();
        self.polarity.to_tokens(&mut bound);
        self.lifetimes.to_tokens(&mut bound);
        self.modifier.to_tokens(&mut bound);
        self.path.to_tokens(&mut bound);

        match self.paren {
            Some(span) => tokens.extend_one(TokenTree::Group(Group {
                delim: Delim::Paren,
                span,
                tokens: bound,
            })),
            None => tokens.extend(bound),
        }
    }
}

#[cfg(feature = "serde")]
fn serialize_present<S: serde::Serializer>(paren: &Option<DelimSpan>, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_bool(paren.is_some())
}
