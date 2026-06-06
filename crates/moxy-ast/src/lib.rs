#![allow(unused)]

extern crate self as moxy_ast;

mod _crate;
pub mod args;
pub mod attr;
mod declaration;
mod delimited;
pub mod expr;
pub mod fields;
pub mod generics;
mod ident;
pub mod item;
mod label;
mod leaf;
pub mod lit;
mod macro_call;
#[macro_use]
mod macros;
pub mod member;
mod misc;
pub mod pat;
pub mod path;
mod precedence;
mod punctuated;
pub mod sig;
pub mod stmt;
pub mod ty;
pub mod use_tree;
pub mod vis;

#[doc(inline)]
pub use _crate::Crate;
#[doc(inline)]
pub use args::{AngleArgs, GenericArgument};
#[doc(inline)]
pub use attr::{Attribute, Meta};
#[doc(inline)]
pub use declaration::*;
#[doc(inline)]
pub use delimited::Delimited;
#[doc(inline)]
pub use expr::{BinaryExpr, BlockExpr, Expr, JumpExpr, MatchArm, PostfixExpr, PrimaryExpr, UnaryExpr};
#[doc(inline)]
pub use fields::{FieldValue, Fields, FieldsNamed};
#[doc(inline)]
pub use generics::{Generics, TraitRef, TypeBound, WhereClause, WherePredicate};
#[doc(inline)]
pub use ident::*;
#[doc(inline)]
pub use item::{Item, Variant};
#[doc(inline)]
pub use label::*;
#[doc(inline)]
pub use leaf::*;
#[doc(inline)]
pub use lit::Lit;
#[doc(inline)]
pub use macro_call::*;
#[doc(inline)]
pub use member::{ForeignItem, ImplItem, Member, TraitItem};
#[doc(inline)]
pub use misc::*;
#[doc(inline)]
pub use pat::Pattern;
#[doc(inline)]
pub use path::{Lifetime, Path, PathSegment};
#[doc(inline)]
pub use punctuated::*;
#[doc(inline)]
pub use sig::{Abi, BareFnArg, Signature, Variadic};
#[doc(inline)]
pub use stmt::{Stmt, StmtBlock};
#[doc(inline)]
pub use ty::{QSelf, Type, TypeMacro, TypedParam};
#[doc(inline)]
pub use use_tree::UseTree;
#[doc(inline)]
pub use vis::*;
