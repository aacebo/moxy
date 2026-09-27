//! # Moxy derive
//!
//! Derive support for `moxy::token::ToTokens`.
//!
//! ## Syntax
//!
//! `#[derive(ToTokens)]` requires `#[moxy(template { ... })]`. Its template is
//! expanded by `moxy::template!`, and `self` is available in the generated
//! `to_tokens` method.
//!
//! ```ignore
//! #[derive(moxy::ToTokens)]
//! #[moxy(template { struct {{ self.name }}; })]
//! struct Generated { name: String }
//! ```
//!
//! ## Debugging expansions
//!
//! Add `#[moxy(debug)]` alongside the template attribute to emit compiler notes
//! with the parsed input declaration and the generated `ToTokens`
//! implementation. The option is intended for inspecting derive output during
//! development and does not change the generated implementation.
//!
//! ```ignore
//! #[derive(moxy::ToTokens)]
//! #[moxy(template { struct {{ self.name }}; })]
//! #[moxy(debug)]
//! struct Generated { name: String }
//! ```

extern crate self as moxy;

mod ast {
    pub use moxy_ast::*;
}

mod token {
    pub use moxy_token::*;
}

mod attribute;
mod function;
mod to_tokens;

use moxy_ast::parse;
use moxy_fmt::fmt;
use moxy_template::template;

/// Derives [`moxy::token::ToTokens`] from a token template.
///
/// The derive generates an implementation of `ToTokens` for the annotated
/// type. Its required `#[moxy(template { ... })]` attribute contains a
/// [`moxy::template!`] body; the generated method makes `self` available to
/// template interpolations.
///
/// # Example
///
/// ```ignore
/// use moxy::token::ToTokenStream;
///
/// #[derive(moxy::ToTokens)]
/// #[moxy(template {
///     pub const GENERATED: &str = {{ self.value }};
/// })]
/// struct Model {
///     value: String,
/// }
///
/// let tokens = Model { value: "ready".into() }.to_token_stream();
/// assert_eq!(tokens.to_string(), "pub const GENERATED : & str = \"ready\" ;");
/// ```
///
/// # Attributes
///
/// - `#[moxy(template { ... })]` is required exactly once. Its value must be a
///   braced Rust token block accepted by `moxy::template!`.
/// - `#[moxy(debug)]` is optional. It emits compiler notes containing the parsed
///   input declaration and the generated `ToTokens` implementation, which is
///   useful when inspecting an expansion during development.
///
/// A missing, repeated, or malformed template attribute produces a
/// span-targeted compiler error.
#[proc_macro_derive(ToTokens, attributes(moxy))]
pub fn derive_to_tokens(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    to_tokens::expand(tokens.into()).into()
}

/// Turns a public token-to-token function into a function-like procedural macro.
///
/// The annotated function must accept one [`moxy_token::TokenStream`] argument and return
/// `Result<TokenStream, ParseError>`. By default, the generated macro has the
/// same name as the function. Set `name` to export it under another identifier.
///
/// # Example
///
/// ```ignore
/// use moxy::ast::ParseError;
/// use moxy::token::TokenStream;
///
/// #[moxy::function(name = "hello")]
/// pub fn expand(tokens: TokenStream) -> Result<TokenStream, ParseError> {
///     Ok(moxy::template! { println!("hello"); })
/// }
///
/// hello!();
/// ```
#[proc_macro_attribute]
pub fn function(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    function::expand(attr.into(), item.into()).into()
}

/// Turns a public token-to-token function into an attribute procedural macro.
///
/// The annotated function must accept two [`moxy_token::TokenStream`] arguments:
/// the attribute arguments first and the annotated item second. It must return
/// `Result<TokenStream, ParseError>`. By default, the generated attribute has
/// the same name as the function. Set `name` to export it under another
/// identifier.
///
/// # Options
///
/// - `name = "…"` or `name = identifier` sets the exported attribute name.
/// - `debug` emits the generated wrapper as a compiler note.
///
/// # Example
///
/// ```ignore
/// use moxy::ast::ParseError;
/// use moxy::token::TokenStream;
///
/// #[moxy::attribute(name = "hello")]
/// pub fn expand(meta: TokenStream, tokens: TokenStream) -> Result<TokenStream, ParseError> {
///     Ok(moxy::template! { println!("hello"); })
/// }
///
/// #[hello]
/// fn main() {}
/// ```
#[proc_macro_attribute]
pub fn attribute(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attribute::expand(attr.into(), item.into()).into()
}
