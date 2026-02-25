use proc_macro2::TokenStream;
use quote::quote;

use crate::attr::Attr;

pub fn derive_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_generics) = &input.generics.split_for_impl();
    let fields: Vec<_> = data.fields.iter().enumerate().collect();
    let field = fields
        .iter()
        .find(|(_, field)| {
            field.attrs.iter().any(|attr| match Attr::parse(attr) {
                Ok(v) => v.is_moxy() && v.get("deref").is_some(),
                Err(_) => false,
            })
        })
        .map(|(i, field)| (syn::Index::from(*i), field))
        .or_else(|| {
            fields
                .first()
                .map(|(i, field)| (syn::Index::from(*i), field))
        });

    match field {
        None => syn::Error::new_spanned(&input, "field not found").to_compile_error(),
        Some((i, field)) => {
            let field_ty = &field.ty;
            let field_ident = match &field.ident {
                None => quote!(#i),
                Some(v) => quote!(#v),
            };

            quote! {
                impl #impl_generics ::std::ops::Deref for #ident #type_generics #where_generics {
                    type Target = #field_ty;

                    fn deref(&self) -> &Self::Target {
                        &self.#field_ident
                    }
                }
            }
        }
    }
}
