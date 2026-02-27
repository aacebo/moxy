# Backlog

## Derive Macros

- [ ] `PartialEq` — specify fields to use for comparison via `#[moxy(pk)]`
- [ ] Error — derive error types

## Field Attributes

- [ ] `forward` — lift methods from field type to parent type

## Method Attributes

- [ ] `log` — adds call logging to any method

## Display Formats

- [ ] `table` — tabular output

## Type Support

- [ ] Enum support for all derive macros

## Derive Syntax Prototypes

### Get

Two separate derives: `Get` for read access, `Set` for write access.

**Ecosystem reference:** [getset](https://crates.io/crates/getset) (most popular — 6 derives, visibility control), [derive-getters](https://crates.io/crates/derive-getters) (simpler — ref getters + dissolve), [derive_setters](https://crates.io/crates/derive_setters) (consuming setters).

Annotate fields with `#[moxy(get)]` to opt in. Consistent with how Build works.

```rust
#[derive(Get)]
struct User {
    #[moxy(get)]
    name: String,
    #[moxy(get)]
    email: String,
    password_hash: String,
}

let user = User { name: "alice".into(), email: "a@b.com".into(), password_hash: "...".into() };
assert_eq!(user.name(), "alice");       // fn name(&self) -> &str
assert_eq!(user.email(), "a@b.com");    // fn email(&self) -> &str
// user.password_hash() — no annotation, no getter
```

**Bool fields** return `bool` by value (devs name the field `is_active` if they want that convention):

```rust
#[derive(Get)]
struct Flags {
    #[moxy(get)]
    is_active: bool,
    #[moxy(get)]
    verified: bool,
}

assert!(flags.is_active());    // fn is_active(&self) -> bool
assert!(flags.verified());     // fn verified(&self) -> bool
```

**Option\<T\> fields** return `Option<&Deref::Target>` via `.as_deref()`:

```rust
#[derive(Get)]
struct Profile {
    #[moxy(get)]
    name: String,
    #[moxy(get)]
    bio: Option<String>,
}

let bio: Option<&str> = profile.bio();  // fn bio(&self) -> Option<&str>
```

**Modifiers** — `copy`, `clone`, `mutable`:

```rust
#[derive(Get)]
struct Metrics {
    #[moxy(get(copy))]
    count: u32,                          // fn count(&self) -> u32

    #[moxy(get(clone))]
    label: Arc<String>,                  // fn label(&self) -> Arc<String>

    #[moxy(get(mutable))]
    buffer: Vec<u8>,                     // fn buffer(&self) -> &[u8]
                                         // fn buffer_mut(&mut self) -> &mut Vec<u8>
}
```

**Custom name:**

```rust
#[derive(Get)]
struct Row {
    #[moxy(get("id"))]
    row_id: u64,                         // fn id(&self) -> &u64
}
```

**Callback** — `on = expr` runs before returning. Receives `&self` context:

```rust
#[derive(Get)]
struct Metrics {
    #[moxy(get(on = log::debug!("accessed count")))]
    count: u32,
}

// Generated:
// fn count(&self) -> &u32 {
//     log::debug!("accessed count");
//     &self.count
// }
```

**Doc forwarding** — `///` comments on fields are forwarded to generated methods:

```rust
#[derive(Get)]
struct User {
    /// The user's display name
    #[moxy(get)]
    name: String,
}

// Generated:
// /// The user's display name
// fn name(&self) -> &String { &self.name }
```

### Set

Annotate fields with `#[moxy(set)]` to opt in. Uses `Into<T>` (consistent with Build). Returns `&mut Self` for chaining.

```rust
#[derive(Set)]
struct Config {
    #[moxy(set)]
    host: String,
    #[moxy(set)]
    port: u16,
    read_only: bool,
}

let mut cfg = Config { host: String::new(), port: 0, read_only: true };
cfg.set_host("localhost").set_port(8080_u16);
// cfg.set_read_only() — no annotation, no setter

assert_eq!(cfg.host, "localhost");
assert_eq!(cfg.port, 8080);
```

**Option\<T\> fields** — setter accepts `T`, wraps in `Some` (consistent with Build):

```rust
#[derive(Set)]
struct Profile {
    #[moxy(set)]
    bio: Option<String>,
}

profile.set_bio("hello");  // fn set_bio(&mut self, value: impl Into<String>) -> &mut Self
assert_eq!(profile.bio, Some("hello".to_string()));
```

**Callback** — `on = expr` runs before assignment. The incoming value is passed as `value: T`. If the expression returns `T`, the returned value is what gets assigned (transform):

```rust
#[derive(Set)]
struct Config {
    #[moxy(set(on = value.to_lowercase()))]
    host: String,
}

// Generated:
// fn set_host(&mut self, value: impl Into<String>) -> &mut Self {
//     let value: String = value.into();
//     self.host = value.to_lowercase();
//     self
// }

cfg.set_host("LOCALHOST");
assert_eq!(cfg.host, "localhost");
```

Side effects without transforms — return `value` to pass through unchanged:

```rust
#[derive(Set)]
struct Config {
    #[moxy(set(on = { log::info!("host changed to {}", value); value }))]
    host: String,
}

// Generated:
// fn set_host(&mut self, value: impl Into<String>) -> &mut Self {
//     let value: String = value.into();
//     self.host = { log::info!("host changed to {}", value); value };
//     self
// }
```

**Custom name:**

```rust
#[derive(Set)]
struct Row {
    #[moxy(set("update_id"))]
    row_id: u64,                         // fn update_id(&mut self, value: impl Into<u64>) -> &mut Self
}
```

### Get/Set ecosystem comparison

| Feature | moxy Get/Set | getset | derive-getters |
|---------|-------------|--------|----------------|
| Opt-in per field | Yes (`#[moxy(get)]`) | No (all fields, skip to exclude) | No (all fields) |
| Option\<T\> → Option\<&T\> | Yes | No (`&Option<T>`) | No (`&Option<T>`) |
| `Into<T>` setters | Yes | No (direct `T`) | N/A |
| Copy/Clone modifiers | Per-field | Separate derives | No |
| Mut getter | `get(mutable)` modifier | Separate derive | No |
| Setter chaining | `&mut Self` | `&mut Self` | N/A |
| Doc forwarding | Yes (`///` → method) | No | Yes |
| Callbacks | `on = expr` (get: side effect, set: transform) | No | No |
| Derive count | 2 (Get, Set) | 6 | 1 |