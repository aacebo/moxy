# Build

The `Build` derive macro generates a **type-safe** fluent builder for your struct. Annotate fields with `#[moxy(build)]` to include them in the builder, then call `YourStruct::new()` to get a builder instance.

The builder uses const generic bools to track which required fields have been set. Missing a required field is a **compile error** — not a runtime panic. Setters can be called in any order, and setting a required field twice is also a compile error.

## Basic Usage

```rust
use moxy::Build;

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
- A `ConfigBuilder` struct with const generic bools tracking required field state
- A setter method per annotated field that accepts `V: Into<T>` for flexible conversions
- A `build()` method — only available once all required fields are set
- A `Config::new()` factory that returns a fresh `ConfigBuilder`

Non-annotated fields are initialised with `Default::default()` via a struct spread.

## What's Next

- [Setters](./01-setters.md) — `V: Into<T>` setter pattern, partial annotation, required vs optional fields
- [Defaults](./02-defaults.md) — inline fallback values with `default = <expr>`
- [Custom Names](./03-custom-names.md) — override the generated setter method name
- [Generics](./04-generics.md) — using `Build` with generic structs
