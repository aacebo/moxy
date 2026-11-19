/// Conversions for the compiler's `proc_macro` token types.
pub mod proc_macro;

/// Conversions for `proc_macro2` token types; requires the `proc-macro2` feature.
#[cfg(feature = "proc-macro2")]
pub mod proc_macro2;

// The stream an infallible conversion returns, which panics on a stream whose
// groups nest past `MAX_DEPTH`. Every infallible conversion has a fallible
// route: `try_into_token_stream` on a stream that holds the tokens
fn expect_converted(converted: Result<crate::TokenStream, crate::LexError>) -> crate::TokenStream {
    converted.unwrap_or_else(|error| {
        panic!("{error}; convert a token stream that holds the tokens with `try_into_token_stream` to get this as an error")
    })
}
