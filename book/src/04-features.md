# Feature Flags

Moxy uses Cargo feature flags to keep optional functionality behind compile-time gates.

## `derive`

Enables the derive macros (`Display`, `Deref`). This is required to use any moxy derives.

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive"] }
```

## `json`

Enables the `json` display format, which serializes structs to JSON via `serde_json`. Your crate must also depend on `serde` and `serde_json`:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

See [JSON](./02-display/04-json.md) for usage.

## `color`

Enables ANSI truecolor output via the `colored` crate. Your crate must also depend on `colored`:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "color"] }
colored = "3"
```

See [Color](./02-display/05-color.md) for usage.

## `full`

Enables both `json` and `color`:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
colored = "3"
```
