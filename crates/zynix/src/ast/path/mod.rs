use zynix_macros::{Parse, ToTokens};

use crate::Span;
use crate::ast::Punctuated;
use crate::token::punct::PathSep;

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

impl From<crate::ast::Ident> for Path {
    fn from(ident: crate::ast::Ident) -> Self {
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

    #[test]
    fn lifetime() {
        let l: Lifetime = parse("'a");
        assert_eq!(l.ident.text, "a");
        assert_eq!(render(&l), "'a");

        let s: Lifetime = parse("'static");
        assert_eq!(s.ident.text, "static");
        assert_eq!(render(&s), "'static");
    }

    #[test]
    fn simple_path() {
        let p: Path = parse("Foo");
        assert!(!p.leading_colon);
        assert_eq!(p.segments.len(), 1);
        assert_eq!(render(&p), "Foo");
    }

    #[test]
    fn multi_segment() {
        let p: Path = parse("std::collections::HashMap");
        assert_eq!(p.segments.len(), 3);
        assert_eq!(render(&p), "std :: collections :: HashMap");
    }

    #[test]
    fn leading_colon() {
        let p: Path = parse("::core::mem");
        assert!(p.leading_colon);
        assert_eq!(p.segments.len(), 2);
        assert_eq!(render(&p), ":: core :: mem");
    }

    #[test]
    fn angle_bracketed() {
        let p: Path = parse("Vec<T>");
        assert_eq!(p.segments.len(), 1);
        assert!(matches!(p.segments.first().unwrap().args, PathArguments::AngleBracketed(_)));
        assert_eq!(render(&p), "Vec < T >");
    }

    #[test]
    fn nested_generics_shr() {
        // `Vec<Box<T>>` closes with a single `>>` token; structured parsing must split it.
        let p: Path = parse("Vec<Box<T>>");
        assert_eq!(p.segments.len(), 1);
        assert!(matches!(p.segments.first().unwrap().args, PathArguments::AngleBracketed(_)));

        let deep: Path = parse("A<B<C<D>>>");
        assert_eq!(deep.segments.len(), 1);
    }

    #[test]
    fn assoc_type_arg() {
        let p: Path = parse("Iterator<Item = u8>");
        match &p.segments.first().unwrap().args {
            PathArguments::AngleBracketed(a) => {
                assert!(matches!(a.args.first().unwrap(), crate::ast::GenericArgument::AssocType(_)));
            }
            _ => panic!("expected angle-bracketed"),
        }
    }
}
