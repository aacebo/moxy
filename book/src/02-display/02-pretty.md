# Pretty Printing

Add `pretty` to any format for multi-line output with indentation. It works as a modifier — combine it with any display format.

## Default + Pretty

```rust,ignore
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

```rust,ignore
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

```rust,ignore
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

```rust,ignore
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

```rust,ignore
#[derive(Display)]
#[moxy(display(pretty))]
struct Pair(String, i32);

// Pair(
//     hello,
//     42,
// )
```

```rust,ignore
#[derive(Display)]
#[moxy(display(debug, pretty))]
struct Pair(String, i32);

// Pair(
//     "hello",
//     42,
// )
```
