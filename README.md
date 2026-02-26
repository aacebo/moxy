# Moxy

A set of helpful macros for day to day quality of life tasks including

- Errors
- Builders
- Field Accessors
- Popular Trait Derives
    - Display
    - Deref
    - etc...

## Options

| **Key**       | **Description**                        |
|---------------|----------------------------------------|
| `get`         | add getter method for field            |
| `get_mut`     | add mutable getter method for field    |
| `set`         | add setter method for field            |
| `build`       | add builder method for field           |
| `forward`     | lifts methods from field type to yours |
| `display`     | implement Display with explicit fields |
| `deref`       | implement Deref for field              |

## Examples

```rust
#[derive(Debug, Display, Clone)]
struct User {
    #[moxy(get, get_mut, set, build, forward, display)]
    name: String,
}
```

### Deref

> Tuple struct with 1 field

```rust
#[derive(Deref)]
struct Email(String);
```

> Named struct field

```rust
#[derive(Deref)]
struct Email {
    inner: String,
}
```

> Multi field

```rust
#[derive(Deref)]
struct User {
    name: String,
    #[moxy(deref)]
    email: String,
    phone: String,
}
```

### Display

> Include all fields

```rust
#[derive(Display)]
struct User {
    name: String,
    email: String,
    phone: String,
}
```

> Whitelist fields

```rust
#[derive(Display)]
#[moxy(display(explicit))]
struct User {
    #[moxy(display)]
    name: String,
    email: String,
    phone: String,
}
```

> Type alias

```rust
#[derive(Display)]
#[moxy(alias = "UserDetails")]
struct User {
    name: String,
    email: String,
    phone: String,
}
```

> Custom formats make it easy to fully customize your display
> logic without writing an impl block.

```rust
#[derive(Display)]
#[moxy(display("hi! my name is {name}"))]
struct User {
    name: String,
    email: String,
    phone: String,
}
```

> Premade formats make it easy to display in many ways
> with minimal code.

```rust
#[derive(Display)]
#[moxy(display(debug | compact | keyvalue | map | table | json | json(pretty)))]
struct User {
    name: String,
    email: String,
    phone: String,
}
```

### PartialEq

> Specify keys to use for cmp

```rust
#[derive(PartialEq)]
struct User {
    #[moxy(pk)]
    name: String,
    email: String,
    phone: String,
}
```

## Inspiration

- `ambassador`
- `getset`
- `bon`
- `thiserror`
- `derive_more`