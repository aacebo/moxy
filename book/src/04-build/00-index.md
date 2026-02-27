# Build

The `Build` derive macro generates a fluent builder for your struct. Annotate fields with `#[moxy(build)]` to include them in the builder, then call `YourStruct::new()` to get a builder instance.

## Basic Usage

```rust
use moxy::derive::Build;

#[derive(Build, Default)]
struct Config {
    #[moxy(build)]
    pub host: String,
    #[moxy(build)]
    pub port: u16,
}

let config = Config::new()
    .host("localhost")
    .port(8080_u16)
    .build();

assert_eq!(config.host, "localhost");
assert_eq!(config.port, 8080);
```

`Build` generates:
- A `ConfigBuilder` struct with `Option<T>` fields for each annotated field
- A setter method per annotated field that accepts `V: Into<T>` for flexible conversions
- A `build()` method that constructs the final struct
- A `Config::new()` factory that returns a fresh `ConfigBuilder`

Non-annotated fields are initialised with `Default::default()` via a struct spread.

## Partial Annotation

Only annotate the fields you want in the builder. The rest receive their `Default` value:

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

## Custom Method Name

By default the builder setter is named after the field. Pass a string literal to override it:

```rust
# use moxy::derive::Build;
#
#[derive(Build, Default)]
struct Credentials {
    #[moxy(build("username"))]
    pub user: String,
    #[moxy(build)]
    pub password: String,
}

let c = Credentials::new()
    .username("alice")   // calls the overridden setter
    .password("secret")
    .build();

assert_eq!(c.user, "alice");
```

## Default Values

Use `default = <expr>` inside `build(...)` to make a field optional in the builder while
providing a fallback value at build time:

```rust
# use moxy::derive::Build;
#
#[derive(Build, Default)]
struct Server {
    #[moxy(build(default = "localhost".to_string()))]
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

The `default` expression can be any valid Rust expression — literals, function calls, constants, etc.

## Generics

`Build` works with generic structs. Type parameters are forwarded to the generated builder:

```rust
# use moxy::derive::Build;
#
#[derive(Build, Default)]
struct Wrapper<T: Default> {
    #[moxy(build)]
    pub value: T,
}

let w: Wrapper<u32> = Wrapper::new().value(42u32).build();
assert_eq!(w.value, 42u32);
```

## Required vs Optional Fields

- **Required** (`#[moxy(build)]` or `#[moxy(build("name"))]`): calling `build()` without setting
  the field will panic with `"required"`.
- **Optional** (`#[moxy(build(default = ...))]`): the field has a fallback and can be omitted.

```rust
# use moxy::derive::Build;
#
#[derive(Build, Default)]
struct Api {
    #[moxy(build(default = "localhost".to_string()))]
    pub host: String,    // optional — has a default
    #[moxy(build)]
    pub token: String,   // required — must be set
}
```
