# Callbacks

Use `on = expr` to transform the value before assignment. The expression receives `value: T` (already converted via `Into`) and its return value is what gets assigned.

## Transform

The callback expression replaces the assigned value:

```rust
use moxy::Set;

#[derive(Set)]
struct Config {
    #[moxy(set(on = value.to_lowercase()))]
    host: String,
}

let mut cfg = Config { host: String::new() };
cfg.set_host("LOCALHOST");
assert_eq!(cfg.host, "localhost");
```

Generated:

```rust,ignore
fn set_host<V: Into<String>>(&mut self, value: V) -> &mut Self {
    let value: String = value.into();
    self.host = value.to_lowercase();
    self
}
```

## Side Effects

For side effects without transforming the value, return `value` from a block:

```rust
# use moxy::Set;
#
#[derive(Set)]
struct Config {
    #[moxy(set(on = { println!("host changed to {}", value); value }))]
    host: String,
}
```

## Custom Name

Override the setter name with a string literal:

```rust
# use moxy::Set;
#
#[derive(Set)]
struct Row {
    #[moxy(set("update_id"))]
    row_id: u64,
}

let mut r = Row { row_id: 0 };
r.update_id(99_u64);
assert_eq!(r.row_id, 99);
```

## Doc Forwarding

`///` comments on fields are forwarded to the generated setter method, just like with Get.
