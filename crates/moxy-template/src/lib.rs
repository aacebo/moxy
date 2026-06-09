mod ast;

use ast::{Paste, Template};
use moxy_token::{Parse, ToTokens, TokenStream};

/// Build a [`moxy_token::TokenStream`] at runtime from a template, in the style
/// of `quote!`.
///
/// The macro expands to a block that constructs and returns a `TokenStream`.
/// Interpolations and control flow are evaluated against the surrounding scope:
///
/// - `{{ expr }}` splices the runtime value of `expr` (via [`moxy_token::ToTokens`]),
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

    let expanded = match Template::parse(&mut ts.parse()) {
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
/// runtime [`moxy_token::ident!`] macro cannot do. Segments are taken by token
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

    let expanded = match Paste::parse(&mut ts.parse()) {
        Ok(p) => p.expand(),
        Err(e) => e.to_compile_error(),
    };

    let mut out = proc_macro::TokenStream::new();
    expanded.to_tokens(&mut out);
    out
}
