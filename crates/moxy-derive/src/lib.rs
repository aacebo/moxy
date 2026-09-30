//! # Moxy derive
//!
//! Derive support for `moxy::token::ToTokens`.
//! It also provides `#[moxy::derive(Name)]` for authoring custom derive macros
//! from a typed moxy AST input.
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
//! On nightly Rust, add `#[moxy(debug)]` alongside the template attribute to
//! emit compiler notes with the parsed input declaration and generated
//! `ToTokens` implementation. The option is intended for inspecting derive
//! output during development and does not change the generated implementation.
//! Stable Rust does not emit these debug notes.
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
mod derive;
mod function;
mod meta;
mod tokens;

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
/// - `#[moxy(debug)]` is optional. On nightly Rust, it emits compiler notes
///   containing the parsed input declaration and generated `ToTokens`
///   implementation, which is useful when inspecting an expansion during
///   development. Stable Rust does not emit these notes.
///
/// A missing, repeated, or malformed template attribute produces a
/// span-targeted compiler error.
#[proc_macro_derive(ToTokens, attributes(moxy))]
pub fn derive_tokens(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens::expand(tokens.into()).into()
}

/// Derives [`moxy::ast::FromMeta`] for a struct or enum.
///
/// Parse a matching attribute with [`moxy::ast::Attributed::parse_meta`]. Named
/// struct fields map to meta-item names and are converted recursively through
/// `FromMeta`; nested values must therefore implement `FromMeta` as well.
///
/// Field behavior can be customized with `#[meta(...)]`:
///
/// - `rename = "..."` changes the accepted meta-item name.
/// - `default` or `default = expr` supplies a value when the item is absent.
/// - `message = "..."` customizes missing-item and duplicate-item errors.
///
/// Newtype enum variants accept their inner value directly, and named enum
/// variants parse their fields from a nested attribute list. Generic type
/// parameters receive a `FromMeta` bound in the generated implementation.
///
/// # Example
///
/// ```ignore
/// use moxy::ast::Attributed;
///
/// #[derive(moxy::FromMeta)]
/// struct BuildArgs {
///     #[meta(default)]
///     rename: Option<String>,
///
///     #[meta(rename = "default", default)]
///     is_default: bool,
/// }
///
/// let field: moxy::ast::Field = moxy::parse!(#[build(rename = "set_name", default)] name: String).unwrap();
/// let args: BuildArgs = field.parse_meta("build")?.unwrap();
/// assert_eq!(args.rename.as_deref(), Some("set_name"));
/// assert!(args.is_default);
/// ```
#[proc_macro_derive(FromMeta, attributes(meta))]
pub fn derive_meta(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    meta::expand(tokens.into()).into()
}

/// Turns a public token-to-token function into a function-like procedural macro.
///
/// The annotated function must accept one [`moxy_token::TokenStream`] argument and return
/// `Result<TokenStream, ParseError>`. By default, the generated macro has the
/// same name as the function. Set `name` to export it under another identifier.
///
/// # Options
///
/// - `name = "…"` or `name = identifier` sets the exported macro name.
/// - `debug` emits the generated wrapper as a compiler note on nightly Rust.
///   Stable Rust does not emit this note.
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
/// - `debug` emits the generated wrapper as a compiler note on nightly Rust.
///   Stable Rust does not emit this note.
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

/// Turns a public parseable-AST-to-token function into a custom derive macro.
///
/// The attribute argument is the derive name exposed to downstream crates.
/// The annotated function accepts one moxy-parsed value and returns
/// `Result<TokenStream, ParseError>`. Its parameter can be any moxy type that
/// implements `Parse`, which determines the syntax accepted by the derive.
/// For example, `ItemStruct` accepts structs, `ItemEnum` accepts enums, and
/// `Declaration` accepts a general annotated declaration.
///
/// # Example
///
/// ```ignore
/// use moxy::ast::{ItemStruct, ParseError};
/// use moxy::token::TokenStream;
///
/// #[moxy::derive(Builder)]
/// pub fn builder(item: ItemStruct) -> Result<TokenStream, ParseError> {
///     Ok(moxy::template! {
///         impl {{ item.ident }} {
///             pub fn builder() -> Self { todo!() }
///         }
///     })
/// }
///
/// #[derive(Builder)]
/// struct Config;
/// ```
///
/// The annotated function must be public, take exactly one parameter, and
/// return a `Result` whose error can produce a compile error.
#[proc_macro_attribute]
pub fn derive(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::expand(attr.into(), item.into()).into()
}
