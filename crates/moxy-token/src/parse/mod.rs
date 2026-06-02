mod error;
mod stream;

pub use error::*;
pub use stream::*;

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError>;
}

pub trait Peek: Sized {
    fn peek(stream: &mut ParseStream) -> Option<Self>;
}

impl<T: Parse> Peek for T {
    fn peek(stream: &mut ParseStream) -> Option<Self> {
        Self::parse(stream).ok()
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(stream.parse_while::<T>())
    }
}

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
    ($src:literal as $ty:ty) => {{
        use ::std::str::FromStr;
        $crate::TokenStream::from_str($src)
            .map_err($crate::parse::ParseError::from)
            .and_then(|ts| <$ty as $crate::Parse>::parse(&mut ts.parse()))
    }};
    ($src:expr) => {{
        use ::std::str::FromStr;
        $crate::TokenStream::from_str(::std::convert::AsRef::<str>::as_ref(&$src))
            .map_err($crate::parse::ParseError::from)
            .and_then(|ts| $crate::Parse::parse(&mut ts.parse()))
    }};
}
