use proc_macro2::TokenStream;
use quote::quote;

use crate::{Error, Render, core::Field};

pub struct StructMacro {
    input: syn::DeriveInput,
    data: syn::DataStruct,
}

impl StructMacro {
    pub fn new(input: syn::DeriveInput, data: syn::DataStruct) -> Self {
        Self { input, data }
    }
}

impl Render for StructMacro {
    fn render(&self) -> syn::Result<TokenStream> {
        let ident = &self.input.ident;
        let (impl_generics, type_generics, where_generics) = &self.input.generics.split_for_impl();
        let fields: Vec<_> = self
            .data
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
            None => Err(self.input.error("field not found")),
            Some(field) => {
                let field_name = field.name();
                let field_ty = field.ty();

                Ok(quote! {
                    impl #impl_generics ::std::ops::Deref for #ident #type_generics #where_generics {
                        type Target = #field_ty;

                        fn deref(&self) -> &Self::Target {
                            &self.#field_name
                        }
                    }
                })
            }
        }
    }
}
