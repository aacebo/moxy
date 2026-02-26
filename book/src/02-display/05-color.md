# Color

The `color` modifier adds ANSI truecolor (24-bit RGB) output via the `colored` crate. It works as a modifier — combine it with any display format.

## Setup

Enable the `color` feature in your `Cargo.toml`:

```toml
[dependencies]
moxy = { version = "0.0.0", features = ["derive", "color"] }
colored = "3"
```

## Basic Usage

Use `color` for the default theme (Dracula):

```rust,ignore
use moxy::derive::Display;

#[derive(Display)]
#[moxy(display(color))]
struct User {
    name: String,
    email: String,
}
```

## Themes

Specify a theme by name:

```rust,ignore
#[derive(Display)]
#[moxy(display(color = "dracula"))]
struct User { name: String, email: String }

#[derive(Display)]
#[moxy(display(color = "atom-one-dark"))]
struct Config { host: String, port: u16 }

#[derive(Display)]
#[moxy(display(color = "github-dark"))]
struct Status { code: u16, message: String }
```

### Theme Colors

Each theme colorizes four elements: the struct name, field names, values, and punctuation (braces, colons, commas).

| Theme | Struct Name | Field Names | Values | Punctuation |
|-------|-------------|-------------|--------|-------------|
| `dracula` (default) | cyan | pink | yellow | white |
| `atom-one-dark` | gold | purple | green | gray |
| `github-dark` | blue | red | light blue | light gray |

## Combining with Formats

Color works with any display format:

```rust,ignore
// Default + color
#[moxy(display(color))]

// Debug + color
#[moxy(display(debug, color))]

// Map + color
#[moxy(display(map, color))]

// Key-value + color with a specific theme
#[moxy(display(keyvalue, color = "github-dark"))]

// Color + pretty
#[moxy(display(color, pretty))]

// Debug + color + pretty
#[moxy(display(debug, color, pretty))]
```

## Tuple Structs

Color works with tuple structs:

```rust,ignore
#[derive(Display)]
#[moxy(display(color))]
struct Pair(String, i32);
```
