# Generics

`Build` works with generic structs. Type parameters and where-clause bounds are forwarded to the generated builder type unchanged.

## Basic Generic Struct

```rust
use moxy::derive::Build;

#[derive(Build, Default)]
struct Wrapper<T: Default> {
    #[moxy(build)]
    pub value: T,
}

let w: Wrapper<u32> = Wrapper::new().value(42u32).build();
assert_eq!(w.value, 42u32);
```

The generated builder is `WrapperBuilder<T: Default>` and `Wrapper::new()` returns `WrapperBuilder<T>`. All generics are inferred from the call site.

## With Defaults

Generic fields can have defaults too, as long as the default expression is compatible with the type parameter's bounds:

```rust
# use moxy::derive::Build;
#
#[derive(Build, Default)]
struct Container<T: Default + Clone> {
    #[moxy(build)]
    pub value: T,
    pub label: String,   // not in builder — receives Default::default()
}

let c: Container<i32> = Container::new().value(99).build();
assert_eq!(c.value, 99);
assert_eq!(c.label, "");
```
