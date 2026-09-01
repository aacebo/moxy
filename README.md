<img src="./assets/banner.svg" width="100%" alt="moxy" />

[![codecov](https://codecov.io/gh/aacebo/moxy/branch/master/graph/badge.svg?token=zVUrga2una)](https://codecov.io/gh/aacebo/moxy)

Rust syntax tools for procedural macros: tokens, typed syntax trees, templates,
formatting, and diagnostics.

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
| `trace` | no | Enables parser tracing |
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

### Tracing

The `trace` feature enables trace output to `stdout` to help debug parsing.

![Tracing](https://github.com/aacebo/moxy/blob/master/assets/tracing.png?raw=true)

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

Add `#[debug]` beside `#[template]` to print the parsed declaration and
generated implementation as compiler notes.

### Integrations

`serde` adds serialization for supported token, AST, and formatter types.
`proc-macro2` adds token conversions for interoperability with the wider
procedural-macro ecosystem.
