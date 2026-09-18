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
//! | `derive` | no | `#[derive(ToTokens)]`; implies its supporting features |
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
//! Pass `trace = true` to `parse!` to print parser activity while developing a
//! grammar or debugging an input.
//!
//! ## Templates
//!
//! The `template` feature creates token streams from Rust-shaped templates.
//! Interpolate values with `{{ expr }}` and use `@for`, `@if`, and `@match` for
//! runtime control flow.
//!
//! ```ignore
//! let fields = ["id", "name"];
//! let tokens = moxy::template! {
//!     struct User {
//!         @for (field in fields) { {{ field }}: String, }
//!     }
//! };
//! assert!(tokens.to_string().contains("struct User"));
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
//! ## Deriving token emission
//!
//! With `derive`, `#[derive(moxy::ToTokens)]` implements
//! `moxy::token::ToTokens` from an inline template:
//!
//! ```ignore
//! #[derive(moxy::ToTokens)]
//! #[moxy(template { const VALUE: &str = {{ self.value }}; })]
//! struct Generated { value: String }
//! ```
//!
//! Add `#[moxy(debug)]` to emit compiler notes containing the parsed input
//! declaration and the generated `ToTokens` implementation. This is intended
//! for inspecting a derive expansion during development.
//!
//! ## Integrations
//!
//! `serde` serializes supported token, AST, and formatting types. The
//! `proc-macro2` feature converts token streams for interoperability with the
//! wider procedural-macro ecosystem.

/// Typed Rust syntax trees and parsing APIs; enabled by the `ast` feature.
#[cfg(feature = "ast")]
#[doc(inline)]
pub use moxy_ast as ast;

/// Build-script and rustc-version helpers; enabled by the `build` feature.
#[cfg(feature = "build")]
#[doc(inline)]
pub use moxy_build as build;

/// Derive macros; enabled by the `derive` feature.
#[cfg(feature = "derive")]
#[doc(inline)]
pub use moxy_derive::*;

/// Span-aware diagnostics; enabled by the `diagnostic` feature.
#[cfg(feature = "diagnostic")]
#[doc(inline)]
pub use moxy_diagnostic as diagnostic;

#[cfg(feature = "diagnostic")]
#[doc(inline)]
pub use moxy_diagnostic::{error, help, note, warn};

/// Formatting APIs; enabled by the `fmt` feature.
#[cfg(feature = "fmt")]
#[doc(inline)]
pub use moxy_fmt as fmt;

#[cfg(feature = "fmt")]
#[doc(inline)]
pub use moxy_fmt::fmt;

/// Runtime token templates and identifier pasting; enabled by the `template` feature.
#[cfg(feature = "template")]
#[doc(inline)]
pub use moxy_template as template;

#[cfg(feature = "template")]
#[doc(inline)]
pub use moxy_template::*;

/// Token streams, lexing, spans, and token-construction APIs; enabled by `token`.
#[cfg(feature = "token")]
#[doc(inline)]
pub use moxy_token as token;

#[cfg(all(feature = "token", not(feature = "ast")))]
#[doc(inline)]
pub use moxy_token::Token;

#[cfg(feature = "ast")]
#[doc(inline)]
pub use moxy_ast::{Token, parse, parse_files};
