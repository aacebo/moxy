# Struct Types

The `Default` derive works with named structs, tuple structs, and unit structs.

## Named Structs

The most common case — fields are referenced by name:

```rust
use moxy::Default;

#[derive(Default)]
struct Config {
    #[moxy(default = "localhost")]
    pub host: String,
    #[moxy(default = 8080u16)]
    pub port: u16,
    pub verbose: bool,
}

let c = Config::default();
assert_eq!(c.host, "localhost");
assert_eq!(c.port, 8080);
assert_eq!(c.verbose, false);
```

## Tuple Structs

Positional fields work the same way:

```rust
# use moxy::Default;
#
#[derive(Default)]
struct Endpoint(
    #[moxy(default = "0.0.0.0")] String,
    #[moxy(default = 3000u16)] u16,
);

let ep = Endpoint::default();
assert_eq!(ep.0, "0.0.0.0");
assert_eq!(ep.1, 3000);
```

## Unit Structs

Unit structs have no fields, so `#[derive(Default)]` just generates `Self`:

```rust
# use moxy::Default;
#
#[derive(Default)]
struct Marker;

let _ = Marker::default();
```

## Generics

Type parameters are propagated through the generated `impl` block. Unannotated fields with generic types require `T: Default`:

```rust
# use moxy::Default;
#
#[derive(Default)]
struct Container<T: Default> {
    #[moxy(default = 10u32)]
    pub capacity: u32,
    pub value: T,
}

let c: Container<String> = Container::default();
assert_eq!(c.capacity, 10);
assert_eq!(c.value, String::default());
```

## With Build

When both `Default` and `Build` are derived, the builder's `..Default::default()` spread picks up moxy's generated defaults automatically:

```rust
use moxy::{Build, Default};

#[derive(Build, Default)]
struct Server {
    #[moxy(build)]
    pub name: String,
    #[moxy(default = "0.0.0.0")]
    pub bind: String,
    #[moxy(default = 8080u16)]
    pub port: u16,
}

// Builder only requires `name` — bind and port come from Default
let s = Server::new().name("api").build();
assert_eq!(s.name, "api");
assert_eq!(s.bind, "0.0.0.0");
assert_eq!(s.port, 8080);
```
