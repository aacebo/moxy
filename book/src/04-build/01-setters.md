# Setters

Every field annotated with `#[moxy(build)]` gets a fluent setter method on the generated builder. Setters accept any value that implements `Into<T>`, so callers are not forced to construct the exact field type.

> [!NOTE]
> All missing-field errors are **compile errors**, not runtime panics. The typestate guarantees that `build()` cannot be called until every required field has been set.

## `V: Into<T>` Signature

```rust
use moxy::Build;

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

Required setters consume the builder and return a new type, advancing the typestate. Optional setters (fields with a `default`) mutate in place and return `Self`.

## Partial Annotation

Only annotate the fields you want in the builder. Fields without `#[moxy(build)]` are not exposed as setters — they receive `Default::default()` when `build()` is called:

```rust
# use moxy::Build;
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

A field annotated with bare `#[moxy(build)]` is **required** — `build()` is not available until all required fields are set. Forgetting one is a compile error:

```rust,compile_fail
# use moxy::Build;
#
#[derive(Build, Default)]
struct Config {
    #[moxy(build)]
    pub host: String,
    #[moxy(build)]
    pub port: u16,
}

// error[E0599]: no method named `build` found for struct `ConfigBuilder<true>`
//  --> src/main.rs:10:35
//   |
//   | Config::new().host("localhost").build();
//   |                                 ^^^^^ method not found in `ConfigBuilder<true>`
//   |
//   = note: the method was found for
//           - `ConfigBuilder<true, true>`
Config::new().host("localhost").build();
```

> [!CAUTION]
> Setting a required field twice is also a compile error — the setter is consumed after first use and the updated builder type no longer has that method.

Setting a required field twice is also a compile error — the setter is consumed after use:

```rust,compile_fail
# use moxy::Build;
#
# #[derive(Build, Default)]
# struct Config {
#     #[moxy(build)]
#     pub host: String,
#     #[moxy(build)]
#     pub port: u16,
# }
#
// error[E0599]: no method named `host` found for struct `ConfigBuilder<true>`
//   |
//   | Config::new().host("a").host("b").port(80_u16).build();
//   |                         ^^^^ method not found in `ConfigBuilder<true>`
//   |
//   = note: method `host` is available on `ConfigBuilder`
Config::new().host("a").host("b").port(80_u16).build();
```

To make a field optional, provide a fallback with [`default = <expr>`](./02-defaults.md) or use an `Option<T>` type.

## `Option<T>` Fields

Fields with an `Option<T>` type are automatically optional — no `default` attribute needed. The setter accepts the inner type `T` and wraps it in `Some`:

```rust
# use moxy::Build;
#
#[derive(Build, Default)]
struct Profile {
    #[moxy(build)]
    pub name: String,
    #[moxy(build)]
    pub bio: Option<String>,
}

// bio is optional — defaults to None
let p = Profile::new().name("alice").build();
assert_eq!(p.bio, None);

// setter accepts &str (not Option<&str>) and wraps in Some
let p = Profile::new().name("alice").bio("hello").build();
assert_eq!(p.bio, Some("hello".to_string()));
```

## Any Order

Required setters can be called in any order:

```rust
# use moxy::Build;
#
# #[derive(Build, Default)]
# struct Config {
#     #[moxy(build)]
#     pub host: String,
#     #[moxy(build)]
#     pub port: u16,
# }
#
let a = Config::new().host("localhost").port(8080_u16).build();
let b = Config::new().port(8080_u16).host("localhost").build();

assert_eq!(a.host, b.host);
assert_eq!(a.port, b.port);
```
