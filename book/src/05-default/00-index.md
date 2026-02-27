# Default

The `Default` derive macro generates an `impl Default` for your struct, with per-field custom default values. Annotate fields with `#[moxy(default = expr)]` to override the standard `Default::default()`.

When imported via `use moxy::derive::Default`, this derive shadows `std`'s built-in `Default` derive. Fields without `#[moxy(default = ...)]` still receive their normal `Default::default()` value.

## Basic Usage

```rust
use moxy::derive::Default;

#[derive(Default)]
struct Config {
    #[moxy(default = "localhost")]
    pub host: String,
    #[moxy(default = 8080u16)]
    pub port: u16,
    pub debug: bool,
}

let config = Config::default();
assert_eq!(config.host, "localhost");
assert_eq!(config.port, 8080);
assert_eq!(config.debug, false); // standard Default::default()
```

## What Gets Generated

The macro generates a standard `impl Default for Config` block. Each annotated field uses its expression (passed through `.into()`), while unannotated fields fall back to `Default::default()`:

```rust,ignore
impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 8080u16.into(),
            debug: Default::default(),
        }
    }
}
```

## When to Use

Use moxy's `Default` instead of std's when you need per-field default values without writing the `impl` block by hand. For structs where every field is fine with its type's `Default::default()`, std's built-in derive is sufficient.

## What's Next

- [Expressions](./01-expressions.md) — literals, constants, method calls, and `Into<T>` coercion
- [Struct Types](./02-structs.md) — named structs, tuple structs, unit structs, and generics
