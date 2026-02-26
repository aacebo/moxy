pub(crate) mod core;
mod deref;

use proc_macro::TokenStream;
use quote::ToTokens;

pub(crate) trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}

impl<T: ToTokens> Render for syn::Result<T> {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Err(err) => Err(err.clone()),
            Ok(v) => Ok(v.to_token_stream()),
        }
    }
}

pub(crate) trait Error: syn::spanned::Spanned {
    fn error(&self, message: &str) -> syn::Error {
        syn::Error::new(self.span(), message)
    }
}

impl<T: syn::spanned::Spanned> Error for T {}

#[proc_macro_derive(Deref, attributes(moxy))]
pub fn derive_deref(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    match input.data.clone() {
        syn::Data::Struct(v) => deref::StructMacro::new(input, v)
            .render()
            .unwrap_or_default()
            .to_token_stream(),
        _ => panic!("unsupported type"),
    }
    .into()
}
