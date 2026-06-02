#![cfg_attr(
    nightly,
    feature(
        extend_one,
        proc_macro_diagnostic,
        proc_macro_span,
        // proc_macro_totokens,
        proc_macro_def_site,
    )
)]

extern crate proc_macro;

pub mod bridge;
pub mod parse;
pub mod source;
pub mod span;
pub mod token;

pub use parse::Parse;
pub use span::Span;
pub use token::{Token, TokenStream, TokenTree};

/// Parse a token stream into a typed AST node, emitting a compile error on failure.
#[macro_export]
macro_rules! parse {
    ($tokens:ident as $ty:ty) => {{
        let mut stream = $tokens.parse();

        match <$ty as $crate::Parse>::parse(&mut stream) {
            Ok(v) => v,
            Err(e) => {
                return e.to_compile_error().into_iter().collect();
            }
        }
    }};
}
