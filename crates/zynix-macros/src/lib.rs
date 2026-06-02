extern crate proc_macro;

mod options;
mod parse;
mod to_tokens;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Parse, attributes(parse))]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match parse::expand(&input) {
        Ok(ts) => ts.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(ToTokens, attributes(parse))]
pub fn derive_to_tokens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match to_tokens::expand(&input) {
        Ok(ts) => ts.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
