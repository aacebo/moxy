# Backlog

## Derive Macros

- [ ] `PartialEq` — specify fields to use for comparison via `#[moxy(pk)]`
- [ ] `Error` — derive error types
- [ ] `Delegate` — delegate trait implementations to a field

## Field Attributes

- [ ] `forward` — lift methods from field type to parent type

## Method Attributes

- [ ] `log` — adds call logging to any method

## Display Formats

- [ ] `table` — tabular output

## Type Support

- [ ] Enum support for all derive macros

## Derive Syntax Prototypes

### Delegate

Delegate a trait implementation to a named field. Annotate fields with `#[moxy(delegate(Trait))]`
to opt in — the macro generates a complete `impl Trait for Struct` that forwards every method
call to that field.

**Ecosystem reference:** [delegate](https://crates.io/crates/delegate) (macro-based, method-level
forwarding), [ambassador](https://crates.io/crates/ambassador) (attribute-based, whole-trait
delegation — closest analogue).

```rust
use moxy::Delegate;

trait Greet {
    fn hello(&self) -> String;
    fn goodbye(&self) -> String;
}

struct Inner;
impl Greet for Inner {
    fn hello(&self) -> String { "Hello!".to_string() }
    fn goodbye(&self) -> String { "Goodbye!".to_string() }
}

#[derive(Delegate)]
struct Wrapper {
    #[moxy(delegate(Greet))]
    inner: Inner,
    label: String,
}

// Generated:
// impl Greet for Wrapper {
//     fn hello(&self) -> String { self.inner.hello() }
//     fn goodbye(&self) -> String { self.inner.goodbye() }
// }

let w = Wrapper { inner: Inner, label: "test".into() };
assert_eq!(w.hello(), "Hello!");
assert_eq!(w.goodbye(), "Goodbye!");
// label is unrelated — Greet is fully covered by delegation
```

**Multiple traits from one field:**

```rust
#[derive(Delegate)]
struct Wrapper {
    #[moxy(delegate(Greet, Farewell))]
    inner: Inner,
}
```

**Different traits delegated to different fields:**

```rust
#[derive(Delegate)]
struct App {
    #[moxy(delegate(Greet))]
    greeter: Greeter,
    #[moxy(delegate(Logger))]
    logger: Logger,
}

// Generated:
// impl Greet for App { ... forwarded to self.greeter ... }
// impl Logger for App { ... forwarded to self.logger ... }
```

**Generics — where bounds are inferred automatically:**

```rust
#[derive(Delegate)]
struct Wrapper<T> {
    #[moxy(delegate(Greet))]
    inner: T,
}

// Generated:
// impl<T: Greet> Greet for Wrapper<T> {
//     fn hello(&self) -> String { self.inner.hello() }
//     fn goodbye(&self) -> String { self.inner.goodbye() }
// }
```

The macro adds `FieldType: Trait` to the generated `impl` where clause automatically. Struct-level
where clauses are forwarded as-is.

**Traits with associated types:**

```rust
#[derive(Delegate)]
struct StreamWrapper<S: Stream> {
    #[moxy(delegate(Stream))]
    inner: S,
}

// Generated:
// impl<S: Stream> Stream for StreamWrapper<S> {
//     type Item = S::Item;
//     fn next(&mut self) -> Option<Self::Item> { self.inner.next() }
// }
```

Associated types are resolved as `<FieldType as Trait>::AssocType` in the generated impl.

**Limitations:**

- The trait and its implementor must be in scope at the delegation site (no auto-imports generated)
- Delegating the same trait twice on different fields is a compile error (duplicate `impl`)
- Traits with `Self`-returning methods (e.g. `Clone`) require `T: Clone` — the generated call
  returns `self.inner.clone()`, which gives the inner type, not `Self`. These traits may need
  a manual `impl` instead.