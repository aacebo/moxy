# Backlog

## 1. Meta Argument Parsing

Similar to the `darling` crate, we need a derive macro and trait to make it easy to
parse complex proc macro arguments.

## 2. Refactor `moxy-fmt` AST

Refactor ast of fmt crate to implement `Parse` and `Format` for each sub node type.

## 3. Refactor `moxy-template` AST

Refactor ast for template crate to implement `Parse` for each sub node type.

## 4. Make Result More Ergonomic

Early returns from proc macro functions are very ergonomic since their return signature is `TokenStream`,
need to find a way for early returns from `Result<TokenStream, ParseError>` to be less verbose, which currently
requires `if let Ok(..)` statements.
