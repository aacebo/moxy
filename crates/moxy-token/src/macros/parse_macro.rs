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
            .map_err($crate::parser::ParseError::from)
            .and_then(|ts| <$ty as $crate::Parse>::parse(&mut ts.parse()))
    }};
    ($src:expr) => {{
        use ::std::str::FromStr;
        $crate::TokenStream::from_str(&$src.to_string())
            .map_err($crate::parser::ParseError::from)
            .and_then(|ts| $crate::Parse::parse(&mut ts.parse()))
    }};
}
