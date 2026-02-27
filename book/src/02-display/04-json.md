# JSON

The `json` format serializes your struct to JSON using `serde_json`. This requires the `json` feature flag.

## Setup

Enable the `json` feature in your `Cargo.toml`:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "json"] }
serde = { version = "1", features = ["derive"] }
```

Your struct must derive both `Display` and `serde::Serialize`:

```rust,ignore
use moxy::Display;

#[derive(Display, serde::Serialize)]
#[moxy(display(json))]
struct User {
    name: String,
    age: i32,
}
```

## Named Structs

Named structs produce JSON objects:

```rust,ignore
#[derive(Display, serde::Serialize)]
#[moxy(display(json))]
struct User {
    name: String,
    age: i32,
}

let user = User { name: "John".into(), age: 30 };
// {"age":30,"name":"John"}
```

## Tuple Structs

Tuple structs produce JSON arrays:

```rust,ignore
#[derive(Display, serde::Serialize)]
#[moxy(display(json))]
struct Pair(String, i32);

let pair = Pair("hello".into(), 42);
// ["hello",42]
```

## Pretty JSON

Combine with `pretty` for indented output:

```rust,ignore
#[derive(Display, serde::Serialize)]
#[moxy(display(json, pretty))]
struct User {
    name: String,
    age: i32,
}

let user = User { name: "John".into(), age: 30 };
// {
//   "age": 30,
//   "name": "John"
// }
```

## Skipping Fields

Use `#[moxy(display(skip))]` to exclude fields from JSON output:

```rust,ignore
#[derive(Display, serde::Serialize)]
#[moxy(display(json))]
struct User {
    name: String,
    #[moxy(display(skip))]
    secret: String,
}

let user = User { name: "John".into(), secret: "hunter2".into() };
// {"name":"John"}
```

## Field Aliases

Field aliases are applied to JSON keys:

```rust,ignore
#[derive(Display, serde::Serialize)]
#[moxy(display(json))]
struct User {
    #[moxy(display(alias = "full_name"))]
    name: String,
}

let user = User { name: "John".into() };
// {"full_name":"John"}
```
