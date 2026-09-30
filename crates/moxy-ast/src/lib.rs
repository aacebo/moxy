//! # Moxy AST
//!
//! Typed, lossless Rust syntax trees built on `moxy-token`. Each top-level
//! grammar family—such as [`Expr`], [`Item`], [`Pattern`], and [`Type`]—is an
//! enum whose variants hold the concrete syntax node.
//!
//! ## Quick start
//!
//! Parse at the grammar level needed by a tool, inspect the resulting node, and
//! emit it again with `moxy::token::ToTokens`:
//!
//! ```ignore
//! use moxy::ast::Item;
//!
//! let item: Item = moxy::parse!("pub struct User;").unwrap();
//! assert!(item.is_struct());
//! ```
//!
//! ## Attributes
//!
//! Nodes that carry Rust attributes implement [`Attributed`]. It exposes their
//! attributes in source order and can parse a named attribute through
//! [`Attributed::parse_meta`]. With the `moxy` crate's `derive` feature, use
//! `#[derive(moxy::FromMeta)]` to convert structured attributes into named
//! structs or enums.
//!
//! ## Generics
//!
//! [`Generics::split`] produces token views for an `impl` header and its type
//! path, and returns the `where` clause separately. [`ImplGenerics`] omits type
//! and const defaults while preserving bounds; [`TypeGenerics`] emits only
//! generic arguments.

mod _crate;
mod args;
mod attr;
mod core;
mod declaration;
mod delimited;
mod expr;
mod fields;
mod generics;
mod item;
mod macro_call;
#[macro_use]
mod macros;
mod member;
mod pat;
mod path;
mod punctuated;
mod sig;
mod stmt;
mod ty;
mod use_tree;
#[cfg(feature = "visit")]
mod visit;
#[macro_use]
mod parse;
mod file;

#[doc(inline)]
pub use _crate::*;
#[doc(inline)]
pub use args::*;
#[doc(inline)]
pub use attr::*;
#[doc(inline)]
pub use core::*;
#[doc(inline)]
pub use declaration::*;
#[doc(inline)]
pub use delimited::*;
#[doc(inline)]
pub use expr::*;
#[doc(inline)]
pub use fields::*;
#[doc(inline)]
pub use file::*;
#[doc(inline)]
pub use generics::*;
#[doc(inline)]
pub use item::*;
#[doc(inline)]
pub use macro_call::*;
#[doc(inline)]
pub use member::*;
#[doc(inline)]
pub use parse::*;
#[doc(inline)]
pub use pat::*;
#[doc(inline)]
pub use path::*;
#[doc(inline)]
pub use punctuated::*;
#[doc(inline)]
pub use sig::*;
#[doc(inline)]
pub use stmt::*;
#[doc(inline)]
pub use ty::*;
#[doc(inline)]
pub use use_tree::*;
#[cfg(feature = "visit")]
#[doc(inline)]
pub use visit::*;
