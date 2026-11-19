mod use_glob;
mod use_group;
mod use_name;
mod use_path_chain;
mod use_rename;

pub use use_glob::*;
pub use use_group::*;
pub use use_name::*;
pub use use_path_chain::*;
pub use use_rename::*;

use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A `use` import tree.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UseTree {
    PathChain(UsePathChain),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
}

impl UseTree {
    pub fn is_path_chain(&self) -> bool {
        matches!(self, Self::PathChain(_))
    }

    pub fn is_name(&self) -> bool {
        matches!(self, Self::Name(_))
    }

    pub fn is_rename(&self) -> bool {
        matches!(self, Self::Rename(_))
    }

    pub fn is_glob(&self) -> bool {
        matches!(self, Self::Glob(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn as_path_chain(&self) -> Option<&UsePathChain> {
        if let Self::PathChain(v) = self { Some(v) } else { None }
    }

    pub fn as_name(&self) -> Option<&UseName> {
        if let Self::Name(v) = self { Some(v) } else { None }
    }

    pub fn as_rename(&self) -> Option<&UseRename> {
        if let Self::Rename(v) = self { Some(v) } else { None }
    }

    pub fn as_glob(&self) -> Option<&UseGlob> {
        if let Self::Glob(v) = self { Some(v) } else { None }
    }

    pub fn as_group(&self) -> Option<&UseGroup> {
        if let Self::Group(v) = self { Some(v) } else { None }
    }
}

impl Spanner for UseTree {
    fn span(&self) -> Span {
        match self {
            Self::PathChain(v) => v.span(),
            Self::Name(v) => v.span(),
            Self::Rename(v) => v.span(),
            Self::Glob(v) => v.span(),
            Self::Group(v) => v.span(),
        }
    }
}

impl Parse for UseTree {
    fn peek(cursor: Cursor<'_>) -> bool {
        UseGlob::peek(cursor)
            || UseGroup::peek(cursor)
            || UsePathChain::peek(cursor)
            || UseRename::peek(cursor)
            || UseName::peek(cursor)
    }

    // A brace group nests the trees inside it one level, as in `a::{b::{c}}`
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let _depth = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;

        if UseGlob::peek(parser.cursor()) {
            return Ok(Self::Glob(<_ as Parse>::parse(parser)?));
        }

        if UseGroup::peek(parser.cursor()) {
            return Ok(Self::Group(<_ as Parse>::parse(parser)?));
        }

        if UsePathChain::peek(parser.cursor()) {
            return Ok(Self::PathChain(<_ as Parse>::parse(parser)?));
        }

        if UseRename::peek(parser.cursor()) {
            return Ok(Self::Rename(<_ as Parse>::parse(parser)?));
        }

        Ok(Self::Name(<_ as Parse>::parse(parser)?))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let _depth = moxy_token::__private::Depth::enter()?;

        if UseGlob::peek(cursor) {
            UseGlob::skip(cursor)
        } else if UseGroup::peek(cursor) {
            UseGroup::skip(cursor)
        } else if UsePathChain::peek(cursor) {
            UsePathChain::skip(cursor)
        } else if UseRename::peek(cursor) {
            UseRename::skip(cursor)
        } else {
            UseName::skip(cursor)
        }
    }
}

impl ToTokens for UseTree {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::PathChain(v) => v.to_tokens(t),
            Self::Name(v) => v.to_tokens(t),
            Self::Rename(v) => v.to_tokens(t),
            Self::Glob(v) => v.to_tokens(t),
            Self::Group(v) => v.to_tokens(t),
        }
    }
}

// A use path segment: an identifier, including a weak keyword such as `raw`,
// or one of the path keywords `crate`, `self`, `super`, and `Self`
fn peek_segment(cursor: Cursor<'_>) -> bool {
    Ident::peek(cursor)
        || cursor.curr().is_some_and(|token| {
            token.is_keyword_crate() || token.is_keyword_self_value() || token.is_keyword_super() || token.is_keyword_self_type()
        })
}

// Skips the leading `::` of a path, if any
fn skip_prefix(cursor: Cursor<'_>) -> Cursor<'_> {
    Option::<Token![::]>::skip(cursor).unwrap_or(cursor)
}

fn skip_segment(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    peek_segment(cursor).then(|| cursor.offset(1))
}
