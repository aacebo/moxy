//! # Moxy
//!
//! Moxy is a modular toolkit for reading, transforming, generating, and
//! formatting Rust syntax in procedural macros and related tooling.
//!
//! ## Getting started
//!
//! Enable only the layers an application needs:
//!
//! ```text
//! cargo add moxy --features template,fmt
//! ```
//!
//! Templates produce tokens, parsers turn them into typed syntax trees, and the
//! formatter renders those trees as Rust source:
//!
//! ```ignore
//! use moxy::ast::Item;
//!
//! let tokens = moxy::template! { pub struct Widget; };
//! let item: Item = moxy::parse!(tokens)?;
//! assert_eq!(moxy::fmt!(&item)?, "pub struct Widget;");
//! ```
//!
//! ## Feature flags
//!
//! `token` and `ast` are enabled by default. Start with `--no-default-features`
//! when a procedural macro needs only a narrow layer.
//!
//! | Feature | Default | Provides |
//! | --- | :---: | --- |
//! | `token` | yes | Token streams, spans, lexing, and token construction |
//! | `ast` | yes | Typed Rust syntax trees and parsing; implies `token` |
//! | `template` | no | `template!` and `paste!`; implies `token` |
//! | `fmt` | no | Formatting through `fmt!`; implies `ast` |
//! | `diagnostic` | no | Span-aware diagnostics and `compile_error!` fallback |
//! | `build` | no | Cargo build-script directives and rustc version helpers |
//! | `macros` | no | `#[derive(ToTokens)]`, `#[derive(FromMeta)]`, and macro-authoring attributes |
//! | `derives` | no | Standard trait derives for supported AST and template types |
//! | `serde` | no | Serialization for supported AST, token, and formatter types |
//! | `proc-macro2` | no | Conversion between Moxy and `proc_macro2` tokens |
//! | `full` | no | Every feature above |
//!
//! ## Tokens
//!
//! Use the `token` feature to construct or lex Rust tokens. `Token!` names the
//! concrete type associated with a punctuation mark or keyword.
//!
//! ```ignore
//! use moxy::{Token, token::{ident, TokenStream}};
//!
//! let name = ident!(Generated, "_", Item);
//! let comma: Token![,] = Default::default();
//! let source: TokenStream = "fn generated() {}".parse()?;
//! assert_eq!(name.to_string(), "Generated_Item");
//! assert_eq!(comma.to_string(), ",");
//! ```
//!
//! ## Parsing and inspecting syntax
//!
//! The `ast` feature parses at the grammar level required by the caller. The
//! category enums expose predicates and accessors for their concrete forms.
//!
//! ```ignore
//! use moxy::ast::{Expr, Item, Type};
//!
//! let item: Item = moxy::parse!("pub struct User { id: u64 }")?;
//! let ty: Type = moxy::parse!("Option<Result<T, E>>")?;
//! let expr: Expr = moxy::parse!("items.next()?")?;
//!
//! assert!(item.is_struct());
//! assert!(ty.is_path());
//! assert!(expr.is_try());
//! ```
//!
//! ## Templates
//!
//! The `template` feature creates token streams from Rust-shaped templates.
//! Interpolate values with `{{ expr }}` and use `@for`, `@if`, and `@match` for
//! runtime control flow. `@for pattern in iter` and `@if condition` accept
//! Rust patterns and conditions without header parentheses; `@match (expr)`
//! accepts Rust patterns and guards. An interpolation can also be a statement
//! block whose tail expression is emitted.
//!
//! ```ignore
//! let fields = ["id", "name"];
//! let tokens = moxy::template! {
//!     struct User {
//!         @for field in fields { {{ field }}: String, }
//!     }
//! };
//! assert!(tokens.to_string().contains("struct User"));
//! ```
//!
//! ```ignore
//! let value = Some("enabled");
//! let tokens = moxy::template! {
//!     @if let Some(value) = value { const ENABLED: &str = {{ value }}; }
//!     {{ let name = "generated"; name }}
//! };
//! ```
//!
//! `paste!` creates an identifier at macro expansion time:
//!
//! ```ignore
//! moxy::paste! { fn {{ get_ value }}() -> u32 { 7 } }
//! assert_eq!(get_value(), 7);
//! ```
//!
//! ## Formatting
//!
//! Enable `fmt` to render parsed syntax with a configurable line width,
//! indentation, and newline style.
//!
//! ```ignore
//! use moxy::ast::Item;
//! use moxy::fmt::{FmtConfig, Indent};
//!
//! let item: Item = moxy::parse!("struct User { id: u64, name: String }")?;
//! let config = FmtConfig::default().with_indent(Indent::space(2));
//! let source = moxy::fmt!(&item, config)?;
//! assert_eq!(source, "struct User {\n  id: u64,\n  name: String,\n}");
//! ```
//!
//! ## Diagnostics
//!
//! The `diagnostic` feature builds span-aware errors, warnings, notes, and help
//! messages. Calling `emit` returns compiler tokens suitable for a proc-macro
//! expansion.
//!
//! ```ignore
//! let tokens = moxy::error!(
//!     "missing template",
//!     [moxy::help!("add #[moxy(template { ... })]")],
//! ).emit();
//! ```
//!
//! ## Build scripts
//!
//! Enable `build` as a build dependency to inspect rustc and emit typed Cargo
//! directives from `build.rs`.
//!
//! ```ignore
//! let mut rustc = moxy::build::rustc::Config::read()?;
//! rustc.min_version("1.85.0").rerun_if_changed("build.rs").emit();
//! ```
//!
//! ## Derive
//!
//! ### ToTokens
//!
//! With `macros`, `#[derive(moxy::ToTokens)]` implements
//! `moxy::token::ToTokens` from an inline template:
//!
//! ```ignore
//! #[derive(moxy::ToTokens)]
//! #[moxy(template { const VALUE: &str = {{ self.value }}; })]
//! struct Generated { value: String }
//! ```
//!
//! On nightly Rust, add `#[moxy(debug)]` to emit compiler notes containing the
//! parsed input declaration and generated `ToTokens` implementation. This is
//! intended for inspecting a derive expansion during development; on stable
//! Rust, the debug notes are not emitted.
//!
//! ### FromMeta
//!
//! `#[derive(moxy::FromMeta)]` converts a structured attribute into a named
//! struct or enum. Parse a matching attribute with
//! `moxy::ast::Attributed::parse_meta`. The derive adds `FromMeta` bounds for
//! generic type parameters.
//!
//! ```ignore
//! use moxy::ast::Attributed;
//!
//! #[derive(moxy::FromMeta)]
//! struct BuildArgs {
//!     // Use `None` when `rename` is omitted.
//!     #[meta(default)]
//!     rename: Option<String>,
//!
//!     // Read the `default` meta item as `is_default`.
//!     #[meta(rename = "default", default)]
//!     is_default: bool,
//!
//!     // Customize the error for this required value.
//!     #[meta(message = "missing build name")]
//!     name: String,
//! }
//!
//! let field: moxy::ast::Field =
//!     moxy::parse!(#[build(rename = "set_name", default, name = "Build")] name: String)?;
//! let args: BuildArgs = field.parse_meta("build")?.unwrap();
//! ```
//!
//! ### Custom derives
//!
//! `#[moxy::derive(Name)]` turns a public function that accepts one parseable
//! moxy AST value into a derive macro:
//!
//! ```ignore
//! use moxy::ast::{ItemStruct, ParseError};
//! use moxy::token::TokenStream;
//!
//! #[moxy::derive(Builder)]
//! pub fn builder(item: ItemStruct) -> Result<TokenStream, ParseError> {
//!     Ok(moxy::template! {
//!         impl {{ item.ident }} { pub fn builder() -> Self { todo!() } }
//!     })
//! }
//! ```
//!
//! ### Function
//!
//! `#[moxy::function]` turns a public function from one token stream to another
//! into a function-like procedural macro. The function returns
//! `Result<TokenStream, ParseError>`:
//!
//! ```ignore
//! use moxy::ast::ParseError;
//! use moxy::token::TokenStream;
//!
//! #[moxy::function(name = "answer")]
//! pub fn expand(_tokens: TokenStream) -> Result<TokenStream, ParseError> {
//!     Ok(moxy::template! { 42 })
//! }
//!
//! answer!();
//! ```
//!
//! On nightly Rust, add `debug` (for example, `#[moxy::function(debug)]`) to
//! emit the generated wrapper as a compiler note. Stable Rust does not emit
//! this debug note.
//!
//! ### Attribute
//!
//! `#[moxy::attribute]` turns a public function taking the attribute arguments
//! and annotated item into an attribute procedural macro:
//!
//! ```ignore
//! use moxy::ast::ParseError;
//! use moxy::token::TokenStream;
//!
//! #[moxy::attribute(name = "passthrough")]
//! pub fn expand(_meta: TokenStream, item: TokenStream) -> Result<TokenStream, ParseError> {
//!     Ok(item)
//! }
//!
//! #[passthrough]
//! fn generated() {}
//! ```
//!
//! On nightly Rust, add `debug` (for example, `#[moxy::attribute(debug)]`) to
//! emit the generated wrapper as a compiler note. Stable Rust does not emit
//! this debug note.
//!
//! ## Integrations
//!
//! `serde` serializes supported token, AST, and formatting types. The
//! `proc-macro2` feature converts token streams for interoperability with the
//! wider procedural-macro ecosystem.

/// Rust syntax tree types, parsers, and traversal APIs from [`moxy_ast`].
#[cfg(feature = "ast")]
#[doc(inline)]
pub use moxy_ast as ast;

/// Build-script helpers for emitting Cargo instructions and inspecting `rustc`.
#[cfg(feature = "build")]
#[doc(inline)]
pub use moxy_build as build;

/// Derives [`ToTokens`] and [`FromMeta`], and provides the [`function`],
/// [`attribute`], [`derive`], and [`apply`] procedural-macro attributes.
#[cfg(feature = "macros")]
#[doc(inline)]
pub use moxy_macros::*;

/// Types for building span-aware compiler diagnostics.
#[cfg(feature = "diagnostic")]
#[doc(inline)]
pub use moxy_diagnostic as diagnostic;

#[cfg(feature = "diagnostic")]
#[doc(inline)]
pub use moxy_diagnostic::{error, help, note, warn};

/// Pretty-printing configuration, formatting traits, and formatting errors.
#[cfg(feature = "fmt")]
#[doc(inline)]
pub use moxy_fmt as fmt;

#[cfg(feature = "fmt")]
#[doc(inline)]
pub use moxy_fmt::fmt;

/// Macros for constructing token streams from runtime templates and for joining
/// identifier fragments during macro expansion.
#[cfg(feature = "template")]
#[doc(inline)]
pub use moxy_template as template;

#[cfg(feature = "template")]
#[doc(inline)]
pub use moxy_template::*;

/// Token-stream types, lexer support, source spans, and token-construction APIs.
#[cfg(feature = "token")]
#[doc(inline)]
pub use moxy_token as token;

#[cfg(all(feature = "token", not(feature = "ast")))]
#[doc(inline)]
pub use moxy_token::Token;

#[cfg(feature = "ast")]
#[doc(inline)]
pub use moxy_ast::{Token, parse, parse_file, parse_files};
