# Backlog

## 1. Refactor `moxy-fmt` AST

Refactor ast of fmt crate to implement `Parse` and `Format` for each sub node type.

## 2. Refactor `moxy-template` AST

Refactor ast for template crate to implement `Parse` for each sub node type.

## 3. `fmt!` should recurse through nested `template!`

```rust
pub fn apply(tokens: TokenStream) -> Result<TokenStream, ParseError> {
    let item = moxy::parse!(tokens as moxy::ast::ItemEnum)?;

    Ok(moxy::template! {
        {{ item }}

        impl {{ &item.ident }} {
            pub fn as_str(&self) -> &'static str {
                match self {
                    @for variant in item.variants.iter() {
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

## 4. Make Punctuated and Delimited More Ergonomic

Make parsing/generating punctuated/delimited syntax easier.
