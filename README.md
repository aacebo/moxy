<img src="https://raw.githubusercontent.com/aacebo/moxy/refs/heads/master/assets/banner.svg" width="100%" alt="moxy" />

<a href="https://crates.io/crates/moxy">
    <img src="https://img.shields.io/crates/v/moxy" />
</a>
<a href="https://app.codecov.io/gh/aacebo/moxy/tree/master">
    <img src="https://img.shields.io/codecov/c/github/aacebo/moxy/master" />
</a>
<a href="https://github.com/aacebo/moxy/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/aacebo/moxy" />
</a>
<a href="https://github.com/aacebo/moxy/actions/workflows/ci.yaml">
  <img src="https://github.com/aacebo/moxy/actions/workflows/ci.yaml/badge.svg?branch=master" />
</a>
<a href="https://crates.io/crates/moxy">
    <img src="https://img.shields.io/crates/size/moxy" />
</a>
<a href="https://github.com/aacebo/moxy/blob/master/BENCH.md">
  <img
    src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fapi.bencher.dev%2Fv0%2Fprojects%2Fmoxy%2Freports%3Fper_page%3D1%26branch%3Dmaster%26direction%3Ddesc&query=%24%5B0%5D.counts.alerts.total&label=bencher&suffix=%20alerts&color=FC7300&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxMjAgMTEzIj48cGF0aCBmaWxsPSIjZmZmIiBkPSJtIDg3LjA0LDU0Ljg3IGMgNS40MTMzMzMsNC4yNCA5LjExMzMzMyw5LjQ3NjY2NyAxMS4xLDE1LjcxIDEuODUzMzMzLDUuNzg2NjY3IDIuMTUzMzMsMTEuNDM2NjY3IDAuOSwxNi45NSAtMS4zNjY2NjcsNi4wMTMzMzMgLTQuNjEsMTEuMDIzMzMzIC05LjczLDE1LjAzIC00LjM4NjY2NywzLjQ0IC05LjczNjY2Nyw1Ljc3MzMzIC0xNi4wNSw3IC05LjgzMzMzMywxLjkxMzMzIC0xOS42MSwxLjY3IC0yOS4zMywtMC43MyAtNC4wNTMzMzMsLTEgLTcuODIzMzMzLC0yLjYzNjY3IC0xMS4zMSwtNC45MSBDIDI3LjgyNjY2NywxMDAuNzg2NjcgMjQuNDIsOTYuNzcgMjIuNCw5MS44NyAyMC41NTMzMzMsODcuMzYzMzMzIDE5Ljk1LDgyLjM2NjY2NyAyMC41OSw3Ni44OCAyMS41ODMzMzMsNjguNDQ2NjY3IDI1LjYyMzMzMyw2MS4xNTMzMzMgMzIuNzEsNTUgYyAwLjYwMjk1OCwtMC41MjY1ODQgMC43NDU5NDYsLTEuNDA1NTM5IDAuMzQsLTIuMDkgLTQuMDQsLTYuNzYgLTcuMjEzMzMzLC0xNC4wNDMzMzMgLTkuNTIsLTIxLjg1IC0xLjM4LC00LjY3MzMzMyAtMS45MDY2NjcsLTguNjEgLTEuNTgsLTExLjgxIDAuNDMzMzMzLC00LjM1MzMzMyAxLjk3LC04LjE2NjY2NjcgNC42MSwtMTEuNDQgMy4xNiwtMy45MTMzMzMzIDcuMDU2NjY3LC01Ljc5NjY2NjcgMTEuNjksLTUuNjUgMy45MiwwLjEyIDcuNiwxLjU5IDExLjA0LDQuNDEgNC42LDMuNzggNy4zMzY2NjY3LDcuNDI5MDQgOC4yMSwxMy45MzU3MDcgMC40OSwzLjY0MzMzMyAwLjY1NzQzMiw1Ljk1Mjk5NiAwLjg2NTM4NywxMC43Mzk4MjMgMC4xNTA0ODIsMy41MjI0MjUgMC4xMTE1OTYsNy4yODA1NzUgMC4wNzQwMiwxMC42MDI0MDUgLTAuMDQ3NzMsMS4yMDUzMTEgLTAuMDExMTYsMi4wNjI4NzcgMC4xOTM0MjYsMi4zOTQxMzkgMC43MzQzOTMsMS4xMzA3NTIgMi40MjM4NDEsMS4wNTQ1OTMgMi44MzcxNzQsLTAuMDkyMDcgMC4xNTMzMzMsLTAuNDMzMzMyIDAuMjA2NjY3LC0xLjE3IDAuMTYsLTIuMjEgLTAuMTgsLTMuNiAtMC4xODMzMzMsLTcuMiAtMC4wMSwtMTAuOCAwLjI0LC01LjA5MzMzMyAwLjU2LC04LjY3NjY2NyAwLjk2LC0xMC43NSAxLjQ4LC03LjY4IDUuNzIzMzMzLC0xMy4yOSAxMi43MywtMTYuODMgMy40LC0xLjcxMzMzMzMgNi44NjY2NjcsLTIuMDEzMzMzIDEwLjQsLTAuOSAyLjk4LDAuOTQ2NjY2NyA1LjYwMzMzMywyLjg3MzMzMzIgNy44Nyw1Ljc4IDMuMzMzMzMzLDQuMjggNC44NjY2NjcsOS4wODMzMzIgNC42LDE0LjQxIC0wLjE4NjY2NywzLjY0IC0xLjYxNjY2Nyw4Ljg2MzMzMyAtNC4yOSwxNS42NyAtMS45NzMzMzIsNS4wNDY2NjcgLTQuMzYsOS45NCAtNy4xNiwxNC42OCAtMC4zMjY1NzIsMC41NTkzNzMgLTAuMTk0Mzc2LDEuMjcxNTIyIDAuMzUsMS42NyB6IG0gLTIwLjg1LC01IEMgNjIuMjgzMzMzLDQ5LjM4MzMzMyA1OC4xNDMzMzMsNDkuMzcgNTMuNzcsNDkuODMgNTMuNDQ3MjQ0LDQ5Ljg1NzggNTMuMTgwNjkzLDQ5LjU4MTU1OCA1My4yMiw0OS4yNiA1NC40NzMzMzMsMzkuNDkzMzMzIDU0LjMxNjY2NywyOS45OSA1Mi43NSwyMC43NSA1Mi4zMjMzMzMsMTguMjMgNTEuMjY2NjY3LDE1Ljc5IDQ5LjU4LDEzLjQzIDQ4LjAzMzMzMywxMS4yNTY2NjcgNDYuMDg2NjY3LDkuNTYgNDMuNzQsOC4zNCAzNy45LDUuMjkzMzMzIDMzLjEzNjY2Nyw2LjQzNjY2NjcgMjkuNDUsMTEuNzcgYyAtMS40MDY2NjcsMi4wMzMzMzMgLTIuMzQzMzMzLDQuMzU2NjY3IC0yLjgxLDYuOTcgLTAuNSwyLjg0IC0wLjQ0MzMzMyw1LjQwNjY2NyAwLjE3LDcuNyAyLjY2NjY2Nyw5Ljg2NjY2NyA2LjkyMzMzMywxOS4yNzY2NjcgMTIuNzcsMjguMjMgMC4xNzMzODgsMC4yNjUyNDkgMC4wOTYzMSwwLjYxODkxMyAtMC4xNywwLjc4IC01LjgsMy42NjY2NjcgLTkuOTIsOC40MjMzMzMgLTEyLjM2LDE0LjI3IC0yLjQ4LDUuOTUzMzMzIC0yLjkyNjY2NywxMS44OSAtMS4zNCwxNy44MSAyLjAwNjY2Nyw3LjUgNy4xMjMzMzMsMTIuNzIgMTUuMzUsMTUuNjYgMTEuNTgsNC4xMzMzMyAyMy42MzY2NjcsNC4zIDM2LjE3LDAuNSA3Ljg3MzMzMywtMi4zODY2NyAxMy4xOCwtNi44MjMzMzMgMTUuOTIsLTEzLjMxIDIuNTEzMzMzLC01Ljk1MzMzMyAyLjY2MzMzMywtMTIuMjQzMzMzIDAuNDUsLTE4Ljg3IC0yLjE5MzMzMywtNi41NCAtNi41MiwtMTEuODU2NjY3IC0xMi45OCwtMTUuOTUgLTAuMzA4MTA0LC0wLjE5NDc2NiAtMC4zOTgzNTYsLTAuNjAwODk5IC0wLjIsLTAuOSA1LjA3MzMzMywtNy43OTMzMzMgOC45NTY2NjcsLTE1LjkyNjY2NyAxMS42NSwtMjQuNCAwLjk2LC0zLjA0IDEuNDg2NjY3LC01LjYzIDEuNTgsLTcuNzcgMC4xMjY2NjcsLTMuMDg2NjY3IC0wLjM2LC01LjczIC0xLjQ2LC03LjkzIC0zLjM0NjY2NywtNi42NTMzMzMzIC03Ljg3MzMzMywtOS4wNyAtMTMuNTgsLTcuMjUgLTMuNTY2NjY3LDEuMTMzMzMzMyAtNi40MywzLjQzMzMzMyAtOC41OSw2LjkgLTIuMDEzMzMzLDMuMjI2NjY3IC0zLjE4NjY2Nyw2LjkwNjY2NyAtMy41MiwxMS4wNCAtMC42OCw4LjQyNjY2NyAtMC42MzMzMzMsMTYuNDg2NjY3IDAuMTQsMjQuMTggMC4wMjgxOCwwLjI1Nzg5MSAtMC4xOTI4MDcsMC40NzM5NjMgLTAuNDUsMC40NCB6Ii8%2BPC9zdmc%2B"
    alt="Bencher"
  />
</a>

Rust syntax tools for procedural macros: tokens, typed syntax trees, templates,
formatting, and diagnostics.

> [!WARNING]
> **Moxy is under active development.**
>
> APIs, behavior, and documentation may change frequently and without notice. Moxy is not yet considered stable or production-ready.

## Quick Start

```console
cargo add moxy --features template,fmt
```

```rust
use moxy::ast::Item;

let name = "Widget";
let tokens = moxy::template! {
    pub struct {{ name }};
};

let item: Item = moxy::parse!(tokens).unwrap();

assert_eq!(item.as_struct().unwrap().ident.text(), "Widget");
assert_eq!(moxy::fmt!(&item).unwrap(), "pub struct Widget;");
```

## Features

Default features are `token` and `ast`.

| Feature | Default | Enables |
| --- | :---: | --- |
| `token` | yes | Token streams, spans, parsing, and token construction |
| `ast` | yes | Typed Rust syntax trees; implies `token` |
| `template` | no | `template!` and `paste!`; implies `token` |
| `fmt` | no | AST formatting with `fmt!`; implies `ast` |
| `diagnostic` | no | Span-aware error, warning, note, and help diagnostics |
| `build` | no | Cargo build-script and rustc-version helpers |
| `derive` | no | `#[derive(ToTokens)]` and its supporting pipeline |
| `serde` | no | Serialization for supported token, AST, and formatting types |
| `proc-macro2` | no | Conversions between moxy and `proc_macro2` tokens |
| `full` | no | Every feature above |

Choose only the layers you need:

```console
cargo add moxy --no-default-features --features token,ast
```

## Feature guide

### Tokens

The `token` feature is the foundation, similar in role to `proc-macro2`.

```rust
use moxy::Token;
use moxy::token::ident;

let name = ident!(Generated, "_", Item);
let comma: Token![,] = Default::default();

assert_eq!(name.to_string(), "Generated_Item");
assert_eq!(comma.as_str(), ",");
```

### Abstract Syntax Tree

The `ast` feature provides typed entry points such as `Item`, `Expr`, and
`Type`, following the same parse-at-the-level-you-need style as `syn`.

```rust
use moxy::ast::{Expr, Item, Type};

let item: Item = moxy::parse!("pub struct User { id: u64 }").unwrap();
let ty: Type = moxy::parse!("Option<Result<T, E>>").unwrap();
let expr: Expr = moxy::parse!("items.next()?").unwrap();

assert!(item.is_struct());
assert!(ty.is_path());
assert!(expr.is_unary());
```

### Templates

The `template` feature builds token streams with interpolation and control
flow in the style of `quote!`.

```rust
let fields = ["id", "name"];

let tokens = moxy::template! {
    struct User {
        @for (field in fields) {
            {{ field }}: String,
        }
    }
};

assert!(tokens.to_string().contains("struct User"));
```

`paste!` creates identifiers at expansion time:

```rust
moxy::paste! {
    fn {{ read_ value }}() -> u32 { 7 }
}

assert_eq!(read_value(), 7);
```

### Formatting

The `fmt` feature formats parsed syntax trees with configurable width,
indentation, and newlines.

```rust
use moxy::ast::Item;
use moxy::fmt::{FmtConfig, Indent};

let item: Item = moxy::parse!("struct User { id: u64, name: String }").unwrap();
let config = FmtConfig::default().with_indent(Indent::space(2));
let output = moxy::fmt!(&item, config).unwrap();

assert_eq!(output, "struct User {\n  id: u64,\n  name: String,\n}");
```

### Diagnostics

The `diagnostic` feature builds span-aware diagnostics with a stable
`compile_error!` fallback.

```rust
let tokens = moxy::error!(
    "missing template",
    [moxy::help!("add #[template { ... }]")],
)
.emit();

assert!(tokens.to_string().contains("compile_error"));
```

### Build

Enable `build` as a build dependency for typed Cargo directives and rustc
version checks.

```console
cargo add moxy --build --no-default-features --features build
```

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = moxy::build::rustc::Config::read()?;

    config
        .min_version("1.85.0")
        .check_cfg("cfg(nightly)")
        .rerun_if_changed("build.rs");

    if config.version().channel.is_nightly() {
        config.cfg("nightly");
    }

    config.emit();
    Ok(())
}
```

### Derive

Enable `derive` to implement `ToTokens` from an inline template.

```console
cargo add moxy --features derive
```

```rust
use moxy::token::ToTokenStream;

#[derive(moxy::ToTokens)]
#[moxy(template { const VALUE: &str = {{ self.value }}; })]
struct Generated {
    value: String,
}

let tokens = Generated { value: "seven".into() }.to_token_stream();
assert!(tokens.to_string().contains("VALUE"));
```

Add `#[moxy(debug)]` beside `#[moxy(template { ... })]` to print the parsed
declaration and generated implementation as compiler notes.

### Integrations

`serde` adds serialization for supported token, AST, and formatter types.
`proc-macro2` adds token conversions for interoperability with the wider
procedural-macro ecosystem.
