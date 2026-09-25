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

use moxy_token::TokenStream;

/// Parse a source string into a typed AST node, returning `Result<T, ParseError>`.
///
/// The type can be given explicitly with `as T` or inferred from context.
///
/// # Example
/// ```
/// use moxy::ast::*;
///
/// let token: Fn = parse!("fn").unwrap();
/// let token = parse!("fn" as Fn).unwrap();
/// ```
#[macro_export]
macro_rules! parse {
    ($src:tt $(as $ty:ty)? $(,)?) => {{
        $crate::__parse_owned $(::<$ty>)* ($src.to_string())
    }};
}

/// Parse a rust source file into `moxy::ast::File`.
///
/// # Example
/// ```
/// use moxy::ast::*;
///
/// let file: File = parse_file!("/path/to/file").unwrap();
/// ```
#[macro_export]
macro_rules! parse_file {
    ($path:tt $(,)?) => {{
        let path = ::std::path::Path::new(&$path);

        match ::std::fs::read_to_string(path) {
            Ok(source) => $crate::__parse_owned(source),
            Err(error) => Err($crate::ParseError::new(
                $crate::__private::moxy_token::Span::call_site(),
                format!("could not read source file `{}`: {}", path.display(), error),
            )),
        }
    }};
}

/// Parse source file(s) into a typed AST node, returning `Result<T, ParseError>`.
///
/// The type can be given explicitly with `as T` or inferred from context.
///
/// # Example
/// ```ignore
/// use moxy::ast::*;
///
/// let files = parse_files!("src/**/*.rs");
/// ```
#[macro_export]
macro_rules! parse_files {
    ($($pattern:literal),+ $(,)?) => {{
        let mut paths = vec![];

        $(
            paths.extend($crate::__private::glob(
                std::env!("CARGO_MANIFEST_DIR"),
                $pattern,
            ).expect(&format!("glob pattern `{}` is not valid", $pattern)));
        )*

        let mut files = vec![];

        for path in paths {
            let source = ::std::fs::read_to_string(&path)
                .expect(&format!("file `{}` not found", path.display()));
            let file: $crate::File = $crate::__parse_owned(source)
                .expect("expected valid rust file");
            files.push(file);
        }

        files
    }};

}

/// Parse an owned source string without copying it again for fallback span storage.
///
/// This is public only so [`parse!`](crate::parse) can call it from downstream crates.
#[doc(hidden)]
pub fn __parse_owned<T: Parse>(source: String) -> Result<T, ParseError> {
    let tokens = TokenStream::from_string(source)?;
    let parser = Parser::from_tokens(&tokens);
    let value = T::parse(&parser)?;

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

/// A trait used while working with Moxy AST syntax.
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
        if T::peek(parser.cursor()) {
            Ok(Some(T::parse(parser)?))
        } else {
            Ok(None)
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        // Option<T> returns Some when peek == false since its an optional
        // node
        if T::peek(cursor) { T::skip(cursor) } else { Some(cursor) }
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
        Ok(Self::new(T::parse(parser)?))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        T::skip(cursor)
    }
}
