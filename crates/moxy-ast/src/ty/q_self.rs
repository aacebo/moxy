use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::Type;
use crate::*;

/// The `<T as Trait>` qualifier of a qualified path.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct QSelf {
    pub lt: Token![<],
    pub ty: Box<Type>,
    pub as_keyword: Option<Token![as]>,
    pub gt: Token![>],
    /// Number of leading path segments that belong inside the `<... as Trait>`.
    pub position: usize,
}

impl QSelf {
    /// Parse a qualified path `<T as Trait>::a::b`, returning the `QSelf` plus the
    /// full merged path (trait segments followed by the trailing segments). Shared
    /// by `TypePath` and expression-path parsing.
    pub fn parse_qualified(parser: &Parser) -> Result<(Self, Path), ParseError> {
        Self::parse_qualified_with(parser, <_ as Parse>::parse)
    }

    /// Parses a qualified path in expression or pattern position, where the
    /// segments after the qualifier take arguments only after `::`, so
    /// `<T>::f()` calls `f`.
    pub(crate) fn parse_qualified_expr(parser: &Parser) -> Result<(Self, Path), ParseError> {
        Self::parse_qualified_with(parser, Path::parse_expr)
    }

    fn parse_qualified_with(
        parser: &Parser,
        parse_rest: fn(&Parser) -> Result<Path, ParseError>,
    ) -> Result<(Self, Path), ParseError> {
        let (qself, trait_path) = Self::parse_with_trait(parser)?;

        // As in rustc, `::` follows the qualifier: `<T>::f`, never `<T> f`
        if !<Token![::]>::peek(parser.cursor()) {
            return parser.error("expected `::` after a qualified path").into();
        }

        let rest = parse_rest(parser)?;
        let path = if let Some(mut p) = trait_path {
            p.extend(rest);
            p
        } else {
            rest
        };

        Ok((qself, path))
    }

    /// Parse `< Type ( as Path )? >`, returning the qself plus the trait path
    /// segments (if any) that the enclosing `TypePath` must prepend to its path.
    pub fn parse_with_trait(parser: &Parser) -> Result<(Self, Option<Path>), ParseError> {
        // A type inside a qualifier takes another level's worth of stack
        let _weight = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
        let lt = <_ as Parse>::parse(parser)?;
        let ty = Box::new(<_ as Parse>::parse(parser)?);
        let (as_keyword, trait_path) = if <Token![as]>::peek(parser.cursor()) {
            let as_keyword = <_ as Parse>::parse(parser)?;
            (Some(as_keyword), Some(Path::parse(parser)?))
        } else {
            (None, None)
        };

        let gt = <_ as Parse>::parse(parser)?;
        let position = trait_path.as_ref().map(|p| p.len()).unwrap_or(0);

        Ok((
            Self {
                lt,
                ty,
                as_keyword,
                gt,
                position,
            },
            trait_path,
        ))
    }
}

impl Spanner for QSelf {
    fn span(&self) -> Span {
        self.lt.span().join(self.gt.span())
    }
}

impl QSelf {
    /// Emits `path` qualified by `qself`, `<ty as Trait>::rest` where the first
    /// `position` segments of `path` belong to the trait, or `path` alone when
    /// it has no qualifier. Every node that holds a `QSelf` beside its path
    /// emits through this.
    pub fn path_to_tokens(qself: Option<&Self>, path: &Path, tokens: &mut TokenStream) {
        let Some(qself) = qself else {
            path.to_tokens(tokens);
            return;
        };

        qself.lt.to_tokens(tokens);
        qself.ty.to_tokens(tokens);
        let segments: Vec<&PathSegment> = path.iter().collect();

        if let Some(as_keyword) = &qself.as_keyword {
            as_keyword.to_tokens(tokens);
            // The trait path's leading `::`, as in `<T as ::a::B>::C`
            path.colon().to_tokens(tokens);

            for (index, segment) in segments[..qself.position].iter().enumerate() {
                if index > 0 {
                    <Token![::]>::default().to_tokens(tokens);
                }

                segment.to_tokens(tokens);
            }
        }

        qself.gt.to_tokens(tokens);

        for segment in &segments[qself.position..] {
            <Token![::]>::default().to_tokens(tokens);
            segment.to_tokens(tokens);
        }
    }
}

impl ToTokens for QSelf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Emits just `< ty >`; the `as Trait` portion is rendered by `TypePath`
        // (it owns the trait segments and the closing `>`/`::` placement).
        self.lt.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.gt.to_tokens(tokens);
    }
}
