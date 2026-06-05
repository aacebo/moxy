use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::PathSep;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path {
    pub span: Span,
    pub leading_colon: bool,
    pub segments: Punctuated<PathSegment, PathSep>,
}

impl Parse for Path {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let start = stream.span();
        let leading_colon = if stream.peek::<PathSep>().is_some() {
            let _ = stream.parse::<PathSep>()?;
            true
        } else {
            false
        };
        let segments = Punctuated::parse_separated_nonempty(stream)?;
        let end = segments.last().map(|s: &PathSegment| s.span).unwrap_or(start);
        Ok(Self {
            span: start.join(end),
            leading_colon,
            segments,
        })
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.leading_colon {
            PathSep::default().to_tokens(tokens);
        }
        self.segments.to_tokens(tokens);
    }
}

impl From<crate::Ident> for Path {
    fn from(ident: crate::Ident) -> Self {
        let span = ident.span;
        let mut segments = Punctuated::new();
        segments.push_value(PathSegment {
            span,
            ident,
            args: PathArguments::None,
        });
        Path {
            span,
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
