# Setters

Every field annotated with `#[moxy(build)]` gets a fluent setter method on the generated builder. Setters accept any value that implements `Into<T>`, so callers are not forced to construct the exact field type.

## `V: Into<T>` Signature

```rust
use moxy::derive::Build;

#[derive(Build, Default)]
struct Server {
    #[moxy(build)]
    pub host: String,
    #[moxy(build)]
    pub port: u16,
}

// &str is accepted because &str: Into<String>
let s = Server::new().host("localhost").port(8080_u16).build();
assert_eq!(s.host, "localhost");
```

The generated setter is equivalent to:

```rust,ignore
pub fn host<V: Into<String>>(mut self, value: V) -> Self {
    self.host = Some(value.into());
    self
}
```

## Partial Annotation

Only annotate the fields you want in the builder. Fields without `#[moxy(build)]` are not exposed as setters — they receive `Default::default()` when `build()` is called:

```rust
# use moxy::derive::Build;
#
#[derive(Build, Default)]
struct Connection {
    #[moxy(build)]
    pub host: String,
    #[moxy(build)]
    pub port: u16,
    pub timeout: u64,   // not in builder — gets 0u64
}

let conn = Connection::new().host("127.0.0.1").port(5432_u16).build();
assert_eq!(conn.timeout, 0u64);
```

## Required vs Optional Fields

A field annotated with bare `#[moxy(build)]` is **required** — calling `build()` without setting it panics:

```rust,should_panic
# use moxy::derive::Build;
#
#[derive(Build, Default)]
struct Config {
    #[moxy(build)]
    pub host: String,
    #[moxy(build)]
    pub port: u16,
}

// panics: "required"
Config::new().host("localhost").build();
```

To make a field optional, provide a fallback with [`default = <expr>`](./02-defaults.md).
