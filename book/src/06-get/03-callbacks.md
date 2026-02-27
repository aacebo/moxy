# Callbacks

Use `on = expr` to run an expression before the getter returns. The expression has access to `&self`.

```rust
use moxy::Get;

static mut ACCESS_COUNT: u32 = 0;

#[derive(Get)]
struct Tracked {
    #[moxy(get(on = unsafe { ACCESS_COUNT += 1 }))]
    value: String,
}

let t = Tracked { value: "hello".into() };

unsafe { ACCESS_COUNT = 0 };
assert_eq!(t.value(), "hello");
assert_eq!(unsafe { ACCESS_COUNT }, 1);
```

## Doc Forwarding

`///` comments on fields are forwarded to the generated getter method:

```rust
# use moxy::Get;
#
#[derive(Get)]
struct User {
    /// The user's display name
    #[moxy(get)]
    name: String,
}

// Generated method has the doc comment:
// /// The user's display name
// pub fn name(&self) -> &str { &self.name }
```
