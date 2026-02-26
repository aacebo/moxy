# Introduction

Moxy is a Rust derive macro crate that eliminates boilerplate. Get `Display` and `Deref` implementations with a single attribute — no hand-written `impl` blocks needed.

## What You Get

- **[Display](./02-display/00-index.md)** — Flexible `std::fmt::Display` with multiple output formats, JSON serialization, and colored terminal output.
- **[Deref](./03-deref/00-index.md)** — Automatic `std::ops::Deref` delegation to inner fields.

## Quick Example

```rust,ignore
use moxy::derive::{Deref, Display};

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
