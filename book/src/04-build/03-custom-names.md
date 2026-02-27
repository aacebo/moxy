# Custom Names

By default the builder setter is named after the struct field. Pass a string literal as the first argument to `build(...)` to override it.

## Renaming a Setter

```rust
use moxy::Build;

#[derive(Build, Default)]
struct Credentials {
    #[moxy(build("username"))]
    pub user: String,
    #[moxy(build)]
    pub password: String,
}

let c = Credentials::new()
    .username("alice")   // setter is `username`, field is `user`
    .password("secret")
    .build();

assert_eq!(c.user, "alice");
```

The field name in the struct (`user`) and the generated setter name (`username`) are independent — the struct field is always assigned correctly.

## Combining with a Default

A custom name and a `default` value can be used together:

```rust
# use moxy::Build;
#
#[derive(Build, Default)]
struct Service {
    #[moxy(build("addr", default = "0.0.0.0"))]
    pub address: String,
}

// Field is optional — falls back to "0.0.0.0"
let s = Service::new().build();
assert_eq!(s.address, "0.0.0.0");

// Override via the custom setter name
let s = Service::new().addr("127.0.0.1").build();
assert_eq!(s.address, "127.0.0.1");
```
