use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Moxy)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    quote! {}.into()
}
