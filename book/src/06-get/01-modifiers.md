# Modifiers

## Copy

For `Copy` types and primitives, use `get(copy)` to return by value instead of by reference:

```rust
use moxy::Get;

#[derive(Get)]
struct Metrics {
    #[moxy(get(copy))]
    count: u32,
    #[moxy(get)]
    is_active: bool,
}

let m = Metrics { count: 42, is_active: true };
let c: u32 = m.count();    // fn count(&self) -> u32
assert!(m.is_active());    // fn is_active(&self) -> bool (bool auto-copies)
```

Bool fields automatically return by value — no `copy` modifier needed.

## Clone

For types where you want an owned copy via `.clone()`:

```rust
# use moxy::Get;
use std::sync::Arc;

#[derive(Get)]
struct Labels {
    #[moxy(get(clone))]
    label: Arc<String>,
}

let l = Labels { label: Arc::new("test".into()) };
let v: Arc<String> = l.label();  // fn label(&self) -> Arc<String>
```

## Mutable

Use `get(mutable)` to generate both a regular getter and a `_mut` variant:

```rust
# use moxy::Get;
#
#[derive(Get)]
struct Buffer {
    #[moxy(get(mutable))]
    data: Vec<u8>,
}

let mut b = Buffer { data: vec![1, 2, 3] };
assert_eq!(b.data(), &[1, 2, 3]);  // fn data(&self) -> &[u8]
b.data_mut().push(4);              // fn data_mut(&mut self) -> &mut Vec<u8>
assert_eq!(b.data(), &[1, 2, 3, 4]);
```

## Custom Name

Override the method name with a string literal:

```rust
# use moxy::Get;
#
#[derive(Get)]
struct Row {
    #[moxy(get(copy, "id"))]
    row_id: u64,
}

let r = Row { row_id: 99 };
assert_eq!(r.id(), 99);  // fn id(&self) -> u64
```

Modifiers can be combined: `#[moxy(get(copy, "id"))]`.
