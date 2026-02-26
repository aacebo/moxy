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

Modifiers can be combined: `#[moxy(display(debug, pretty, color))]`

## Display — Field Level

| Attribute | Description | Example |
|-----------|-------------|---------|
| `display(skip)` | Exclude field from output | `#[moxy(display(skip))]` |
| `display(alias = "name")` | Rename field in output | `#[moxy(display(alias = "full_name"))]` |

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
