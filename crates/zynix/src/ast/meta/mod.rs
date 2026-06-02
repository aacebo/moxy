pub mod attr;
mod meta_list;
mod meta_name_value;

pub use attr::Attribute;
pub use meta_list::*;
pub use meta_name_value::*;

use crate::ast::{Expr, Path};
use crate::parse::{Parse, ParseError, ParseStream};
use crate::token::{Eq, ToTokens};
use crate::{Span, TokenStream};

#[doc = "A structured attribute meta item (`name`, `name(...)`, `name = expr`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Meta {
    Path(super::Path),
    List(MetaList),
    NameValue(MetaNameValue),
}

impl Parse for Meta {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let path = stream.parse::<Path>()?;

        if let Some(crate::token::TokenTree::Group(group)) = stream.curr() {
            let delim = group.delim;
            let tokens = stream.parse_group(delim)?;

            return Ok(Self::List(MetaList {
                span: Span::default(),
                path,
                delim,
                tokens,
            }));
        }

        if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            let value = stream.parse::<Expr>()?;

            return Ok(Meta::NameValue(MetaNameValue {
                span: Span::default(),
                path,
                value,
            }));
        }

        Ok(Self::Path(path))
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Path(p) => p.to_tokens(t),
            Self::List(l) => l.to_tokens(t),
            Self::NameValue(nv) => nv.to_tokens(t),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::attr::*;
    use super::*;
    use crate::token::ToTokenStream;
    use crate::{Parse, TokenStream};

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    // `TokenStream` Display spaces top-level tokens (only the lifetime tick is
    // glued), so `#`/`#!` render with a space before the `[...]` group.
    #[test]
    fn outer_empty() {
        let a: Attribute = parse("#[inline]");
        assert_eq!(a.style, AttrStyle::Outer);
        assert!(matches!(a.args, AttrArgs::Empty));
        assert_eq!(render(&a), "# [inline]");
    }

    #[test]
    fn outer_delimited() {
        let a: Attribute = parse("#[derive(Clone, Debug)]");
        assert_eq!(a.style, AttrStyle::Outer);
        assert!(matches!(a.args, AttrArgs::Delimited { .. }));
        assert_eq!(render(&a), "# [derive (Clone , Debug)]");
    }

    #[test]
    fn inner() {
        let a: Attribute = parse("#![no_std]");
        assert_eq!(a.style, AttrStyle::Inner);
        assert_eq!(render(&a), "# ! [no_std]");
    }

    #[test]
    fn many() {
        let attrs: Vec<Attribute> = {
            let ts = TokenStream::from_str("#[a] #[b(1)]").unwrap();
            let mut ps = ts.parse();
            let mut out = Vec::new();
            while !ps.is_empty() {
                out.push(ps.parse::<Attribute>().unwrap());
            }
            out
        };
        assert_eq!(attrs.len(), 2);
    }

    #[test]
    fn name_value() {
        let a: Attribute = parse("#[path = \"x.rs\"]");
        assert!(matches!(a.args, AttrArgs::NameValue(_)));
        assert_eq!(render(&a), "# [path = \"x.rs\"]");
    }

    #[test]
    fn cfg_delimited() {
        let a: Attribute = parse("#[cfg(feature = \"x\")]");
        assert!(matches!(a.args, AttrArgs::Delimited { .. }));
    }

    #[test]
    fn meta_forms() {
        assert!(matches!(parse::<Meta>("inline"), Meta::Path(_)));
        assert!(matches!(parse::<Meta>("derive(Clone)"), Meta::List(_)));
        assert!(matches!(parse::<Meta>("path = \"x\""), Meta::NameValue(_)));
    }
}
