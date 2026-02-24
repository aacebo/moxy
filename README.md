# Moxy

A set of helpful macros for day to day quality of life tasks including

- Errors
- Builders
- Field Accessors
- Popular Trait Derives
    - Display
    - etc...

## Options

| **Key**       | **Description**                        |
|---------------|----------------------------------------|
| `get`         | add getter method for field            |
| `get_mut`     | add mutable getter method for field    |
| `set`         | add setter method for field            |
| `build`       | add builder method for field           |
| `forward`     | lifts methods from field type to yours |

## Examples

```rust
#[derive(Debug, Display, Clone, Moxy)]
struct User {
    #[moxy(get, get_mut, set, build, forward)]
    name: String,
}
```

## Inspiration

- `ambassador`
- `getset`
- `bon`
- `thiserror`
- `derive_more`