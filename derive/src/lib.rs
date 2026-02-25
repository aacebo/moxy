mod deref;

use proc_macro::TokenStream;

#[proc_macro_derive(Deref, attributes(moxy))]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(v) => deref::derive_struct(&input, v),
        _ => panic!("unsupported type"),
    }
    .into()
}
