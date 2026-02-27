use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

        let fields: Vec<_> = fields
            .into_iter()
            .filter(|field| field.attrs().exists("set"))
            .collect();

        let methods: Vec<TokenStream> = fields
            .iter()
            .map(|field| -> syn::Result<TokenStream> {
                let fname = field.name();
                let docs = self.render_docs(field);
                let is_option = self.render_option_inner(field).is_some();
                let on_callback = self.render_on_callback(field)?;

                let method_name = match self.render_custom_method_name(field)? {
                    Some(id) => id,
                    None => format_ident!("set_{}", fname.to_string()),
                };

                let setter_ty = if is_option {
                    let inner = self.render_option_inner(field).unwrap();
                    quote!(#inner)
                } else {
                    let ty = field.ty();
                    quote!(#ty)
                };

                let body = if let Some(on_expr) = on_callback {
                    if is_option {
                        quote! {
                            let value: #setter_ty = value.into();
                            self.#fname = ::std::option::Option::Some(#on_expr);
                        }
                    } else {
                        quote! {
                            let value: #setter_ty = value.into();
                            self.#fname = #on_expr;
                        }
                    }
                } else if is_option {
                    quote! {
                        self.#fname = ::std::option::Option::Some(value.into());
                    }
                } else {
                    quote! {
                        self.#fname = value.into();
                    }
                };

                Ok(quote! {
                    #(#docs)*
                    pub fn #method_name<V: ::std::convert::Into<#setter_ty>>(&mut self, value: V) -> &mut Self {
                        #body
                        self
                    }
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(quote! {
            impl #impl_generics #ident #type_generics #where_generics {
                #(#methods)*
            }
        })
    }
}

impl StructSyntax {
    fn render_option_inner<'a>(&self, field: &'a Field) -> Option<&'a syn::Type> {
        let syn::Type::Path(type_path) = field.ty() else {
            return None;
        };
        let segment = type_path.path.segments.last()?;
        if segment.ident != "Option" {
            return None;
        }
        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
            return None;
        };
        let syn::GenericArgument::Type(inner) = args.args.first()? else {
            return None;
        };
        Some(inner)
    }

    fn render_custom_method_name(&self, field: &Field) -> syn::Result<Option<proc_macro2::Ident>> {
        let set_args = field.attrs().get("set")?;
        Ok(set_args
            .iter()
            .find_map(|arg| arg.as_attr())
            .and_then(|attr| {
                attr.args().iter().find_map(|a| {
                    if !a.path().is_ident("__value") {
                        return None;
                    }
                    a.as_lit().and_then(|lit| match lit {
                        syn::Lit::Str(s) => Some(format_ident!("{}", s.value())),
                        _ => None,
                    })
                })
            }))
    }

    fn render_on_callback(&self, field: &Field) -> syn::Result<Option<proc_macro2::TokenStream>> {
        let set_args = field.attrs().get("set")?;
        Ok(set_args
            .iter()
            .find_map(|arg| arg.as_attr())
            .and_then(|attr| {
                attr.args()
                    .iter()
                    .find(|a| a.path().is_ident("on"))
                    .and_then(|a| a.as_value_tokens())
            }))
    }

    fn render_docs<'a>(&self, field: &'a Field) -> Vec<&'a syn::Attribute> {
        field
            .raw_attrs()
            .iter()
            .filter(|a| a.path().is_ident("doc"))
            .collect()
    }
}
