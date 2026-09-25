# Backlog

## 2. Meta Argument Parsing

### 2.1

Audit and possibly redesign attribute/meta query system to support optional, boolean, string,
numeric type arguments, and error messages.

### 2.2

Similar to the `darling` crate, we need a derive macro and trait to make it easy to
parse complex proc macro arguments.

## 3. Refactor `moxy-fmt` AST

Refactor ast of fmt crate to implement `Parse` and `Format` for each sub node type.

## 4. Refactor `moxy-template` AST

Refactor ast for template crate to implement `Parse` for each sub node type.

## 5. Audit/Remove `moxy-ast` Leaf Nodes

Most of the leaf node types found in [leaf.rs](./crates/moxy-ast/src/leaf.rs) are
essentially `Option<Token![{..}]>` and can be replaced by that instead of having
independent types.

## 6. Add Non Exhaustive

Add `#[non_exhaustive]` to:

- Expr
- Pattern
- Item
- Type
- ForeignItem
- TraitItem
- ImplItem

## 7. Make Result More Ergonomic

Early returns from proc macro functions are very ergonomic since their return signature is `TokenStream`,
need to find a way for early returns from `Result<TokenStream, ParseError>` to be less verbose, which currently
requires `if let Ok(..)` statements.
