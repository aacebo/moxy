use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{Render, core::Field, params};

#[derive(Clone, Default)]
pub struct StructSyntax;

impl Render for StructSyntax {
    type Args = params::StructParams;

    fn render(&self, args: Self::Args) -> syn::Result<TokenStream> {
        let ident = &args.input.ident;
        let build_ident = format_ident!("{}Builder", &ident);
        let vis = &args.input.vis;
        let (impl_generics, type_generics, where_generics) = &args.input.generics.split_for_impl();
        let fields: Vec<_> = args
            .data
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::parse(i, field))
            .collect::<syn::Result<Vec<_>>>()?
            .into_iter()
            .filter(|field| field.attrs().exists("build"))
            .collect();

        let build_fields: Vec<_> = fields
            .iter()
            .map(|field| {
                let name = field.name();
                let ty = field.ty();
                quote!(#name: Option<#ty>)
            })
            .collect();

        let build_methods: Vec<_> = fields
            .iter()
            .map(|field| {
                let name = field.name();
                let ty = field.ty();

                quote! {
                    pub fn #name<V: Into<#ty>>(mut self, value: V) -> Self {
                        self.#name = Some(value.into());
                        self
                    }
                }
            })
            .collect();

        let build_fields_assign: Vec<_> = fields
            .iter()
            .map(|field| {
                let name = field.name();
                quote!(#name: self.#name.expect("required"))
            })
            .collect();

        Ok(quote! {
            #[derive(Default)]
            #vis struct #build_ident #type_generics #where_generics {
                #(#build_fields,)*
            }

            impl #impl_generics #build_ident #type_generics #where_generics {
                #(#build_methods)*

                pub fn build(self) -> #ident #type_generics {
                    #ident {
                        #(#build_fields_assign,)*
                        ..Default::default()
                    }
                }
            }

            impl #impl_generics #ident #type_generics #where_generics {
                pub fn new() -> #build_ident #type_generics {
                    #build_ident::default()
                }
            }
        })
    }
}
