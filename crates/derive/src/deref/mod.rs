mod structs;

use crate::{Render, params};

pub fn render(tokens: proc_macro::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse::<syn::DeriveInput>(tokens.into())?;

    match input.data.clone() {
        syn::Data::Struct(data) => {
            structs::StructSyntax.render(params::StructParams { input, data })
        }
        _ => panic!("invalid type"),
    }
}
