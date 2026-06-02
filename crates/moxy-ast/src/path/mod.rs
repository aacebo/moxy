use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;
use moxy_token::punct::PathSep;

use crate::Punctuated;

mod lifetime;
mod lifetime_name;
mod path_arguments;
mod path_segment;

pub use lifetime::*;
pub use lifetime_name::*;
pub use path_arguments::*;
pub use path_segment::*;

#[doc = "A path expression or type path (e.g. `std::collections::HashMap`, `crate::Foo`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path {
    #[parse(skip)]
    pub span: Span,
    #[parse(peek = PathSep)]
    pub leading_colon: bool,
    #[parse(separated)]
    pub segments: Punctuated<PathSegment, PathSep>,
}

impl From<crate::Ident> for Path {
    fn from(ident: crate::Ident) -> Self {
        let mut segments = Punctuated::new();
        segments.push_value(PathSegment {
            span: Span::default(),
            ident,
            args: PathArguments::None,
        });
        Path {
            span: Span::default(),
            leading_colon: false,
            segments,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::{Parse, ToTokenStream, TokenStream};

    use super::*;

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn lifetime() {
        let l = moxy_token::parse!("'a" as Lifetime).unwrap();
        assert_eq!(l.ident.text, "a");
        assert_eq!(render(&l), "'a");

        let s = moxy_token::parse!("'static" as Lifetime).unwrap();
        assert_eq!(s.ident.text, "static");
        assert_eq!(render(&s), "'static");
    }

    #[test]
    fn simple_path() {
        let p = moxy_token::parse!("Foo" as Path).unwrap();
        assert!(!p.leading_colon);
        assert_eq!(p.segments.len(), 1);
        assert_eq!(render(&p), "Foo");
    }

    #[test]
    fn multi_segment() {
        let p = moxy_token::parse!("std::collections::HashMap" as Path).unwrap();
        assert_eq!(p.segments.len(), 3);
        assert_eq!(render(&p), "std :: collections :: HashMap");
    }

    #[test]
    fn leading_colon() {
        let p = moxy_token::parse!("::core::mem" as Path).unwrap();
        assert!(p.leading_colon);
        assert_eq!(p.segments.len(), 2);
        assert_eq!(render(&p), ":: core :: mem");
    }

    #[test]
    fn angle_bracketed() {
        let p = moxy_token::parse!("Vec<T>" as Path).unwrap();
        assert_eq!(p.segments.len(), 1);
        assert!(matches!(p.segments.first().unwrap().args, PathArguments::AngleBracketed(_)));
        assert_eq!(render(&p), "Vec < T >");
    }

    #[test]
    fn nested_generics_shr() {
        let p = moxy_token::parse!("Vec<Box<T>>" as Path).unwrap();
        assert_eq!(p.segments.len(), 1);
        assert!(matches!(p.segments.first().unwrap().args, PathArguments::AngleBracketed(_)));

        let deep = moxy_token::parse!("A<B<C<D>>>" as Path).unwrap();
        assert_eq!(deep.segments.len(), 1);
    }

    #[test]
    fn assoc_type_arg() {
        let p = moxy_token::parse!("Iterator<Item = u8>" as Path).unwrap();
        match &p.segments.first().unwrap().args {
            PathArguments::AngleBracketed(a) => {
                assert!(matches!(a.args.first().unwrap(), crate::GenericArgument::AssocType(_)));
            }
            _ => panic!("expected angle-bracketed"),
        }
    }
}
