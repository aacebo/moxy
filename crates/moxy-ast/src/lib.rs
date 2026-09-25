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
//! ```
//! use moxy::ast::Item;
//!
//! let item: Item = moxy::parse!("pub struct User;").unwrap();
//! assert!(item.is_struct());
//! ```

mod _crate;
/// Generic argument syntax.
pub mod args;
/// Attributes and attribute metadata.
pub mod attr;
mod core;
mod declaration;
mod delimited;
/// Expression syntax.
pub mod expr;
/// Struct, enum, and union field syntax.
pub mod fields;
/// Generic parameters, bounds, and where clauses.
pub mod generics;
/// Item-level Rust syntax.
pub mod item;
mod label;
mod macro_call;
#[macro_use]
mod macros;
mod lifetime;
/// Members of traits, implementations, and foreign modules.
pub mod member;
/// Pattern syntax.
pub mod pat;
/// Path syntax and path arguments.
pub mod path;
mod punctuated;
mod sig;
/// Statement syntax.
pub mod stmt;
/// Type syntax.
pub mod ty;
/// `use`-tree syntax.
pub mod use_tree;
mod vis;
#[cfg(feature = "visit")]
mod visit;
#[macro_use]
mod parse;
mod file;
mod punct;

#[doc(inline)]
pub use _crate::Crate;
#[doc(inline)]
pub use args::{AngleArguments, GenericArgument, ParenArguments};
#[doc(inline)]
pub use attr::*;
#[doc(inline)]
pub use core::*;
#[doc(inline)]
pub use declaration::*;
#[doc(inline)]
pub use delimited::Delimited;
#[doc(inline)]
pub use expr::Expr;
#[doc(inline)]
pub use fields::*;
#[doc(inline)]
pub use file::*;
#[doc(inline)]
pub use generics::{Generics, TraitRef, TypeBound, WhereClause, WherePredicate};
#[doc(inline)]
pub use item::{Item, Variant};
#[doc(inline)]
pub use label::*;
#[doc(inline)]
pub use lifetime::*;
#[doc(inline)]
pub use macro_call::*;
#[doc(inline)]
pub use member::{ForeignItem, ImplItem, Member, TraitItem};
#[doc(inline)]
pub use parse::*;
#[doc(inline)]
pub use pat::Pattern;
#[doc(inline)]
pub use path::{Path, PathSegment};
#[doc(inline)]
pub use punct::*;
#[doc(inline)]
pub use punctuated::*;
#[doc(inline)]
pub use sig::*;
#[doc(inline)]
pub use stmt::{Stmt, StmtBlock};
#[doc(inline)]
pub use ty::{QSelf, Type, TypeMacro};
#[doc(inline)]
pub use use_tree::UseTree;
#[doc(inline)]
pub use vis::*;
#[cfg(feature = "visit")]
#[doc(inline)]
pub use visit::*;
