# Moxy

Derive macros that eliminate Rust boilerplate. Get `Display` and `Deref` implementations with a single attribute — no hand-written `impl` blocks needed.

## Install

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive"] }
```

## Deref

Derives `std::ops::Deref`, forwarding to the inner field.

```rust
use moxy::derive::Deref;

// Tuple struct — auto-targets the single field
#[derive(Deref)]
struct Email(String);

// Named struct — auto-targets the single field
#[derive(Deref)]
struct Email {
    raw: String,
}

// Multi-field — mark the target with #[moxy(deref)]
#[derive(Deref)]
struct User {
    name: String,
    #[moxy(deref)]
    email: String,
    phone: String,
}

let user = User { name: "John".into(), email: "john@example.com".into(), phone: "".into() };
assert_eq!(user.len(), 16); // delegates to String::len via email
```

## Display

Derives `std::fmt::Display` with multiple output formats.

```rust
use moxy::derive::Display;
```

### Default

```rust
#[derive(Display)]
struct User {
    name: String,
    email: String,
}

// User { name: John, email: john@example.com }
```

### Formats

```rust
// Debug — quotes around values
#[derive(Display)]
#[moxy(display(debug))]
struct User { name: String, email: String }
// User { name: "John", email: "john@example.com" }

// Compact — values only, space-separated
#[derive(Display)]
#[moxy(display(compact))]
struct User { name: String, email: String }
// John john@example.com

// Key-Value — key=value pairs
#[derive(Display)]
#[moxy(display(keyvalue))]
struct User { name: String, email: String }
// name=John email=john@example.com

// Map — anonymous map style
#[derive(Display)]
#[moxy(display(map))]
struct User { name: String, email: String }
// { name: John, email: john@example.com }
```

### Pretty Printing

Add `pretty` to any format for multi-line output.

```rust
#[derive(Display)]
#[moxy(display(debug, pretty))]
struct User {
    name: String,
    email: String,
}

// User {
//     name: "John",
//     email: "john@example.com",
// }
```

### Custom Format Strings

```rust
#[derive(Display)]
#[moxy(display("hi! my name is {name} and my email is {email}"))]
struct User {
    name: String,
    email: String,
}

// hi! my name is John and my email is john@example.com
```

#### Expression Arguments

Use `std::fmt`-style positional arguments with arbitrary expressions.

```rust
#[derive(Display)]
#[moxy(display("{}", self.greeting()))]
struct User { name: String }

impl User {
    fn greeting(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}
// Hello, John!

#[derive(Display)]
#[moxy(display("double: {}", count * 2))]
struct Counter { count: i32 }
// double: 10
```

### JSON (feature `json`)

Serializes fields to JSON. Requires field types to implement `serde::Serialize`.
The user's crate must depend on `serde_json`.

```rust
#[derive(Display, serde::Serialize)]
#[moxy(display(json))]
struct User { name: String, age: i32 }
// {"age":30,"name":"John"}

#[derive(Display, serde::Serialize)]
#[moxy(display(json, pretty))]
struct User { name: String, age: i32 }
// {
//   "age": 30,
//   "name": "John"
// }
```

Tuple structs produce JSON arrays:

```rust
#[derive(Display, serde::Serialize)]
#[moxy(display(json))]
struct Pair(String, i32);
// ["hello",42]
```

### Color (feature `color`)

Adds ANSI colors to output via the `colored` crate. The user's crate must depend on `colored`.
Works as a modifier — combine with any style.

```rust
#[derive(Display)]
#[moxy(display(color))]
struct User { name: String, email: String }
// User { name: John, email: john@example.com }  (with colored names)

// Combine with other styles
#[moxy(display(debug, color))]
#[moxy(display(map, color, pretty))]
#[moxy(display(keyvalue, color))]
```

Color scheme: struct name in cyan/bold, field names in blue.

### Skip Fields

```rust
#[derive(Display)]
struct User {
    name: String,
    #[moxy(display(skip))]
    email: String,
}

// User { name: John }
```

### Aliases

```rust
// Type alias
#[derive(Display)]
#[moxy(display(alias = "Person"))]
struct User {
    name: String,
    email: String,
}
// Person { name: John, email: john@example.com }

// Field alias
#[derive(Display)]
struct User {
    #[moxy(display(alias = "full_name"))]
    name: String,
    email: String,
}
// User { full_name: John, email: john@example.com }
```

### Tuple and Unit Structs

```rust
#[derive(Display)]
struct Pair(String, i32);
// Pair(hello, 42)

#[derive(Display)]
struct Marker;
// Marker
```

## Inspiration

- [ambassador](https://crates.io/crates/ambassador)
- [getset](https://crates.io/crates/getset)
- [bon](https://crates.io/crates/bon)
- [thiserror](https://crates.io/crates/thiserror)
- [derive_more](https://crates.io/crates/derive_more)

## License

MIT
