# Feature Flags

Moxy uses Cargo feature flags to keep optional functionality behind compile-time gates.

## `derive`

> [!IMPORTANT]
> `derive` is always required — no other moxy feature works without it.

Enables the derive macros (`Display`, `Deref`, `Build`). This is required to use any moxy derives.

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive"] }
```

## `json`

> [!NOTE]
> `serde` must be added as a separate dependency — moxy does not re-export it.

Enables the `json` display format, which serializes structs to JSON via `serde_json`. Your crate must also depend on `serde`:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "json"] }
serde = { version = "1", features = ["derive"] }
```

See [JSON](./02-display/04-json.md) for usage.

## `color`

Enables ANSI truecolor output via the `colored` crate.

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "color"] }
```

See [Color](./02-display/05-color.md) for usage.

## `full`

Enables both `json` and `color`:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "full"] }
serde = { version = "1", features = ["derive"] }
```
