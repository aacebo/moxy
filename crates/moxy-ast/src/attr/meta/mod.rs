mod meta_custom;
mod meta_list;
mod meta_map;
mod meta_name_value;

pub use meta_custom::*;
pub use meta_list::*;
pub use meta_map::*;
pub use meta_name_value::*;
use moxy_token::parser::{Parse, ParseError, ParseStream};
use moxy_token::{Delim, Eq, EqEq, FatArrow, Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Path, Punctuated};

/// A structured attribute meta item (`name`, `name(...)`, `name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Meta {
    /// `#[debug]`
    Path(Path),
    /// `#[debug(true, env = "test")]`
    List(MetaList),
    /// `#[debug = true]`
    NameValue(MetaNameValue),
    /// `#[debug { .. }]`
    Custom(MetaCustom),
}

impl Meta {
    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    pub fn is_name_value(&self) -> bool {
        matches!(self, Self::NameValue(_))
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    pub fn as_path(&self) -> Option<&Path> {
        if let Self::Path(v) = self { Some(v) } else { None }
    }

    pub fn as_list(&self) -> Option<&MetaList> {
        if let Self::List(v) = self { Some(v) } else { None }
    }

    pub fn as_name_value(&self) -> Option<&MetaNameValue> {
        if let Self::NameValue(v) = self { Some(v) } else { None }
    }

    pub fn as_custom(&self) -> Option<&MetaCustom> {
        if let Self::Custom(v) = self { Some(v) } else { None }
    }

    pub fn path(&self) -> &Path {
        match self {
            Self::Path(v) => v,
            Self::List(v) => &v.path,
            Self::NameValue(v) => &v.path,
            Self::Custom(v) => &v.path,
        }
    }
}

impl Parse for Meta {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let path = stream.parse::<Path>()?;

        if let Some(curr) = stream.curr()
            && curr.delim() == Some(Delim::Paren)
            && let Ok(items) = Delimited::parse_paren_with(stream, Punctuated::parse_separated_nonempty)
        {
            return Ok(Self::List(MetaList { path, items }));
        }

        if stream.peek::<Eq>() && !stream.peek::<EqEq>() && !stream.peek::<FatArrow>() {
            return Ok(Self::NameValue(MetaNameValue {
                path,
                eq: stream.parse()?,
                value: stream.parse()?,
            }));
        }

        if stream.is_empty() {
            return Ok(Self::Path(path));
        }

        let mut tokens = TokenStream::new();

        while let Some(tt) = stream.advance() {
            tokens.extend_one(tt.clone());
        }

        Ok(Self::Custom(MetaCustom { path, tokens }))
    }
}

impl Spanner for Meta {
    fn span(&self) -> Span {
        match self {
            Self::Path(p) => p.span(),
            Self::List(l) => l.span(),
            Self::NameValue(nv) => nv.span(),
            Self::Custom(v) => v.span(),
        }
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Path(p) => p.to_tokens(t),
            Self::List(l) => l.to_tokens(t),
            Self::NameValue(nv) => nv.to_tokens(t),
            Self::Custom(v) => v.to_tokens(t),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::{ToTokenStream, TokenStream};

    use super::super::*;
    use super::*;

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn outer_empty() {
        let a = moxy_token::parse!("#[inline]" as Attribute).unwrap();
        assert!(matches!(a.style, AttrStyle::Outer(_)));
        assert!(matches!(a.meta.inner, Meta::Path(_)));
        assert_eq!(render(&a), "# [inline]");
    }

    #[test]
    fn outer_delimited() {
        let a = moxy_token::parse!("#[derive(Clone, Debug)]" as Attribute).unwrap();
        assert!(matches!(a.style, AttrStyle::Outer(_)));
        assert!(matches!(a.meta.inner, Meta::List(_)));
        assert_eq!(render(&a), "# [derive (Clone , Debug)]");
    }

    #[test]
    fn inner() {
        let a = moxy_token::parse!("#![no_std]" as Attribute).unwrap();
        assert!(matches!(a.style, AttrStyle::Inner(..)));
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
        assert!(matches!(a.meta.inner, Meta::NameValue(_)));
        assert_eq!(render(&a), "# [path = \"x.rs\"]");
    }

    #[test]
    fn cfg_delimited() {
        let a = moxy_token::parse!("#[cfg(feature = \"x\")]" as Attribute).unwrap();
        assert!(matches!(a.meta.inner, Meta::List(_)));
    }

    #[test]
    fn meta_forms() {
        assert!(matches!(moxy_token::parse!("inline" as Meta).unwrap(), Meta::Path(_)));
        assert!(matches!(moxy_token::parse!("derive(Clone)" as Meta).unwrap(), Meta::List(_)));
        assert!(matches!(
            moxy_token::parse!("path = \"x\"" as Meta).unwrap(),
            Meta::NameValue(_)
        ));
        assert!(matches!(
            moxy_token::parse!("debug { x = 1 }" as Meta).unwrap(),
            Meta::Custom(_)
        ));
    }

    #[test]
    fn custom_round_trip() {
        let m = moxy_token::parse!("debug { x = 1 }" as Meta).unwrap();
        assert!(matches!(m, Meta::Custom(_)));
        assert_eq!(render(&m), "debug {x = 1}");
    }
}
