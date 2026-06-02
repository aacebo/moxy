use crate::ast::Ident;
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::As;
use crate::token::punct::{PathSep, Star};
use crate::token::{Delim, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

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
            let _ = stream.parse::<Star>()?;
            return Ok(UseTree::Glob(UseGlob { span }));
        }

        if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            let group = stream.parse_group(Delim::Brace)?;
            let mut inner = group.parse();
            let items = crate::ast::Punctuated::parse_terminated(&mut inner)?;
            return Ok(UseTree::Group(UseGroup {
                span: Span::default(),
                items,
            }));
        }

        let ident = stream.parse::<Ident>()?;

        if stream.peek::<PathSep>().is_some() {
            let _ = stream.parse::<PathSep>()?;
            let tree = Box::new(stream.parse::<UseTree>()?);
            return Ok(UseTree::Path(UsePath {
                span: Span::default(),
                ident,
                tree,
            }));
        }

        if stream.peek::<As>().is_some() {
            let _ = stream.parse::<As>()?;
            let rename = stream.parse::<Ident>()?;
            return Ok(UseTree::Rename(UseRename {
                span: Span::default(),
                ident,
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
