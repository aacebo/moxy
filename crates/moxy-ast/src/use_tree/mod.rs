use moxy_token::keyword::As;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{PathSep, Star};
use moxy_token::{Delim, Parse, Span, ToTokens, TokenStream, TokenTree};

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

#[doc = "A `use` import tree."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UseTree {
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
}

impl Parse for UseTree {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Star>().is_some() {
            let span = stream.span();
            let star = stream.parse::<Star>()?;
            return Ok(UseTree::Glob(UseGlob { span, star }));
        }

        if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let items = Delimited::parse_brace_with(stream, crate::Punctuated::parse_terminated)?;
            return Ok(UseTree::Group(UseGroup {
                span: Span::default(),
                items,
            }));
        }

        let ident = stream.parse::<Ident>()?;

        if stream.peek::<PathSep>().is_some() {
            let path_sep = stream.parse::<PathSep>()?;
            let tree = Box::new(stream.parse::<UseTree>()?);
            return Ok(UseTree::Path(UsePath {
                span: Span::default(),
                ident,
                path_sep,
                tree,
            }));
        }

        if stream.peek::<As>().is_some() {
            let as_keyword = stream.parse::<As>()?;
            let rename = stream.parse::<Ident>()?;
            return Ok(UseTree::Rename(UseRename {
                span: Span::default(),
                ident,
                as_keyword,
                rename,
            }));
        }

        Ok(UseTree::Name(UseName {
            span: Span::default(),
            ident,
        }))
    }
}

impl ToTokens for UseTree {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            UseTree::Path(v) => v.to_tokens(t),
            UseTree::Name(v) => v.to_tokens(t),
            UseTree::Rename(v) => v.to_tokens(t),
            UseTree::Glob(v) => v.to_tokens(t),
            UseTree::Group(v) => v.to_tokens(t),
        }
    }
}
