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
    ($src:tt $(as $ty:ty)? $(, $key:ident = $value:expr)* $(,)?) => {{
        let mut config = $crate::ParseConfig::default();

        $(
            $crate::parse!(@option config, $key = $value);
        )*

        $crate::__parse_owned $(::<$ty>)* ($src.to_string(), config)
    }};

    (@option $config:ident, trace = $value:expr) => {
        $config.trace = $value;
    };
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
    ($($pattern:literal),+ $(as $ty:ty)? $(, $key:ident = $value:expr)* $(,)?) => {{
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

        $crate::parse!(tokens $(as $ty)? $(, $key = $value)*).expect("could not parse tokens")
    }};
}

/// Parse an owned source string without copying it again for fallback span storage.
///
/// This is public only so [`parse!`](crate::parse) can call it from downstream crates.
#[doc(hidden)]
pub fn __parse_owned<T: Parse>(source: String, config: ParseConfig) -> Result<T, ParseError> {
    let tokens = TokenStream::from_string(source)?;
    let parser = Parser::from_config(&tokens, config);
    let value = parser.parse()?;

    if !parser.is_empty() {
        return Err(parser.error("unexpected trailing input"));
    }

    Ok(value)
}

#[doc(hidden)]
pub mod __private {
    pub use moxy_token;
    pub use moxy_token::TokenStream;
    pub use moxy_token::source::glob;
}

pub trait Parse: Sized {
    fn peek(cursor: Cursor<'_>) -> bool;
    fn parse(parser: &Parser) -> Result<Self, ParseError>;
    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>>;
}

impl<T: Parse> Parse for Option<T> {
    fn peek(cursor: Cursor<'_>) -> bool {
        T::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<T>() {
            Ok(Some(parser.parse::<T>()?))
        } else {
            Ok(None)
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        // Option<T> returns Some when peek == false since its an optional
        // node
        if cursor.peek::<T>() {
            cursor.skip::<T>()
        } else {
            Some(cursor)
        }
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn peek(cursor: Cursor<'_>) -> bool {
        T::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(parser.parse_while::<T>())
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while let Some(next) = T::skip(cursor) {
            cursor = next;
        }

        Some(cursor)
    }
}

impl<T: Parse> Parse for Box<T> {
    fn peek(cursor: Cursor<'_>) -> bool {
        T::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self::new(parser.parse()?))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<T>()
    }
}
