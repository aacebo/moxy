# Display

The `Display` derive macro implements `std::fmt::Display` for your structs. It supports multiple output formats, custom format strings, JSON serialization, and colored terminal output.

## Default Format

Without any attributes, `Display` produces a struct-literal style output:

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

assert_eq!(format!("{user}"), "User { name: John, email: john@example.com }");
```

## Attribute Syntax

Display behavior is controlled through `#[moxy(display(...))]` attributes at the struct level and field level:

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
#[moxy(display(debug, pretty))]      // struct-level: format + modifiers
struct User {
    name: String,
    #[moxy(display(skip))]            // field-level: skip this field
    password: String,
}
```

Multiple `#[moxy(...)]` attributes on the same item are merged, so these two forms are equivalent:

```rust
#[moxy(display(debug, pretty))]

// same as:
#[moxy(display(debug))]
#[moxy(display(pretty))]
```

## What's Next

- [Formats](./01-formats.md) — debug, compact, keyvalue, map
- [Pretty Printing](./02-pretty.md) — multi-line output
- [Custom Format Strings](./03-custom.md) — template syntax with expressions
- [JSON](./04-json.md) — JSON serialization
- [Color](./05-color.md) — ANSI colored output with themes
- [Fields](./06-fields.md) — skip and alias fields
- [Struct Types](./07-structs.md) — named, tuple, and unit structs
