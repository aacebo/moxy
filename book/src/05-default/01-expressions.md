# Expressions

The `default = <expr>` attribute accepts any valid Rust expression — string literals, typed literals, constants, and method calls. The expression is passed through `.into()`, so it does not need to match the exact field type.

## Literals

String literals and typed numeric literals work directly:

```rust
use moxy::derive::Default;

#[derive(Default)]
struct Server {
    #[moxy(default = "0.0.0.0")]
    pub bind: String,
    #[moxy(default = 443u16)]
    pub port: u16,
    #[moxy(default = true)]
    pub tls: bool,
}

let s = Server::default();
assert_eq!(s.bind, "0.0.0.0");
assert_eq!(s.port, 443);
assert_eq!(s.tls, true);
```

## `impl Into<T>` Coercion

The default expression is wrapped in `.into()`, so any type implementing `Into<T>` for the field type works:

```rust
# use moxy::derive::Default;
#
#[derive(Default)]
struct App {
    // &str → String via Into
    #[moxy(default = "my-app")]
    pub name: String,

    // method call returning String — identity Into
    #[moxy(default = "api".to_string())]
    pub prefix: String,
}

let app = App::default();
assert_eq!(app.name, "my-app");
assert_eq!(app.prefix, "api");
```

## Constants

Named constants and static values work as expressions:

```rust
# use moxy::derive::Default;
#
const MAX_RETRIES: u32 = 3;

#[derive(Default)]
struct Client {
    #[moxy(default = MAX_RETRIES)]
    pub retries: u32,
}

let c = Client::default();
assert_eq!(c.retries, 3);
```

## Complex Expressions

Any expression valid in value position works — function calls, constructor calls, and more:

```rust
# use moxy::derive::Default;
#
#[derive(Default)]
struct Collection {
    #[moxy(default = Vec::new())]
    pub items: Vec<String>,

    #[moxy(default = String::from("unnamed"))]
    pub label: String,
}

let c = Collection::default();
assert!(c.items.is_empty());
assert_eq!(c.label, "unnamed");
```

The expression is evaluated each time `default()` is called — there is no caching or static initialization.
