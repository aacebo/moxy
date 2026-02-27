mod structs;

use crate::{Render, params::Params};

pub fn render(tokens: proc_macro::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let args = syn::parse(tokens)?;

    match args {
        Params::Struct(v) => structs::StructSyntax.render(v),
    }
}
