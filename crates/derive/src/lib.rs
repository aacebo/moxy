mod build;
pub(crate) mod core;
mod default;
mod deref;
mod display;
pub(crate) mod params;
mod traits;

use proc_macro::TokenStream;

pub(crate) use traits::*;

#[proc_macro_derive(Deref, attributes(moxy))]
pub fn derive_deref(tokens: TokenStream) -> TokenStream {
    match deref::render(tokens) {
        Err(err) => err.to_compile_error().into(),
        Ok(v) => v.into(),
    }
}

#[proc_macro_derive(Display, attributes(moxy))]
pub fn derive_display(tokens: TokenStream) -> TokenStream {
    match display::render(tokens) {
        Err(err) => err.to_compile_error().into(),
        Ok(v) => v.into(),
    }
}

#[proc_macro_derive(Build, attributes(moxy))]
pub fn derive_build(tokens: TokenStream) -> TokenStream {
    match build::render(tokens) {
        Err(err) => err.to_compile_error().into(),
        Ok(v) => v.into(),
    }
}

#[proc_macro_derive(Default, attributes(moxy))]
pub fn derive_default(tokens: TokenStream) -> TokenStream {
    match default::render(tokens) {
        Err(err) => err.to_compile_error().into(),
        Ok(v) => v.into(),
    }
}
