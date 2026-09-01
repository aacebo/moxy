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
/// ```
/// use moxy_token::*;
///
/// let token: Fn = parse!("fn").unwrap();
/// let token = parse!("fn" as Fn).unwrap();
/// ```
#[macro_export]
macro_rules! parse {
    ($src:tt as $ty:ty) => {{ $crate::parser::__parse_owned::<$ty>($src.to_string()) }};
    ($src:tt) => {{ $crate::parser::__parse_owned($src.to_string()) }};
}

/// Parse source file(s) into a typed AST node, returning `Result<T, ParseError>`.
///
/// The type can be given explicitly with `as T` or inferred from context.
///
/// # Example
/// ```ignore
/// use moxy_token::*;
///
/// let tokens = parse_files!("src/**/*.rs");
/// ```
#[macro_export]
macro_rules! parse_files {
    ($($pattern:literal),+ $(,)? as $ty:ty) => {{
        let mut tokens = $crate::TokenStream::new();

        $(
            let paths = $crate::source::glob(
                std::env!("CARGO_MANIFEST_DIR"),
                $pattern,
            ).expect(&format!("glob pattern `{}` is not valid", $pattern));

            println!("{}", paths.len());

            for path in paths {
                println!("{}", path.display());
                let src = ::std::fs::read_to_string(&path).expect(&format!("file `{}` not found", path.display()));
                let stream: $crate::TokenStream = src.parse().expect("invalid source file");
                tokens.extend(stream);
            }
        )*

        $crate::parse!(tokens as $ty).expect("could not parse tokens")
    }};
    ($($pattern:literal),+ $(,)?) => {{
        let mut tokens = $crate::TokenStream::new();

        $(
            let paths = $crate::source::glob(
                std::env!("CARGO_MANIFEST_DIR"),
                $pattern,
            ).expect(&format!("glob pattern `{}` is not valid", $pattern));

            println!("{}", paths.len());

            for path in paths {
                println!("{}", path.display());
                let src = ::std::fs::read_to_string(&path).expect(&format!("file `{}` not found", path.display()));
                let stream: $crate::TokenStream = src.parse().expect("invalid source file");
                tokens.extend(stream);
            }
        )*

        tokens
    }};
}

/// Parse an owned source string without copying it again for fallback span storage.
///
/// This is public only so [`parse!`](crate::parse) can call it from downstream crates.
#[doc(hidden)]
pub fn __parse_owned<T: Parse>(source: String) -> Result<T, ParseError> {
    crate::TokenStream::from_string(source).and_then(|tokens| tokens.parse().parse())
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
        Ok(stream.parse_if())
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(stream.parse_while::<T>())
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self::new(stream.parse()?))
    }
}
