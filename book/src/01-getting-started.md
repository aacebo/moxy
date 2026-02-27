# Getting Started

## Installation

Add moxy to your `Cargo.toml` with the `derive` feature:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive"] }
```

To enable all optional features:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "full"] }
```

Or pick individual features:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "json", "color"] }
```

See [Feature Flags](./04-features.md) for details on each feature.

## Basic Usage

Import the derives you need from `moxy::derive`:

```rust
use moxy::derive::{Deref, Display};
```

### Display

Add `#[derive(Display)]` to get a `std::fmt::Display` implementation:

```rust
use moxy::derive::Display;

#[derive(Display)]
struct User {
    name: String,
    email: String,
}

let user = User {
    name: "John".into(),
    email: "john@example.com".into(),
};

println!("{user}");
// User { name: John, email: john@example.com }
```

### Deref

Add `#[derive(Deref)]` to delegate `std::ops::Deref` to an inner field:

```rust
use moxy::derive::Deref;

#[derive(Deref)]
struct Email(String);

let email = Email("john@example.com".into());
assert_eq!(email.len(), 16);
```

### Build

Add `#[derive(Build)]` to generate a fluent builder. Annotate each field you want in the builder with `#[moxy(build)]`:

```rust
use moxy::derive::Build;

#[derive(Build, Default)]
struct Config {
    #[moxy(build)]
    pub host: String,
    #[moxy(build(default = 8080u16))]
    pub port: u16,
}

let config = Config::new().host("localhost").build();
assert_eq!(config.host, "localhost");
assert_eq!(config.port, 8080);
```

## Next Steps

- [Display formats](./02-display/01-formats.md) — debug, compact, keyvalue, map
- [Custom format strings](./02-display/03-custom.md) — template syntax with expressions
- [JSON output](./02-display/04-json.md) — serialize to JSON
- [Colored output](./02-display/05-color.md) — ANSI truecolor themes
- [Deref patterns](./03-deref/00-index.md) — tuple, named, and multi-field structs
- [Build defaults](./04-build/02-defaults.md) — inline fallback values with `default = <expr>`
- [Build custom names](./04-build/03-custom-names.md) — rename generated setter methods
