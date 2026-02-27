# Custom Format Strings

Instead of using a built-in format, you can provide a custom format string. Field names are interpolated directly using `{field_name}` syntax.

## Basic Interpolation

Reference fields by name inside curly braces:

```rust
use moxy::Display;

#[derive(Display)]
#[moxy(display("hi! my name is {name} and my email is {email}"))]
struct User {
    name: String,
    email: String,
}

let user = User {
    name: "John".into(),
    email: "john@example.com".into(),
};

assert_eq!(
    format!("{user}"),
    "hi! my name is John and my email is john@example.com"
);
```

## Expression Arguments

Use `std::fmt`-style positional arguments with arbitrary Rust expressions. The format string comes first, followed by comma-separated expressions:

```rust,ignore
#[moxy(display("{}", expr))]
```

### Self Field Access

Access fields through `self`:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display("{}", self.name))]
struct User {
    name: String,
}

// John
```

### Self Method Calls

Call methods on `self`:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display("{}", self.greeting()))]
struct User {
    name: String,
}

impl User {
    fn greeting(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

// Hello, John!
```

### Arithmetic and Other Expressions

Any valid Rust expression works:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display("double: {}", count * 2))]
struct Counter {
    count: i32,
}

let counter = Counter { count: 5 };
assert_eq!(format!("{counter}"), "double: 10");
```
