pub mod attr;
mod meta_list;
mod meta_name_value;

pub use attr::Attribute;
pub use meta_list::*;
pub use meta_name_value::*;
use moxy_token::parse::{Parse, ParseError, ParseStream};
use moxy_token::{Eq, Span, ToTokens, TokenStream};

use crate::{Expr, Path};

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

        if let Some(moxy_token::TokenTree::Group(group)) = stream.curr() {
            let delim = group.delim();
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

    use moxy_token::{Parse, ToTokenStream, TokenStream};

    use super::attr::*;
    use super::*;

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn outer_empty() {
        let a = moxy_token::parse!("#[inline]" as Attribute).unwrap();
        assert_eq!(a.style, AttrStyle::Outer);
        assert!(matches!(a.args, AttrArgs::Empty));
        assert_eq!(render(&a), "# [inline]");
    }

    #[test]
    fn outer_delimited() {
        let a = moxy_token::parse!("#[derive(Clone, Debug)]" as Attribute).unwrap();
        assert_eq!(a.style, AttrStyle::Outer);
        assert!(matches!(a.args, AttrArgs::Delimited { .. }));
        assert_eq!(render(&a), "# [derive (Clone , Debug)]");
    }

    #[test]
    fn inner() {
        let a = moxy_token::parse!("#![no_std]" as Attribute).unwrap();
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
        let a = moxy_token::parse!("#[path = \"x.rs\"]" as Attribute).unwrap();
        assert!(matches!(a.args, AttrArgs::NameValue(_)));
        assert_eq!(render(&a), "# [path = \"x.rs\"]");
    }

    #[test]
    fn cfg_delimited() {
        let a = moxy_token::parse!("#[cfg(feature = \"x\")]" as Attribute).unwrap();
        assert!(matches!(a.args, AttrArgs::Delimited { .. }));
    }

    #[test]
    fn meta_forms() {
        assert!(matches!(moxy_token::parse!("inline" as Meta).unwrap(), Meta::Path(_)));
        assert!(matches!(moxy_token::parse!("derive(Clone)" as Meta).unwrap(), Meta::List(_)));
        assert!(matches!(
            moxy_token::parse!("path = \"x\"" as Meta).unwrap(),
            Meta::NameValue(_)
        ));
    }
}
