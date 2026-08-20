<img src="./assets/banner.svg" width="100%" />

A proc-macro development framework.

## Features

- [`token`](#tokensspans)
- [`ast`](#abstract-syntax-tree-ast)
- [`template`](#templates)
- [`fmt`](#formatting)
- [`diagnostic`](#diagnostics)
- [`test`](#testing)
- `serde` - adds serde support for all token/ast/error types.
- `proc-macro2` - bridge between our token types and proc-macro2's.
- `syn` - bridge between our AST types and syn types.

## Tokens/Spans

## Abstract Syntax Tree (AST)

## Templates

## Formatting

## Diagnostics

## Testing

Install [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) before
running the full workspace checks.

```sh
cargo test --workspace --all-features
cargo cov-check
```

Use `cargo cov` to generate and open the all-features HTML coverage report.
