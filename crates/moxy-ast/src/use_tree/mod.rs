mod use_glob;
mod use_group;
mod use_name;
mod use_path;
mod use_rename;

pub use use_glob::*;
pub use use_group::*;
pub use use_name::*;
pub use use_path::*;
pub use use_rename::*;

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A `use` import tree.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UseTree {
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
}

impl UseTree {
    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
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

    pub fn as_path(&self) -> Option<&UsePath> {
        if let Self::Path(v) = self { Some(v) } else { None }
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
            Self::Path(v) => v.span(),
            Self::Name(v) => v.span(),
            Self::Rename(v) => v.span(),
            Self::Glob(v) => v.span(),
            Self::Group(v) => v.span(),
        }
    }
}

impl Parse for UseTree {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<UseGlob>()
            || cursor.peek::<UseGroup>()
            || cursor.peek::<UsePath>()
            || cursor.peek::<UseRename>()
            || cursor.peek::<UseName>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<UseGlob>() {
            return Ok(Self::Glob(parser.parse()?));
        }

        if parser.peek::<UseGroup>() {
            return Ok(Self::Group(parser.parse()?));
        }

        if parser.peek::<UsePath>() {
            return Ok(Self::Path(parser.parse()?));
        }

        if parser.peek::<UseRename>() {
            return Ok(Self::Rename(parser.parse()?));
        }

        Ok(Self::Name(parser.parse()?))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<UseGlob>() {
            cursor.skip::<UseGlob>()
        } else if cursor.peek::<UseGroup>() {
            cursor.skip::<UseGroup>()
        } else if cursor.peek::<UsePath>() {
            cursor.skip::<UsePath>()
        } else if cursor.peek::<UseRename>() {
            cursor.skip::<UseRename>()
        } else {
            cursor.skip::<UseName>()
        }
    }
}

impl ToTokens for UseTree {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Path(v) => v.to_tokens(t),
            Self::Name(v) => v.to_tokens(t),
            Self::Rename(v) => v.to_tokens(t),
            Self::Glob(v) => v.to_tokens(t),
            Self::Group(v) => v.to_tokens(t),
        }
    }
}
