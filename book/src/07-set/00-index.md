# Set

The `Set` derive macro generates setter methods for struct fields. Annotate fields with `#[moxy(set)]` to opt in — unannotated fields get no setter.

Setters use `Into<T>` for flexible type coercion (consistent with Build) and return `&mut Self` for chaining.

## Basic Usage

```rust
use moxy::Set;

#[derive(Set)]
struct Config {
    #[moxy(set)]
    host: String,
    #[moxy(set)]
    port: u16,
    read_only: bool,
}

let mut cfg = Config { host: String::new(), port: 0, read_only: true };

// &str → String via Into, chaining via &mut Self
cfg.set_host("localhost").set_port(8080_u16);

assert_eq!(cfg.host, "localhost");
assert_eq!(cfg.port, 8080);
// cfg.set_read_only() — no annotation, no setter
```

## What's Next

- [Option Fields](./01-option.md) — automatic `Some` wrapping for `Option<T>` fields
- [Callbacks](./02-callbacks.md) — transforms and side effects with `on = expr`
