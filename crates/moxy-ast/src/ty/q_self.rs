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
        let (qself, trait_path) = Self::parse_with_trait(parser)?;
        let rest = parser.parse()?;
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
        let lt = parser.parse()?;
        let ty = Box::new(parser.parse()?);
        let (as_keyword, trait_path) = if <Token![as]>::peek(parser.cursor()) {
            let as_keyword = parser.parse()?;
            (Some(as_keyword), Some(Path::parse(parser)?))
        } else {
            (None, None)
        };

        let gt = parser.parse()?;
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
        self.lt.span().join(self.ty.span())
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
