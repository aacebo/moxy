# Pretty Printing

Add `pretty` to any format for multi-line output with indentation. It works as a modifier — combine it with default, debug, keyvalue, map, and json formats. Compact mode does not support pretty printing.

## Default + Pretty

```rust
use moxy::derive::Display;

#[derive(Display)]
#[moxy(display(pretty))]
struct User {
    name: String,
    email: String,
}

let user = User {
    name: "John".into(),
    email: "john@example.com".into(),
};

// User {
//     name: John,
//     email: john@example.com,
// }
```

## Debug + Pretty

```rust
# use moxy::derive::Display;
#
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

## Key-Value + Pretty

One pair per line, no indentation:

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
#[moxy(display(keyvalue, pretty))]
struct User {
    name: String,
    email: String,
}

// name=John
// email=john@example.com
```

## Map + Pretty

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
#[moxy(display(map, pretty))]
struct User {
    name: String,
    email: String,
}

// {
//     name: John,
//     email: john@example.com,
// }
```

## Tuple Structs

Pretty printing works with tuple structs too:

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

```rust
# use moxy::derive::Display;
#
#[derive(Display)]
#[moxy(display(debug, pretty))]
struct Pair(String, i32);

// Pair(
//     "hello",
//     42,
// )
```
