# Backlog

## 1. Meta Argument Parsing

Similar to the `darling` crate, we need a derive macro and trait to make it easy to
parse complex proc macro arguments.

## 2. Refactor `moxy-fmt` AST

Refactor ast of fmt crate to implement `Parse` and `Format` for each sub node type.

## 3. Refactor `moxy-template` AST

Refactor ast for template crate to implement `Parse` for each sub node type.

## 4. Make Result More Ergonomic

Early returns from proc macro functions are very ergonomic since their return signature is `TokenStream`,
need to find a way for early returns from `Result<TokenStream, ParseError>` to be less verbose, which currently
requires `if let Ok(..)` statements.

## 5. `fmt!` should recurse through nested `template!`

```rust
pub fn apply(tokens: TokenStream) -> Result<TokenStream, ParseError> {
    let item = moxy::parse!(tokens as moxy::ast::ItemEnum)?;

    Ok(moxy::template! {
        {{ item }}

        impl {{ &item.ident }} {
            pub fn as_str(&self) -> &'static str {
                match self {
                    @for (variant in item.variants.iter()) {
                        Self::{{ variant.ident }} => stringify!({{ variant.ident }}),
                    }
                }
            }
        }
    })
}
```

```rust
#[proc_macro]
pub fn apply(tokens: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
	pub fn __call__(tokens: TokenStream) -> Result<TokenStream, ParseError> {
		let item = moxy::parse!(tokens as moxy :: ast :: ItemEnum)?;
		Ok(moxy::template!{{{item}} impl {{& item . ident}} {pub fn as_str (& self) -> & 'static str {match self {@ for (variant in item . variants . iter ()) {Self :: {{variant . ident}} => stringify ! ({{variant . ident}}) ,}}}}})
	}
	match __call__(tokens.into()) {
		Err(err) => err.to_compile_error().into(),
		Ok(v) => v.into(),
	}
}
```

## 6. Make Punctuated and Delimited More Ergonomic

Make parsing/generating punctuated/delimited syntax easier.

## 7. Support Deconstruct Syntax In Control Flow

Template control flow syntax should support things like

```rust
@for ((name, ty) in fields) {
    ...
}
```

## 8. Rename `moxy::ast::Declaration` to `CustomType`
