# Deref

The `Deref` derive macro implements `std::ops::Deref`, forwarding to an inner field. This is useful for the newtype pattern — wrapping a type while exposing its methods directly.

## Single-Field Structs

For structs with one field, `Deref` automatically targets that field. No attribute needed.

### Tuple Struct

```rust,ignore
use moxy::derive::Deref;

#[derive(Deref)]
struct Email(String);

let email = Email("john@example.com".into());
assert_eq!(email.len(), 16); // delegates to String::len
```

### Named Struct

```rust,ignore
#[derive(Deref)]
struct Email {
    raw: String,
}

let email = Email { raw: "john@example.com".into() };
assert_eq!(email.len(), 16);
```

## Multi-Field Structs

When a struct has multiple fields, mark the deref target with `#[moxy(deref)]`:

```rust,ignore
#[derive(Deref)]
struct User {
    name: String,
    #[moxy(deref)]
    email: String,
    phone: String,
}

let user = User {
    name: "John".into(),
    email: "john@example.com".into(),
    phone: "".into(),
};

assert_eq!(user.len(), 16); // delegates to email.len()
```

Without `#[moxy(deref)]` on a multi-field struct, the macro will produce a compile error asking you to specify which field to target.

## Use Cases

The `Deref` derive is ideal for the newtype pattern:

```rust,ignore
#[derive(Deref)]
struct Username(String);

#[derive(Deref)]
struct Port(u16);

let name = Username("alice".into());
let port = Port(8080);

// Access all String methods on Username
assert!(name.starts_with("ali"));
assert_eq!(name.to_uppercase(), "ALICE");

// Access all u16 methods on Port
assert_eq!(port.leading_zeros(), 3);
```
