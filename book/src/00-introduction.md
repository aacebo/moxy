![moxy](https://raw.githubusercontent.com/aacebo/moxy/master/assets/banner.svg)

# Introduction

Moxy is a Rust derive macro crate that eliminates boilerplate. Get `Display`, `Deref`, `Default`, `Build`, `Get`, and `Set` implementations with a single attribute — no hand-written `impl` blocks needed.

## What You Get

- **[Display](./02-display/00-index.md)** — Flexible `std::fmt::Display` with multiple output formats, JSON serialization, and colored terminal output.
- **[Deref](./03-deref/00-index.md)** — Automatic `std::ops::Deref` delegation to inner fields.
- **[Build](./04-build/00-index.md)** — Type-safe fluent builder with compile-time field tracking, `V: Into<T>` setters, and inline defaults.
- **[Default](./05-default/00-index.md)** — `std::default::Default` with per-field custom expressions via `#[moxy(default = expr)]`.
- **[Get](./06-get/00-index.md)** — Field getters returning `Deref::Target` (e.g. `String` → `&str`), with copy/clone/mutable modifiers and callbacks.
- **[Set](./07-set/00-index.md)** — Field setters with `Into<T>` coercion, `Option<T>` auto-wrapping, transform callbacks, and chaining.

## Quick Example

```rust
use moxy::{Deref, Display};

#[derive(Deref, Display)]
#[moxy(display(debug, pretty))]
struct User {
    #[moxy(deref)]
    name: String,
    email: String,
}

let user = User {
    name: "John".into(),
    email: "john@example.com".into(),
};

// Display output:
// User {
//     name: "John",
//     email: "john@example.com",
// }

// Deref delegates to name:
assert_eq!(user.len(), 4);
```

## Inspiration

Moxy draws ideas from these excellent crates:

- [ambassador](https://crates.io/crates/ambassador)
- [getset](https://crates.io/crates/getset)
- [bon](https://crates.io/crates/bon)
- [thiserror](https://crates.io/crates/thiserror)
- [derive_more](https://crates.io/crates/derive_more)

## License

MIT
