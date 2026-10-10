mod ast;
mod fixtures;
mod parser;

#[cfg(all(feature = "fmt", feature = "derives"))]
mod attributes;

#[cfg(feature = "fmt")]
mod constants;

#[cfg(all(feature = "fmt", feature = "visit"))]
mod crates;

#[cfg(feature = "fmt")]
mod enums;

#[cfg(all(feature = "fmt", feature = "derives"))]
mod expressions;

#[cfg(feature = "fmt")]
mod externs;

#[cfg(feature = "fmt")]
mod functions;

#[cfg(feature = "fmt")]
mod generics;

#[cfg(feature = "fmt")]
mod implementations;

#[cfg(all(feature = "fmt", feature = "derives"))]
mod literals;

#[cfg(feature = "fmt")]
mod modules;

#[cfg(feature = "fmt")]
mod operators;

#[cfg(feature = "fmt")]
mod paths;

#[cfg(all(feature = "fmt", feature = "derives"))]
mod patterns;

#[cfg(all(feature = "fmt", feature = "derives"))]
mod statements;

#[cfg(feature = "fmt")]
mod structs;

#[cfg(all(feature = "fmt", feature = "derives"))]
mod traits;

#[cfg(feature = "fmt")]
mod types;

#[cfg(feature = "fmt")]
mod unions;

#[cfg(feature = "fmt")]
mod use_items;

#[cfg(all(feature = "fmt", feature = "macros", feature = "template"))]
mod macros;

#[cfg(all(feature = "fmt", feature = "template"))]
mod template;

#[cfg(all(feature = "quote", feature = "template"))]
mod quote;
