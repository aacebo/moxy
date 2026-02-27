use proc_macro2::TokenStream;
use quote::quote;

use crate::{Error, Render, core::Field, params};

#[derive(Clone, Default)]
pub struct StructSyntax;

impl Render for StructSyntax {
    type Args = params::StructParams;

    fn render(&self, args: Self::Args) -> syn::Result<TokenStream> {
        let ident = &args.input.ident;
        let (impl_generics, type_generics, where_generics) = &args.input.generics.split_for_impl();
        let fields: Vec<_> = args
            .data
            .fields
            .iter()
            .enumerate()
            .filter_map(|(i, field)| Field::parse(i, field).ok())
            .collect();

        let field = fields
            .iter()
            .find(|field| field.attrs().exists("deref"))
            .or_else(|| fields.first());

        match field {
            None => Err(args.input.error("field not found")),
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
