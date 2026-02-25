use proc_macro2::TokenStream;
use quote::quote;

use crate::core::Field;

pub fn derive_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_generics) = &input.generics.split_for_impl();
    let fields: Vec<_> = data
        .fields
        .iter()
        .enumerate()
        .filter_map(|(i, field)| Field::parse(i, field).ok())
        .collect();

    let field = fields
        .iter()
        .find(|field| field.attrs().exists("moxy", "deref"))
        .or_else(|| fields.first());

    match field {
        None => syn::Error::new_spanned(&input, "field not found").to_compile_error(),
        Some(field) => {
            let field_name = field.name();
            let field_ty = field.ty();

            quote! {
                impl #impl_generics ::std::ops::Deref for #ident #type_generics #where_generics {
                    type Target = #field_ty;

                    fn deref(&self) -> &Self::Target {
                        &self.#field_name
                    }
                }
            }
        }
    }
}
