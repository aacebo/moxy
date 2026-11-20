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

use moxy_token::TryIntoTokenStream;

/// Parse a source string into a typed AST node, returning `Result<T, ParseError>`.
///
/// The type can be given explicitly with `as T` or inferred from context.
///
/// A string lexes as tokens, with
/// [`TokenStream::from_string`](moxy_token::TokenStream::from_string), so a
/// leading `#!` line is tokens and no shebang. Parse the text of a source file
/// with [`File::from_source_file`](crate::File::from_source_file) instead.
///
/// # Example
/// ```ignore
/// use moxy::ast::*;
///
/// let token: Fn = parse!("fn").unwrap();
/// let token = parse!("fn" as Fn).unwrap();
/// ```
#[macro_export]
macro_rules! parse {
    ($src:tt $(as $ty:ty)? $(,)?) => {{
        $crate::__parse_owned $(::<$ty>)* (&$src)
    }};
}

/// Parse a rust source file into `moxy::ast::File`.
///
/// The file is read with [`File::from_source_file`](crate::File::from_source_file),
/// which skips a byte order mark and keeps a shebang line in `File::shebang`.
///
/// # Example
/// ```ignore
/// use moxy::ast::*;
///
/// let file: File = parse_file!("/path/to/file").unwrap();
/// ```
#[macro_export]
macro_rules! parse_file {
    ($path:tt $(,)?) => {{
        let path = ::std::path::Path::new(&$path);

        match ::std::fs::read_to_string(path) {
            Ok(source) => $crate::File::from_source_file(source),
            Err(error) => Err($crate::ParseError::new(
                $crate::__private::moxy_token::Span::call_site(),
                format!("could not read source file `{}`: {}", path.display(), error),
            )),
        }
    }};
}

/// Parse every source file that matches the glob patterns, relative to the
/// calling crate's manifest directory, into a `Vec<File>`.
///
/// Each file is read with [`File::from_source_file`](crate::File::from_source_file),
/// which skips a byte order mark and keeps a shebang line in `File::shebang`.
///
/// # Panics
///
/// Panics when a pattern is not valid or a file cannot be read or parsed.
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
            let file: $crate::File = $crate::File::from_source_file(source)
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
pub fn __parse_owned<T: Parse>(source: impl TryIntoTokenStream) -> Result<T, ParseError> {
    source.with_token_stream(|tokens| {
        let parser = Parser::from_tokens(tokens);
        let value = T::parse(&parser)?;

        if !parser.is_empty() {
            return Err(parser.error("unexpected trailing input"));
        }

        Ok(value)
    })?
}

/// Parse the text of a source file, lexed by
/// [`TokenStream::from_source_file`](moxy_token::TokenStream::from_source_file),
/// which skips a byte order mark and a shebang line.
///
/// This is public only so [`parse_file!`](crate::parse_file) and
/// [`parse_files!`](crate::parse_files) can call it from downstream crates.
#[doc(hidden)]
pub fn __parse_source_file<T: Parse>(text: String) -> Result<T, ParseError> {
    __parse_owned(&moxy_token::TokenStream::from_source_file(text)?)
}

// Parses a node and converts it into the enum that holds it. Choosing this
// function per variant and calling it once keeps a single return slot in the
// caller's stack frame instead of one per variant.
pub(crate) fn parse_into<T: Parse, U: From<T>>(parser: &Parser) -> Result<U, ParseError> {
    Ok(T::parse(parser)?.into())
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

    // Parses values while one starts here. A value that starts and then fails
    // to parse is the error, rather than input left after the list
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let mut items = Vec::new();

        while T::peek(parser.cursor()) {
            items.push(T::parse(parser)?);
        }

        Ok(items)
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while T::peek(cursor) {
            cursor = T::skip(cursor)?;
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
