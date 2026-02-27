# Attribute Reference

All moxy attributes use the `#[moxy(...)]` syntax. This page is a quick reference for every available attribute.

## Display — Struct Level

| Attribute | Description | Example |
|-----------|-------------|---------|
| `display(debug)` | Quote string values | `#[moxy(display(debug))]` |
| `display(compact)` | Values only, space-separated | `#[moxy(display(compact))]` |
| `display(keyvalue)` | `key=value` pairs | `#[moxy(display(keyvalue))]` |
| `display(map)` | Map style without type name | `#[moxy(display(map))]` |
| `display(json)` | JSON serialization (requires `json` feature) | `#[moxy(display(json))]` |
| `display(pretty)` | Multi-line output (modifier) | `#[moxy(display(pretty))]` |
| `display(color)` | Colored output with default theme (requires `color` feature) | `#[moxy(display(color))]` |
| `display(color = "theme")` | Colored output with named theme | `#[moxy(display(color = "dracula"))]` |
| `display(alias = "name")` | Rename the type in output | `#[moxy(display(alias = "Person"))]` |
| `display("fmt", exprs...)` | Custom format string | `#[moxy(display("{}", self.name))]` |

Modifiers can be combined in a single attribute: `#[moxy(display(debug, pretty, color))]`

They can also be split across multiple `#[moxy(...)]` attributes — arguments are merged automatically:

```rust
#[moxy(display(debug))]
#[moxy(display(pretty))]    // equivalent to #[moxy(display(debug, pretty))]
```

Specifying the same option twice with different values is a compile error:

```rust
#[moxy(display(alias = "A"))]
#[moxy(display(alias = "B"))]  // error: conflicting values for `alias`

#[moxy(display(compact))]
#[moxy(display(debug))]        // error: conflicting display styles
```

## Display — Field Level

| Attribute | Description | Example |
|-----------|-------------|---------|
| `display(skip)` | Exclude field from output | `#[moxy(display(skip))]` |
| `display(alias = "name")` | Rename field in output | `#[moxy(display(alias = "full_name"))]` |

## Build — Field Level

| Attribute | Description | Example |
|-----------|-------------|---------|
| `build` | Include field in builder (panics if unset at build time) | `#[moxy(build)]` |
| `build("name")` | Include field with a custom setter method name | `#[moxy(build("username"))]` |
| `build(default = expr)` | Include field with a fallback value (optional in builder) | `#[moxy(build(default = 8080u16))]` |
| `build("name", default = expr)` | Custom setter name + default value | `#[moxy(build("port", default = 8080u16))]` |

## Default — Field Level

| Attribute | Description | Example |
|-----------|-------------|---------|
| `default = expr` | Use expression as field's default value (passed through `.into()`) | `#[moxy(default = "localhost")]` |

Supports literals, typed literals, constants, and arbitrary expressions:

```rust
#[moxy(default = "hello")]          // string literal
#[moxy(default = 8080u16)]          // typed literal
#[moxy(default = MAX_RETRIES)]      // constant
#[moxy(default = Vec::new())]       // expression
```

## Deref — Field Level

| Attribute | Description | Example |
|-----------|-------------|---------|
| `deref` | Mark field as deref target (required for multi-field structs) | `#[moxy(deref)]` |

## Color Themes

| Theme Name | Struct Name | Fields | Values | Punctuation |
|------------|-------------|--------|--------|-------------|
| `dracula` (default) | cyan | pink | yellow | white |
| `atom-one-dark` | gold | purple | green | gray |
| `github-dark` | blue | red | light blue | light gray |
