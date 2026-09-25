mod ast;
mod attributes;
mod constants;
mod crates;
mod enums;
mod expressions;
mod externs;
mod fixtures;
mod functions;
mod generics;
mod implementations;
mod literals;
mod modules;
mod operators;
mod parser;
mod paths;
mod patterns;
mod statements;
mod structs;
mod traits;
mod types;
mod unions;
mod use_items;

#[cfg(feature = "derive")]
mod derive;

#[cfg(feature = "template")]
mod template;
