# Fields

Field-level attributes let you control which fields appear in the output and how they're labeled.

## Skip

Exclude a field from the display output with `#[moxy(display(skip))]`:

```rust
use moxy::Display;

#[derive(Display)]
struct User {
    name: String,
    #[moxy(display(skip))]
    email: String,
}

let user = User {
    name: "John".into(),
    email: "john@example.com".into(),
};

assert_eq!(format!("{user}"), "User { name: John }");
```

Skip works with all formats — the field is omitted from the output regardless of the display mode.

## Aliases

### Field Alias

Rename a field in the output with `#[moxy(display(alias = "..."))]`:

```rust
# use moxy::Display;
#
#[derive(Display)]
struct User {
    #[moxy(display(alias = "full_name"))]
    name: String,
    email: String,
}

let user = User {
    name: "John".into(),
    email: "john@example.com".into(),
};

assert_eq!(
    format!("{user}"),
    "User { full_name: John, email: john@example.com }"
);
```

### Struct Alias

Rename the struct itself in the output with a struct-level alias:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display(alias = "Person"))]
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
    "Person { name: John, email: john@example.com }"
);
```

### Combining Aliases

Struct and field aliases can be used together, and they work with any format:

```rust
# use moxy::Display;
#
#[derive(Display)]
#[moxy(display(debug, alias = "U"))]
struct User {
    #[moxy(display(alias = "n"))]
    name: String,
    email: String,
}

let user = User {
    name: "John".into(),
    email: "john@example.com".into(),
};

assert_eq!(
    format!("{user}"),
    "U { n: \"John\", email: \"john@example.com\" }"
);
```
