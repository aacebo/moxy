mod config;
mod cursor;
mod error;
mod parser;
mod tokens;

#[doc(inline)]
pub use error::*;

#[doc(inline)]
pub use cursor::*;

#[doc(inline)]
pub use parser::*;

#[doc(inline)]
pub use config::*;

use moxy_token::TokenStream;

/// Parse a source string into a typed AST node, returning `Result<T, ParseError>`.
///
/// The type can be given explicitly with `as T` or inferred from context.
///
/// # Example
/// ```
/// use moxy_ast::*;
///
/// let token: Fn = parse!("fn").unwrap();
/// let token = parse!("fn" as Fn).unwrap();
/// ```
#[macro_export]
macro_rules! parse {
    ($src:tt as $ty:ty) => {{ $crate::__parse_owned::<$ty>($src.to_string()) }};
    ($src:tt) => {{ $crate::__parse_owned($src.to_string()) }};
}

/// Parse source file(s) into a typed AST node, returning `Result<T, ParseError>`.
///
/// The type can be given explicitly with `as T` or inferred from context.
///
/// # Example
/// ```ignore
/// use moxy_ast::*;
///
/// let tokens = parse_files!("src/**/*.rs");
/// ```
#[macro_export]
macro_rules! parse_files {
    ($($pattern:literal),+ $(,)? as $ty:ty) => {{
        let mut tokens = $crate::__private::TokenStream::new();

        $(
            let paths = $crate::__private::glob(
                std::env!("CARGO_MANIFEST_DIR"),
                $pattern,
            ).expect(&format!("glob pattern `{}` is not valid", $pattern));

            for path in paths {
                let src = ::std::fs::read_to_string(&path).expect(&format!("file `{}` not found", path.display()));
                let parser: $crate::__private::TokenStream = src.parse().expect("invalid source file");
                tokens.extend(parser);
            }
        )*

        $crate::parse!(tokens as $ty).expect("could not parse tokens")
    }};
    ($($pattern:literal),+ $(,)?) => {{
        let mut tokens = $crate::__private::TokenStream::new();

        $(
            let paths = $crate::__private::glob(
                std::env!("CARGO_MANIFEST_DIR"),
                $pattern,
            ).expect(&format!("glob pattern `{}` is not valid", $pattern));

            for path in paths {
                let src = ::std::fs::read_to_string(&path).expect(&format!("file `{}` not found", path.display()));
                let parser: $crate::__private::TokenStream = src.parse().expect("invalid source file");
                tokens.extend(parser);
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
    let tokens = TokenStream::from_string(source)?;
    let parser = Parser::from_tokens(&tokens);
    let value = parser.parse()?;

    if !parser.is_empty() {
        return Err(parser.error("unexpected trailing input"));
    }

    Ok(value)
}

#[doc(hidden)]
pub mod __private {
    pub use moxy_token::TokenStream;
    pub use moxy_token::source::glob;
}

pub trait Parse: Sized {
    fn parse(parser: &Parser) -> Result<Self, ParseError>;
}

pub trait Peek: Sized {
    fn peek(parser: &Parser) -> bool;
}

impl<T: Parse> Parse for Option<T> {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(parser.parse_if())
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(parser.parse_while::<T>())
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self::new(parser.parse()?))
    }
}
