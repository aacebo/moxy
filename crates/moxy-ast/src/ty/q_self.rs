use moxy_token::keyword::As;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{Gt, Lt};
use moxy_token::{Span, ToTokens, TokenStream};

use super::Type;
use crate::Path;

#[doc = "The `<T as Trait>` qualifier of a qualified path."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct QSelf {
    pub span: Span,
    pub lt: Lt,
    pub ty: Box<Type>,
    pub as_keyword: Option<As>,
    /// Number of leading path segments that belong inside the `<... as Trait>`.
    pub position: usize,
}

impl QSelf {
    /// Parse a qualified path `<T as Trait>::a::b`, returning the `QSelf` plus the
    /// full merged path (trait segments followed by the trailing segments). Shared
    /// by `TypePath` and expression-path parsing.
    pub fn parse_qualified(stream: &mut ParseStream) -> Result<(Self, Path), ParseError> {
        let (qself, trait_path) = Self::parse_with_trait(stream)?;
        let _ = stream.parse::<moxy_token::punct::PathSep>()?;
        let rest = stream.parse::<Path>()?;

        let mut segments = trait_path.map(|p| p.segments).unwrap_or_default();

        for seg in rest.segments {
            segments.push(seg);
        }

        Ok((
            qself,
            Path {
                span: Span::default(),
                leading_colon: false,
                segments,
            },
        ))
    }

    /// Parse `< Type ( as Path )? >`, returning the qself plus the trait path
    /// segments (if any) that the enclosing `TypePath` must prepend to its path.
    pub fn parse_with_trait(stream: &mut ParseStream) -> Result<(Self, Option<Path>), ParseError> {
        let lt = stream.parse::<Lt>()?;
        let ty = Box::new(stream.parse::<Type>()?);

        let (as_keyword, trait_path) = if stream.peek::<As>().is_some() {
            let as_keyword = stream.parse::<As>()?;
            (Some(as_keyword), Some(stream.parse::<Path>()?))
        } else {
            (None, None)
        };

        stream.eat_angle_close()?;

        let position = trait_path.as_ref().map(|p| p.segments.len()).unwrap_or(0);

        Ok((
            Self {
                span: Span::default(),
                lt,
                ty,
                as_keyword,
                position,
            },
            trait_path,
        ))
    }
}

impl ToTokens for QSelf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Emits just `< ty >`; the `as Trait` portion is rendered by `TypePath`
        // (it owns the trait segments and the closing `>`/`::` placement).
        self.lt.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        Gt::default().to_tokens(tokens);
    }
}
