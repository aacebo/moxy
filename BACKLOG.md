# Backlog

## 1. Formatter Macros

We need formatter macros like `format_ident!("a_{}", "test")` to make constructing new identifiers
and other syntax simpler.

## 2. `moxy-ast-macros`

We need to make sum types and other ast type patterns easier to maintain, we can make a new 
`proc-macro` crate that exposes a derive macro for said patterns.

## 3. Token/AST Accessors

We need more `is_*` and `as_*` accessors on sum types to make usage less
verbose.

## 4. Add `moxy::format!`

Add format macro for token formatting in macro_rules like how it is done via the crate 
https://github.com/dtolnay/paste
