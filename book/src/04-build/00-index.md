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

## What's Next

- [Setters](./01-setters.md) — `V: Into<T>` setter pattern, partial annotation, required vs optional fields
- [Defaults](./02-defaults.md) — inline fallback values with `default = <expr>`
- [Custom Names](./03-custom-names.md) — override the generated setter method name
- [Generics](./04-generics.md) — using `Build` with generic structs
