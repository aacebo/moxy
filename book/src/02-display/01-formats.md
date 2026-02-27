# Formats

The `Display` derive supports several built-in format modes. Each is specified as a flag in the `#[moxy(display(...))]` attribute.

## Default

No attribute needed. Produces struct-literal style output with the type name, field names, and unquoted values:

```rust
# use moxy::Display;
#
#[derive(Display)]
struct User {
    name: String,
    email: String,
}

// User { name: John, email: john@example.com }
```

## Debug

Wraps string values in quotes, similar to Rust's `Debug` trait:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display(debug))]
struct User {
    name: String,
    email: String,
}

// User { name: "John", email: "john@example.com" }
```

## Compact

Values only, space-separated. No type name, no field names, no punctuation:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display(compact))]
struct User {
    name: String,
    email: String,
}

// John john@example.com
```

## Key-Value

Field-value pairs separated by `=`, space-delimited:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display(keyvalue))]
struct User {
    name: String,
    email: String,
}

// name=John email=john@example.com
```

## Map

Anonymous map style — like default but without the type name:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display(map))]
struct User {
    name: String,
    email: String,
}

// { name: John, email: john@example.com }
```
