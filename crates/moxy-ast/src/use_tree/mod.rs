use moxy_token::keyword::As;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{PathSep, Star};
use moxy_token::{Delim, Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::{Delimited, Ident};

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

/// A `use` import tree.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Star>() {
            let star = stream.parse::<Star>()?;
            return Ok(Self::Glob(UseGlob { star }));
        }

        if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let items = Delimited::parse_brace_with(stream, crate::Punctuated::parse_terminated)?;
            return Ok(Self::Group(UseGroup { items }));
        }

        let prefix = stream.parse_if::<PathSep>();
        let ident = stream.parse::<Ident>()?;

        if stream.peek::<PathSep>() {
            let path_sep = stream.parse::<PathSep>()?;
            let tree = Box::new(stream.parse::<Self>()?);
            return Ok(Self::Path(UsePath {
                prefix,
                ident,
                path_sep,
                tree,
            }));
        }

        if stream.peek::<As>() {
            let as_keyword = stream.parse::<As>()?;
            let rename = stream.parse::<Ident>()?;
            return Ok(Self::Rename(UseRename {
                ident,
                as_keyword,
                rename,
            }));
        }

        Ok(Self::Name(UseName { ident }))
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
