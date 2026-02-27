# Ecosystem Comparison

How moxy's derive macros compare to their closest ecosystem equivalents.

## Display — moxy vs derive_more

| Feature | moxy | derive_more |
|---------|------|-------------|
| Pre-built formats | 5 modes (default, debug, compact, keyvalue, map) | None — manual format strings only |
| Custom format strings | `#[moxy(display("hi {name}"))]` — direct field names | `#[display(fmt = "...", _0)]` — positional refs |
| Field skip | `#[moxy(display(skip))]` | No |
| Field/struct alias | `#[moxy(display(alias = "..."))]` | No |
| Pretty printing | `#[moxy(display(pretty))]` modifier | No (manual `\n`) |
| Color themes | 3 built-in (dracula, atom-one-dark, github-dark) | No |
| JSON output | Feature-gated `#[moxy(display(json))]` | No |
| Enum support | No | Yes |
| Other fmt traits (Binary, Octal, Hex) | No (Display only) | Yes |
| Custom trait bounds | No | `#[display(bound = "...")]` |

Moxy is far richer for struct Display (formats, colors, JSON, skip/alias, pretty). derive_more wins on enum support and coverage of non-Display fmt traits.

## Deref — moxy vs derive_more

| Feature | moxy | derive_more |
|---------|------|-------------|
| Single-field auto-detect | Yes | Yes |
| Multi-field selection | `#[moxy(deref)]` | `#[deref]` |
| Forwarded deref | No | `#[deref(forward)]` |
| DerefMut | No | No |
| Enum support | No | No |

Nearly identical. derive_more has a `forward` mode that delegates through a layer of indirection. Neither supports `DerefMut`.

## Default — moxy vs smart-default vs derivative

| Feature | moxy | smart-default | derivative |
|---------|------|---------------|-----------|
| Syntax | `#[moxy(default = expr)]` | `#[default = val]` / `#[default(expr)]` / `#[default(_code = "...")]` | `#[derivative(Default(value="expr"))]` |
| `.into()` coercion | All values | String literals only | No |
| Enum support | No | Yes | Yes |
| Custom bounds | No | No | `bound=""` |
| `new()` method | No | No | `new="true"` |
| Derive naming | Shadows std `Default` | Separate `SmartDefault` | Separate `Derivative` |

Moxy has the cleanest syntax and universal `.into()`. The main gap is enum support.

## Build — moxy vs bon

| Feature | moxy | bon |
|---------|------|-----|
| Compile-time safety | Const generic bools | Typestate with marker types + traits |
| Missing field error | Compile error (method not found) | Compile error (trait bound) |
| Double-set prevention | Yes (setter consumed) | Yes (`IsUnset` bound) |
| `Option<T>` handling | Auto-optional, setter accepts `T` | Auto-optional, generates `maybe_` setter |
| `Into<T>` coercion | Always (all setters) | Opt-in per field (`#[builder(into)]`) |
| Defaults | `#[moxy(build(default = expr))]` | `#[builder(default = expr)]` |
| Custom names | `#[moxy(build("name"))]` | `#[builder(name = name)]` |
| Function builders | No | Yes |
| Method builders | No | Yes |
| Builder field access | No | `#[builder(field)]` |
| Getters | No | `#[builder(getter)]` |

Moxy's const generic approach is simpler (no extra types/traits), has universal `Into`, and covers the struct builder case well. Bon is more powerful with function/method builders and a richer attribute set.

## Get/Set — moxy vs getset vs derive-getters

| Feature | moxy | getset | derive-getters |
|---------|------|--------|----------------|
| Opt-in per field | Yes (`#[moxy(get)]`) | No (all fields, skip to exclude) | No (all fields) |
| Deref::Target returns | Yes (`String` → `&str`, `Option<String>` → `Option<&str>`) | No (`&String`, `&Option<T>`) | No (`&String`) |
| `Into<T>` setters | Yes | No (direct `T`) | N/A |
| Copy/Clone modifiers | Per-field (`copy`, `clone`) | Separate derives | No |
| Mut getter | `get(mutable)` modifier | Separate derive | No |
| Setter chaining | `&mut Self` | `&mut Self` | N/A |
| Doc forwarding | Yes (`///` → method) | No | Yes |
| Transform callbacks | `on = expr` (get: side effect, set: transform) | No | No |
| Derive count | 2 (Get, Set) | 6 | 1 |

Moxy consolidates 6 getset derives into 2, with per-field modifiers, smarter Option handling, Into coercion on setters, and transform callbacks — features no competing crate offers.

## Cross-cutting gaps

The consistent gap across all six macros is **enum support**. The second most impactful addition would be **DerefMut** for the Deref macro.
