use proc_macro2::TokenStream;
use quote::quote;

use crate::{Render, core::Field, params};

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
            .map(|(i, field)| Field::parse(i, field))
            .collect::<syn::Result<Vec<_>>>()?;

        let defaults: Vec<_> = fields
            .iter()
            .map(|field| {
                let fname = field.name();
                let default = field.attrs().iter().find_map(|attr| {
                    attr.args()
                        .iter()
                        .find(|arg| arg.path().is_ident("default"))
                        .and_then(|arg| arg.as_value_tokens())
                });

                if let Some(value) = default {
                    quote!(#fname: #value.into())
                } else {
                    quote!(#fname: ::std::default::Default::default())
                }
            })
            .collect();

        Ok(quote! {
            impl #impl_generics ::std::default::Default for #ident #type_generics #where_generics {
                fn default() -> Self {
                    Self {
                        #(#defaults,)*
                    }
                }
            }
        })
    }
}
