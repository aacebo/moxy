# `moxy::ast` ↔ `syn` parse audit

I audited the current `moxy-ast` parser architecture against current `syn`, focusing on **AST nodes that exist but cannot be produced, duplicate grammar represented by multiple nodes, parser/AST disagreement, token loss, and grammar boundaries**.

The result: **`PatType` is not an isolated problem.** There are several instances of the same architectural failure mode.

## Executive summary

| Severity | Finding                                                                                               |                                    |
| -------- | ----------------------------------------------------------------------------------------------------- | ---------------------------------- |
| 🔴 P0    | Top-level `parse!` does **not require EOF**, so prefix parses silently succeed                        |                                    |
| 🔴 P0    | `PatType` exists but is unreachable; typed-pattern syntax is duplicated 3 different ways              |                                    |
| 🔴 P0    | `PatParen` exists but `(x)` is always parsed as `PatTuple`                                            |                                    |
| 🔴 P0    | `PatRange` exists but is never constructed by the pattern parser                                      |                                    |
| 🔴 P0    | `Pattern::Macro` exists but the pattern parser never constructs it                                    |                                    |
| 🔴 P0    | Qualified patterns beginning `<...>` are unsupported                                                  |                                    |
| 🔴 P1    | Function typed params use `Pattern`, not `parse_single`, allowing grammar `syn` intentionally rejects |                                    |
| 🔴 P1    | `TypeBareFn.variadic` can never become `Some`                                                         |                                    |
| 🔴 P1    | `TypeTraitObject { dyn_token: None }` is representable but unreachable through `Type`                 |                                    |
| 🔴 P1    | `TypeGroup` prints **nothing**, including dropping its inner type                                     |                                    |
| 🔴 P1    | `GenericParam` incorrectly dispatches attributed lifetime parameters                                  |                                    |
| 🔴 P1    | `ItemStruct` silently ignores a required semicolon parse failure                                      |                                    |
| 🟠 P1    | `Generics` owns/parses `where` clauses at the wrong grammatical layer                                 |                                    |
| 🟠 P2    | Generic `<` / `>` spans are discarded and replaced with default tokens                                |                                    |
| 🟠 P2    | Lifetime parameter `:` span is discarded and fabricated                                               |                                    |
| 🟠 P2    | Leading `                                                                                             | `in`PatOr` is parsed but discarded |
| 🟠 P2    | Pattern attributes are lost for several unit/wrapper variants                                         |                                    |
| 🟠 P2    | Moxy type nodes systematically lack `syn`'s node-level attributes                                     |                                    |
| 🟠 P2    | `ExprType` appears to be stale/dead AST from obsolete expression type-ascription                      |                                    |
| 🟠 P2    | `Expr::Infer` has no token payload and `ToTokens` emits nothing                                       |                                    |

The **first thing I would fix is actually not `PatType`: it is EOF enforcement.** That bug is currently hiding parser mistakes like `PatType`.

---

# 1. 🔴 Top-level parsing accepts partial input

This is the most important finding.

Moxy's `parse!` effectively does:

```rust
<$ty as Parse>::parse(&mut ts.parse())
```

and then returns success. There is **no assertion that the stream was fully consumed**.

Consequently:

```rust
parse!("x: i32" as Pattern)
```

can successfully return approximately:

```rust
Pattern::Ident(x)
```

while leaving:

```text
: i32
```

unconsumed.

This means entire classes of missing parser behavior can remain invisible.

`syn`'s equivalent top-level parsing requires complete consumption. Its parser architecture distinguishes parsing a prefix internally from parsing a complete syntax object.

### Fix

Your public parse entrypoint should essentially be:

```rust
let mut stream = ts.parse();
let value = T::parse(&mut stream)?;

if !stream.is_empty() {
    return Err(...);
}

Ok(value)
```

Internal recursive parsers should still be allowed to leave tokens behind.

**Priority: P0. Fix this first.**

---

# 2. 🔴 `PatType` is dead, and typed patterns are fragmented

As we already found, Moxy exposes:

```rust
Pattern::Type(PatType)
```

but `Pattern::parse_single` never creates it.

`PatType` itself has no `Parse` implementation.

`syn` explicitly defines:

```rust
impl Parse for PatType {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::parse_single(input)?),
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}
```

But Moxy has **three alternative representations** for this syntax.

### Function parameters

```rust
pub struct TypedParam {
    attrs,
    pat,
    colon,
    ty,
}
```

### Closures

```rust
pub enum ClosureParam {
    Typed { pat, colon, ty },
    Inferred { pat },
}
```

### `let`

```rust
pub struct StmtLocal {
    pub pat: Pattern,
    pub ty: Option<(Colon, Type)>,
    ...
}
```

So you have:

```text
PatType
TypedParam
ClosureParam::Typed
StmtLocal.ty
```

all describing substantially the same syntax.

### Recommendation

Make `PatType` canonical, like `syn`.

Function params:

```rust
enum FnParam {
    Receiver(Receiver),
    Typed(PatType),
}
```

Closure inputs can be `Pattern`, where a typed closure argument is `Pattern::Type`.

`StmtLocal.pat` similarly becomes `Pattern::Type(...)` when appropriate.

That eliminates three parallel implementations.

---

# 3. 🔴 `TypedParam` uses the wrong pattern grammar

This one is subtle.

Your `TypedParam` does:

```rust
let pat = Box::new(stream.parse::<Pattern>()?);
```

But `Pattern::parse` accepts top-level `|`.

`syn::PatType` deliberately uses:

```rust
Pat::parse_single(input)
```

because top-level or-patterns aren't permitted in function arguments without parentheses.

So Moxy can over-accept things corresponding to:

```rust
fn foo(A | B: T) {}
```

where the correct grammar requires:

```rust
fn foo((A | B): T) {}
```

Another reason to delete `TypedParam` and use `PatType`.

---

# 4. 🔴 `PatParen` is unreachable

Moxy exposes both:

```rust
Pattern::Tuple(PatTuple)
Pattern::Paren(PatParen)
```

but the parser currently handles every `(...)` like this:

```rust
let elems =
    Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;

return Ok(Pattern::Tuple(PatTuple { attrs, elems }));
```

Therefore:

```rust
(x)
```

becomes a tuple pattern.

But Rust/syn distinguish:

```text
(x)   → PatParen
(x,)  → PatTuple
()    → PatTuple
(x,y) → PatTuple
```

`syn` explicitly routes parentheses through `pat_paren_or_tuple`.

This is exactly the same class of bug as `PatType`: **the AST advertises a node the source parser cannot produce.**

---

# 5. 🔴 `PatRange` is unreachable

`PatRange` exists:

```rust
pub struct PatRange {
    pub attrs: Attributes,
    pub start: Option<Expr>,
    pub limits: RangeLimits,
    pub end: Option<Expr>,
}
```

but has no parser.

And `Pattern::parse_single` contains no construction of:

```rust
Pattern::Range(...)
```

Worse, the literal branch does this:

```rust
if token_is_literal {
    let expr = stream.parse::<Expr>()?;
    return Ok(Pattern::Lit(PatLit { attrs, expr }));
}
```

So `PatLit` contains a completely unrestricted `Expr`:

```rust
pub struct PatLit {
    pub attrs: Attributes,
    pub expr: Expr,
}
```

That can blur things like:

```rust
1
1..5
1..=5
```

into expression parsing rather than pattern parsing.

`syn` deliberately has a `pat_lit_or_range` path and also explicitly recognizes leading `-`.

### Related bug

Moxy's pattern dispatcher only recognizes a literal if the first token is a literal.

So:

```rust
-1
```

doesn't enter the literal branch at all.

`syn` explicitly checks:

```rust
input.peek(Token![-]) || lookahead.peek(Lit)
```

---

# 6. 🔴 `Pattern::Macro` is effectively unreachable

`Pattern` contains:

```rust
Macro(crate::MacroCall)
```

but the path-led parser only distinguishes:

```text
path(...)
path{...}
bare ident
path
```

There is no `path ! group` branch.

`syn` explicitly sends those prefixes through:

```text
pat_path_or_macro_or_struct_or_range
```

So something like:

```rust
some_macro!(...)
```

in pattern position cannot become Moxy's advertised `Pattern::Macro`.

---

# 7. 🔴 Qualified patterns are missing

Moxy's path-pattern branch starts only when the first token is:

```text
Ident
Keyword
::
```

Therefore it cannot start with `<`.

But Rust/syn supports qualified pattern paths such as:

```rust
<T as Trait>::CONST
```

and `syn` explicitly considers `<` a pattern-path prefix. Its `PatPath`, `PatStruct`, and `PatTupleStruct` carry `QSelf`.

Moxy's corresponding constructors currently hard-code:

```rust
qself: None
```

So those public `qself` fields are another **AST capability not reachable through the parser**.

---

# 8. 🟠 `PatOr.leading_vert` information is lost

Moxy recognizes:

```rust
| A | B
```

and consumes the initial `|`.

But `PatOr` only stores:

```rust
pub struct PatOr {
    pub attrs: Attributes,
    pub cases: Punctuated<Pattern, Or>,
}
```

There is nowhere to store the first token.

`syn` has:

```rust
pub leading_vert: Option<Token![|]>
```

Therefore:

```text
parse → AST → ToTokens
```

changes:

```rust
| A | B
```

into:

```rust
A | B
```

That's lossy AST behavior.

---

# 9. 🟠 Pattern attributes are lost

`syn` uses actual node structs for:

```rust
PatWild
PatRest
...
```

and those contain `attrs`.

Moxy instead has unit variants:

```rust
Wild,
Rest,
```

Yet `parse_single()` parses attributes *before* detecting these variants.

That means attributes can be consumed and then discarded.

Similar fidelity issues exist with:

```rust
Box(Box<Pattern>)
Const(StmtBlock)
```

because the wrapper token/node metadata is not represented the way `syn` represents pattern nodes.

---

# 10. 🔴 `TypeBareFn.variadic` is dead

This is another clean `PatType`-style inconsistency.

You expose:

```rust
pub struct BareFnParams {
    pub inputs: Punctuated<BareFnArg, Comma>,
    pub variadic: Option<Variadic>,
}
```

but the parser always constructs:

```rust
BareFnParams {
    inputs,
    variadic: None,
}
```

So:

```rust
extern "C" fn(i32, ...)
```

cannot populate the field that exists specifically for it.

`syn::TypeFnPtr` explicitly contains and parses:

```rust
variadic: Option<FnPtrVariadic>
```

**Definite bug.**

---

# 11. 🔴 `TypeTraitObject::dyn_token == None` is unreachable

Your node says:

```rust
pub struct TypeTraitObject {
    pub dyn_token: Option<Dyn>,
    ...
}
```

and its own parser accepts optional `dyn`.

But `Type::parse` only enters that parser when:

```rust
if stream.peek::<Dyn>() {
    return Ok(Self::TraitObject(stream.parse()?));
}
```

Otherwise the fallback is `Type::Path`.

So the parent parser makes:

```rust
TypeTraitObject {
    dyn_token: None,
}
```

unreachable.

This matters for legacy bare trait-object syntax, which is exactly why `syn` models `dyn_token` as optional.

---

# 12. 🔴 `TypeGroup` gets erased during printing

This is outright broken.

`TypeGroup` contains:

```rust
pub struct TypeGroup {
    pub span: Span,
    pub elem: Box<Type>,
}
```

But:

```rust
impl ToTokens for Type {
    ...
    Self::Group(_) => {}
}
```

So if the proc-macro bridge constructs:

```text
Type::Group(T)
```

then `ToTokens` emits **nothing**, rather than emitting `T` under an invisible group.

That's data loss.

---

# 13. 🟠 Type nodes don't preserve `syn`-level attributes

Compare `TypeReference`.

Moxy:

```rust
pub struct TypeReference {
    pub and: And,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}
```

`syn`:

```rust
pub struct TypeReference {
    pub attrs: Vec<Attribute>,
    pub and_token: Token![&],
    ...
}
```

The same applies broadly to `TypeArray`, `TypeParen`, `TypeTuple`, `TypeNever`, `TypeInfer`, etc.

Moxy's central `Type::parse` also doesn't begin by parsing attributes.

This isn't necessarily a parser crash, but it means Moxy cannot be a fully faithful Rust syntax AST in the same way `syn` is.

---

# 14. 🟠 `Type::Infer` and `Type::Never` are under-modeled

Moxy represents:

```rust
Never(Not),
Infer(Ident),
```

while `syn` has:

```rust
TypeNever {
    attrs,
    bang_token,
}

TypeInfer {
    attrs,
    underscore_token,
}
```

Using an `Ident` for `_` is especially semantically odd; `_` is not an identifier in this grammar.

I'd introduce `TypeInfer` and `TypeNever` structs if fidelity with `syn` is your goal.

---

# 15. 🔴 Attributed lifetime generic parameters dispatch incorrectly

`LifetimeParam` correctly supports attributes:

```rust
pub struct LifetimeParam {
    pub attrs: Attributes,
    pub lifetime: Lifetime,
    ...
}
```

But `GenericParam::parse` determines lifetime-vs-type **before skipping attributes**:

```rust
if current_token_is_quote {
    return Lifetime(...)
}

let mut fork = stream.fork();
fork.skip_while::<Attribute>();

if fork.peek::<Const>() {
    ...
}

Type(...)
```

So:

```rust
<#[some_attr] 'a>
```

doesn't dispatch to `LifetimeParam`.

It sees `#`, fails the lifetime branch, skips attrs only to detect `const`, then falls through to `TypeParam`.

Parent and child parsers disagree about the child syntax.

---

# 16. 🟠 `LifetimeParam` fabricates its `:` token

The child helper:

```rust
Lifetime::parse_bounds()
```

consumes the actual colon and throws it away.

Then `LifetimeParam::parse` reconstructs:

```rust
let colon_punct =
    if !bounds.is_empty() {
        Some(Colon::default())
    } else {
        None
    };
```

So the AST claims to preserve punctuation but doesn't preserve its span.

That should simply be parsed at the owning node:

```rust
let colon = stream.parse_if::<Colon>();
```

and then parse bounds if present.

---

# 17. 🟠 `Generics` owns too much grammar

Moxy defines:

```rust
pub struct Generics {
    pub lt_punct: Lt,
    pub gt_punct: Gt,
    pub params: ...,
    pub where_clause: Option<WhereClause>,
}
```

and `Generics::parse` parses both `<...>` **and immediately following `where`**.

`syn` explicitly documents why these are logically one AST object but **not one contiguous grammar production**:

> Generic parameters and a where clause may have other syntax between them.

Your `Signature` already has to work around this:

```rust
let mut generics = stream.parse::<Generics>()?;
let params = ...;
let output = ...;

if stream.peek::<Where>() {
    generics.where_clause = Some(stream.parse()?);
}
```

That is the architecture telling you `Generics::parse()` shouldn't greedily own the `where`.

### Recommendation

Have something equivalent to:

```rust
Generics::parse_params(...)
WhereClause::parse(...)
```

and let the grammatical owner decide where the `where` occurs.

Keep `where_clause` *stored* on `Generics`; just don't make generic-parameter parsing automatically consume it.

---

# 18. 🟠 Generic delimiters lose their real spans

`Generics::parse` does:

```rust
let _ = stream.parse::<Lt>()?;
...
let _ = stream.parse::<Gt>()?;
```

then stores:

```rust
lt_punct: Lt::default(),
gt_punct: Gt::default(),
```

So:

```text
actual <
actual >
```

are deliberately discarded and replaced.

`syn` stores the actual:

```rust
lt_token: Option<Token![<]>,
gt_token: Option<Token![>]>,
```

This hurts span fidelity and diagnostics.

`Signature::emit_angle_params` then goes one step further and prints fresh default delimiters again.

---

# 19. 🔴 `ItemStruct` suppresses a parse error for `;`

This is a real correctness bug.

The parser does:

```rust
let fields = stream.parse::<Fields>()?;
let _ = stream.parse::<Semi>();
```

Notice the absence of `?`.

For tuple/unit structs, the semicolon is syntactically required, but failure is thrown away.

So malformed source like:

```rust
struct Foo(u32)
```

can be accepted by `ItemStruct`.

Combined with the lack of EOF checking, this family of ignored-result bugs is particularly dangerous.

Also, the AST stores:

```rust
semi_punct: Semi
```

rather than optionality corresponding to named-vs-unnamed fields, and manufactures a default token.

---

# 20. 🟠 Tuple-struct `where` positioning needs restructuring

`ItemStruct` currently parses:

```text
struct
Ident
Generics
optional where
Fields
semi
```

But Rust permits the where clause of tuple structs after the tuple fields:

```rust
struct Foo<T>(T)
where
    T: Clone;
```

This is another consequence of treating the `where_clause` as if it always belongs immediately after `<...>`.

`syn`'s Generics design specifically avoids making that assumption.

---

# 21. 🟠 `ExprType` looks like dead legacy AST

Moxy currently has:

```rust
BinaryExpr::Type(ExprType)
```

but its binary parser contains handling for:

```text
as
=
assignment ops
ranges
binary ops
```

and never constructs `ExprType`.

Current `syn` no longer has an `ExprType` variant corresponding to old expression type-ascription syntax.

So this appears to be another dead AST branch rather than missing parser support.

### Recommendation

Unless Moxy intentionally supports some nonstandard syntax:

**delete `ExprType` rather than making it parsable.**

---

# 22. 🟠 `Expr::Infer` is similarly suspicious

Moxy has:

```rust
Expr::Infer
```

and even a file whose entire implementation is:

```rust
// ExprInfer is represented as a unit variant `Expr::Infer` in the enum.
// No separate struct is needed; the variant carries no data.
```

But `ToTokens` does:

```rust
Self::Infer => {}
```

So even manually constructing `Expr::Infer` erases `_`.

`syn::ExprInfer` retains an underscore token and attributes.

This should be a real node:

```rust
pub struct ExprInfer {
    pub attrs: Attributes,
    pub underscore: Underscore,
}
```

---

# 23. 🟠 `BoundLifetimes` is narrower than `syn`

Moxy:

```rust
pub struct BoundLifetimes {
    pub for_keyword: For,
    pub lt: Lt,
    pub params: Punctuated<Lifetime, Comma>,
    pub gt: Gt,
}
```

`syn` uses:

```rust
Punctuated<GenericParam, Token![,]>
```

inside `BoundLifetimes`.

That allows the binder's parameter syntax to be represented consistently with the rest of the generic grammar, including attributes and newer language evolution.

I'd align this.

---

# 24. 🟠 Your generic `Peek` mechanism hides malformed syntax

This isn't an AST node, but it's producing AST inconsistencies.

Moxy globally defines:

```rust
impl<T: Parse> Peek for T {
    fn peek(stream: &mut ParseStream) -> bool {
        Self::parse(stream).is_ok()
    }
}
```

and:

```rust
impl<T: Parse> Parse for Option<T> {
    fn parse(...) {
        let mut fork = stream.fork();

        match T::parse(&mut fork) {
            Ok(v) => ...
            Err(_) => Ok(None),
        }
    }
}
```

Likewise `ParseStream::peek()` executes that speculative full parse and rolls back.

This has two problems.

First, **malformed optional syntax and absent optional syntax become indistinguishable**.

Second, something like `Item::parse` does:

```rust
if stream.peek::<ItemStruct>() { ... }
if stream.peek::<ItemEnum>() { ... }
...
```

meaning you're potentially parsing entire items merely to determine which kind they are.

`syn` relies much more heavily on cheap deterministic lookahead.

I wouldn't necessarily remove generic speculative parsing, but **don't make it your universal `Peek` implementation**.

---

# Architecture assessment

The recurring pattern is:

```text
AST design
   ↓
roughly modeled after syn

Parser design
   ↓
implemented independently

Result
   ↓
AST capabilities and parser capabilities drift apart
```

The strongest examples are:

```text
PatType             node exists, parser cannot create
PatParen            node exists, parser cannot create
PatRange            node exists, parser cannot create
Pattern::Macro      variant exists, parser cannot create
Pattern qself       field exists, parser never populates
TypeTraitObject     optional dyn exists, parent never permits None
TypeBareFn.variadic field exists, parser always writes None
ExprType            variant exists, parser never creates
ExprInfer           variant exists but has no token fidelity
```

This is too repetitive to fix case-by-case without putting a guardrail around it.

---

# What I would change

In order:

1. **Make top-level parsing require EOF.** This will immediately expose lots of currently hidden failures.
2. **Rebuild `Pattern` parsing around `syn`'s grammar boundaries:** `parse_single`, multi-pattern, leading-vert multi-pattern, and standalone `PatType::parse`.
3. **Delete `TypedParam` and `ClosureParam` as parallel typed-pattern ASTs.** Represent typed parameters using `PatType`.
4. **Fix pattern dispatch:** paren-vs-tuple, macro, range, negative literal, qself, leading vert, attributes.
5. **Separate generic parameter parsing from where-clause parsing.**
6. **Audit every field of every AST node for reachability.** A simple test should prove that every syntax-bearing variant/optional field has at least one source string capable of producing it.
7. **Stop synthesizing punctuation after throwing the real token away.**
8. **Fix `TypeBareFn::variadic` and `TypeGroup::ToTokens`.**
9. **Remove stale nodes such as `ExprType` unless they're intentionally non-Rust extensions.**
10. **Replace universal full-parse `Peek` with syntax-specific lookahead for major grammar nodes.**

## Most valuable test suite

I would add a differential suite where the same snippet is parsed by Moxy and `syn`, then compare **node classification**, not exact AST structure:

```text
(x)                     PatParen
(x,)                    PatTuple
foo: u32                 PatType
-1                       PatLit
1..=5                    PatRange
foo!(x)                  PatMacro
<T as Trait>::CONST      PatPath
| A | B                  PatOr + leading |
fn f(x: u32) {}          typed PatType
|x: u32| x               closure PatType
fn(...)                  TypeFnPtr variadic tests
dyn Trait                TypeTraitObject
```

Then add negative differential cases:

```text
fn f(A | B: T) {}        reject
struct Foo(u32)           reject
Pattern("x: T garbage")   reject trailing input
```

That test harness would have caught **most of the issues above automatically**.

### Bottom line

The **pattern subsystem is the most inconsistent area and needs a small redesign**, not just a `PatType::parse` implementation. The type/generics systems have several similar reachability and token-fidelity bugs, while the expression layer contains at least two stale/under-modeled branches. The parser infrastructure's failure to enforce EOF is amplifying all of them.

I would make **“Moxy parses the same grammar boundary as `syn`”** an explicit invariant and then allow AST structural differences only where they're deliberate.
