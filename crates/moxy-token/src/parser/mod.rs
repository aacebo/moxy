mod error;
mod stream;

#[doc(inline)]
pub use error::*;
#[doc(inline)]
pub use stream::*;

/// Parse a source string into a typed AST node, returning `Result<T, ParseError>`.
///
/// The type can be given explicitly with `as T` or inferred from context.
///
/// # Example
/// ```ignore
/// let item: Item = moxy_token::parse!("fn foo() {}");
/// let item = moxy_token::parse!("fn foo() {}" as Item);
/// ```
#[macro_export]
macro_rules! parse {
    ($src:tt as $ty:ty) => {{
        use ::std::str::FromStr;
        $crate::TokenStream::from_str(&$src.to_string())
            .map_err($crate::parser::ParseError::from)
            .and_then(|ts| <$ty as $crate::Parse>::parse(&mut ts.parse()))
    }};
    ($src:tt) => {{
        use ::std::str::FromStr;
        $crate::TokenStream::from_str(&$src.to_string())
            .map_err($crate::parser::ParseError::from)
            .and_then(|ts| $crate::Parse::parse(&mut ts.parse()))
    }};
}

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError>;
}

pub trait Peek: Sized {
    fn peek(stream: &mut ParseStream) -> bool;
}

impl<T: Parse> Peek for T {
    fn peek(stream: &mut ParseStream) -> bool {
        Self::parse(stream).is_ok()
    }
}

impl<T: Parse> Parse for Option<T> {
    #[inline]
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let mut fork = stream.fork();

        match T::parse(&mut fork) {
            Ok(v) => {
                stream.seek(&fork);
                Ok(Some(v))
            }
            Err(_) => Ok(None),
        }
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(stream.parse_while::<T>())
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self::new(T::parse(stream)?))
    }
}
