//! # Moxy templates
//!
//! Procedural macros for runtime token templates and identifier pasting.
//!
//! ## Template syntax
//!
//! [`template!`] returns a `moxy::token::TokenStream`. `{{ expr }}` emits a
//! value implementing `moxy::token::ToTokens`; `@for`, `@if`/`@else`, and
//! `@match` evaluate ordinary Rust control flow while building the stream.
//!
//! ```ignore
//! let fields = ["id", "name"];
//! let tokens = moxy::template! {
//!     struct User { @for (field in fields) { {{ field }}: String, } }
//! };
//! ```
//!
//! [`paste!`] instead joins tokens inside `{{ ... }}` into an identifier in
//! declaration position, such as `fn {{ get_ value }}() {}`.

mod ast;

use ast::{Paste, Template};
use moxy_ast::{Parse, Parser};
use moxy_token::{ToTokens, TokenStream};

/// Build a `moxy::token::TokenStream` at runtime from a template, in the style
/// of `quote!`.
///
/// The macro expands to a block that constructs and returns a `TokenStream`.
/// Interpolations and control flow are evaluated against the surrounding scope:
///
/// - `{{ expr }}` splices the runtime value of `expr` (via `moxy::token::ToTokens`),
///   preserving its source spans.
/// - `@for (binding in iter) { … }`, `@if (cond) { … } @else { … }`, and
///   `@match (expr) { pat => { … }, … }` run as real control flow, appending to
///   the output as they execute.
///
/// # Example
///
/// ```ignore
/// let items = vec!["a", "b", "c"];
/// let tokens = template! {
///     @for (item in items) {
///         {{ item }}
///     }
/// };
/// assert_eq!(tokens.to_string(), "a b c");
/// ```
///
/// A malformed template produces a span-targeted compile error.
#[proc_macro]
pub fn template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ts = TokenStream::new();
    input.to_tokens(&mut ts);

    let parser = Parser::from_tokens(&ts);
    let expanded = match Template::parse(&parser) {
        Ok(tmpl) => tmpl.expand(),
        Err(e) => e.to_compile_error(),
    };

    let mut out = proc_macro::TokenStream::new();
    expanded.to_tokens(&mut out);
    out
}

/// Concatenates the tokens inside each `{{ ... }}` marker into a single
/// identifier at compile time, passing all other tokens through unchanged.
///
/// Use it to mint identifier *names* in declaration position — something the
/// runtime `moxy::token::ident!` macro cannot do. Segments are taken by token
/// text (not evaluated), then validated as a Rust identifier.
///
/// # Example
///
/// ```ignore
/// paste! {
///     fn {{ get_ value }}() -> u32 { 7 }   // expands to: fn get_value() -> u32 { 7 }
/// }
/// assert_eq!(get_value(), 7);
/// ```
///
/// A `{{ ... }}` that does not concatenate to a valid identifier produces a
/// span-targeted compile error.
#[proc_macro]
pub fn paste(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ts = TokenStream::new();
    input.to_tokens(&mut ts);

    let parser = Parser::from_tokens(&ts);
    let expanded = match Paste::parse(&parser) {
        Ok(p) => p.expand(),
        Err(e) => e.to_compile_error(),
    };

    let mut out = proc_macro::TokenStream::new();
    expanded.to_tokens(&mut out);
    out
}
