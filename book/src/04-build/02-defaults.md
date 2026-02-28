# Defaults

Use `default = <expr>` inside `build(...)` to make a field optional in the builder. When `build()` is called without setting the field, the expression is evaluated and its result is used as the value.

## Basic Default

```rust
use moxy::Build;

#[derive(Build, Default)]
struct Server {
    #[moxy(build(default = "localhost"))]
    pub host: String,
    #[moxy(build(default = 8080u16))]
    pub port: u16,
    #[moxy(build)]
    pub name: String,
}

// Only `name` is required; host and port fall back to their defaults.
let s = Server::new().name("api").build();
assert_eq!(s.host, "localhost");
assert_eq!(s.port, 8080u16);

// Defaults can still be overridden.
let s = Server::new().host("0.0.0.0").port(443u16).name("tls").build();
assert_eq!(s.host, "0.0.0.0");
```

## `impl Into<T>` Expressions

The default expression is passed through the same `Into<T>` conversion used by the setter, so it does not need to be the exact field type — it only needs to implement `Into<T>`:

```rust
# use moxy::Build;
#
#[derive(Build, Default)]
struct App {
    // &str → String via Into
    #[moxy(build(default = "localhost"))]
    pub host: String,

    // method call returning String — identity Into
    #[moxy(build(default = "api".to_string()))]
    pub name: String,

    // Vec::new() — Vec<String>: Into<Vec<String>>
    #[moxy(build(default = Vec::new()))]
    pub tags: Vec<String>,
}
```

## Constants and Expressions

Any valid Rust expression works — including named constants, function calls, and more complex expressions:

```rust
# use moxy::Build;
#
const DEFAULT_RETRIES: u32 = 3;

#[derive(Build, Default)]
struct Client {
    #[moxy(build(default = DEFAULT_RETRIES))]
    pub retries: u32,
}

let c = Client::new().build();
assert_eq!(c.retries, 3);
```

The expression is placed inside `unwrap_or_else(|| <expr>.into())`, so it is evaluated lazily — only when the field was not set.

> [!TIP]
> Because defaults are evaluated lazily, expensive expressions like `Vec::new()` or function calls are only executed if the field was not set by the caller — there is no cost when the field is provided.
