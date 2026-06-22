use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, Token, TokenStream};

use crate::{IntoIter, Punctuated};

mod arguments;
mod segment;

pub use arguments::*;
pub use segment::*;

#[macro_export]
macro_rules! path {
    ($x:ident) => { $crate::Path::try_from_str(stringify!($x)).expect("invalid syntax") };
    ($head:ident :: $($tail:tt)+) => {{
        let mut __ident = stringify!($head).to_string();
        path!(@accum __ident, $($tail)+)
    }};
    (@accum $acc:ident, $next:ident :: $($tail:tt)+) => {{
        $acc += "::";
        $acc += stringify!($next);
        path!(@accum $acc, $($tail)+)
    }};
    (@accum $acc:ident, $last:ident) => {{
        $acc += "::";
        $acc += stringify!($last);
        $crate::Path::try_from_str($acc).expect("invalid syntax")
    }};
}

/// A path expression or type path (e.g. `std::collections::HashMap`, `crate::Foo`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Path {
    leading_colon: Option<Token![::]>,
    segments: Punctuated<PathSegment, Token![::]>,
}

impl Path {
    pub fn try_from_str(value: impl AsRef<str>) -> Result<Self, ParseError> {
        std::str::FromStr::from_str(value.as_ref())
    }

    pub fn leading_colon(&self) -> Option<Token![::]> {
        self.leading_colon
    }

    pub fn as_ident(&self) -> Option<&crate::Ident> {
        if self.leading_colon.is_none() && self.segments.len() == 1 {
            return self.segments.first().map(|s| &s.ident);
        }

        None
    }
}

impl Parse for Path {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let leading_colon = stream.parse_if::<Token![::]>();
        let segments = Punctuated::parse_separated_nonempty(stream)?;

        Ok(Self { leading_colon, segments })
    }
}

impl Spanner for Path {
    fn span(&self) -> Span {
        match (self.leading_colon, self.segments.last()) {
            (Some(colon), Some(segment)) => colon.span().join(segment.span()),
            (Some(colon), None) => colon.span(),
            (None, Some(segment)) => segment.span(),
            _ => Span::default(),
        }
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(colon) = self.leading_colon {
            colon.to_tokens(tokens);
        }

        self.segments.to_tokens(tokens);
    }
}

impl std::str::FromStr for Path {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stream = TokenStream::from_str(s)?;
        Self::parse(&mut stream.parse())
    }
}

impl From<crate::Ident> for Path {
    fn from(ident: crate::Ident) -> Self {
        let mut segments = Punctuated::new();

        segments.push_value(PathSegment {
            ident,
            args: PathArguments::None,
        });

        Self {
            leading_colon: None,
            segments,
        }
    }
}

impl From<Vec<PathSegment>> for Path {
    fn from(value: Vec<PathSegment>) -> Self {
        Self {
            leading_colon: None,
            segments: Punctuated::from_iter(value),
        }
    }
}

impl Extend<PathSegment> for Path {
    fn extend<T: IntoIterator<Item = PathSegment>>(&mut self, iter: T) {
        self.segments.extend(iter);
    }
}

impl std::ops::Deref for Path {
    type Target = Punctuated<PathSegment, Token![::]>;

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl std::ops::DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.segments
    }
}

impl IntoIterator for Path {
    type IntoIter = IntoIter<PathSegment>;
    type Item = PathSegment;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter()
    }
}

impl std::hash::Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(colon) = &self.leading_colon {
            colon.hash(state);
        }

        for (seg, sep) in self.pairs().map(|v| v.into_tuple()) {
            seg.hash(state);

            if let Some(colon) = sep {
                colon.hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use moxy_token::ToTokenStream;

    use super::*;
    use crate::Lifetime;

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
        assert!(!p.leading_colon.is_some());
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
        assert!(p.leading_colon.is_some());
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

    #[test]
    fn macro_construct() {
        let path = path!(hello::world);
        assert_eq!(path.segments.len(), 2)
    }
}
