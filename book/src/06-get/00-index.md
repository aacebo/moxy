# Get

The `Get` derive macro generates getter methods for struct fields. Annotate fields with `#[moxy(get)]` to opt in — unannotated fields get no getter.

Getters return through `Deref::Target`, so `String` fields return `&str`, `Vec<T>` returns `&[T]`, etc. Use `copy` for primitives and `clone` for types like `Arc<T>`.

> [!NOTE]
> Getters return `&Deref::Target`, not `&T`. This is more ergonomic for common types (`String` → `&str`, `Vec<T>` → `&[T]`), but for types without a natural `Deref` target, use `get(copy)` or `get(clone)` to get an owned value instead.

## Basic Usage

```rust
use moxy::Get;

#[derive(Get)]
struct User {
    #[moxy(get)]
    name: String,
    #[moxy(get)]
    email: String,
    password_hash: String,
}

let user = User {
    name: "alice".into(),
    email: "a@b.com".into(),
    password_hash: "hash".into(),
};

assert_eq!(user.name(), "alice");    // fn name(&self) -> &str
assert_eq!(user.email(), "a@b.com"); // fn email(&self) -> &str
// user.password_hash() — no annotation, no getter
```

## What's Next

- [Modifiers](./01-modifiers.md) — copy, clone, and mutable variants
- [Option Fields](./02-option.md) — automatic `Option<&str>` via `as_deref()`
- [Callbacks](./03-callbacks.md) — side effects with `on = expr`
