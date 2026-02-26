# Struct Types

The `Display` derive works with named structs, tuple structs, and unit structs. Each struct type has its own output style.

## Named Structs

The most common case. Fields are displayed with their names:

```rust
use moxy::derive::Display;

#[derive(Display)]
struct User {
    name: String,
    email: String,
}

// User { name: John, email: john@example.com }
```

All format modes, pretty printing, field attributes, and modifiers work with named structs.

## Tuple Structs

Tuple struct fields are displayed positionally inside parentheses:

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
struct Pair(String, i32);

let pair = Pair("hello".into(), 42);
// Pair(hello, 42)
```

### With Debug

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
#[moxy(display(debug))]
struct Pair(String, i32);

// Pair("hello", 42)
```

### With Compact

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
#[moxy(display(compact))]
struct Pair(String, i32);

// hello 42
```

### With Pretty

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
#[moxy(display(pretty))]
struct Pair(String, i32);

// Pair(
//     hello,
//     42,
// )
```

## Unit Structs

Unit structs display as just their type name:

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
struct Marker;

assert_eq!(format!("{}", Marker), "Marker");
```
