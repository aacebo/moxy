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
        let generics = &args.input.generics;
        let (impl_generics, type_generics, where_generics) = generics.split_for_impl();
        let type_params: Vec<_> = generics.type_params().collect();
        let type_param_idents: Vec<_> = generics.type_params().map(|tp| &tp.ident).collect();
        let lifetime_params: Vec<_> = generics.lifetimes().collect();
        let all_fields: Vec<_> = args
            .data
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::parse(i, field))
            .collect::<syn::Result<Vec<_>>>()?;

        let fields: Vec<_> = all_fields
            .into_iter()
            .filter(|field| field.attrs().exists("build"))
            .collect();

        let is_optional = |f: &&Field| {
            self.render_default_tokens(f).ok().flatten().is_some()
                || self.render_option_inner(f).is_some()
        };

        let required: Vec<_> = fields.iter().filter(|f| !is_optional(f)).collect();

        let optional: Vec<_> = fields
            .iter()
            .filter(|f| self.render_default_tokens(f).ok().flatten().is_some())
            .collect();

        let option_fields: Vec<_> = fields
            .iter()
            .filter(|f| {
                self.render_option_inner(f).is_some()
                    && self.render_default_tokens(f).ok().flatten().is_none()
            })
            .collect();

        let const_param_idents: Vec<_> = required
            .iter()
            .map(|f| format_ident!("{}", f.name().to_string().to_uppercase()))
            .collect();

        let builder_fields: Vec<_> = fields
            .iter()
            .map(|field| {
                let name = field.name();
                if self.render_option_inner(field).is_some()
                    && self.render_default_tokens(field).ok().flatten().is_none()
                {
                    let ty = field.ty();
                    quote!(#name: #ty)
                } else {
                    let ty = field.ty();
                    quote!(#name: ::std::option::Option<#ty>)
                }
            })
            .collect();

        let field_names: Vec<_> = fields.iter().map(|f| f.name()).collect();
        let init_fields: Vec<_> = fields
            .iter()
            .map(|f| {
                let name = f.name();
                quote!(#name: ::std::option::Option::None)
            })
            .collect();

        let const_param_defs: Vec<_> = const_param_idents
            .iter()
            .map(|id| quote!(const #id: bool = false))
            .collect();

        let const_param_refs: Vec<_> = const_param_idents.iter().map(|id| quote!(#id)).collect();
        let const_param_true: Vec<_> = const_param_idents.iter().map(|_| quote!(true)).collect();
        let builder_struct = quote! {
            #vis struct #build_ident <#(#lifetime_params,)* #(#type_params,)* #(#const_param_defs,)*> #where_generics {
                #(#builder_fields,)*
            }
        };

        let required_setters: Vec<TokenStream> = required
            .iter()
            .enumerate()
            .map(|(i, field)| -> syn::Result<TokenStream> {
                let fname = field.name();
                let ty = field.ty();
                let method_name = match self.render_custom_method_name(field)? {
                    Some(id) => quote!(#id),
                    None => quote!(#fname),
                };

                let other_const_params: Vec<_> = const_param_idents
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, id)| quote!(const #id: bool))
                    .collect();

                let impl_const_refs: Vec<_> = const_param_idents
                    .iter()
                    .enumerate()
                    .map(|(j, id)| if j == i { quote!(false) } else { quote!(#id) })
                    .collect();

                let ret_const_refs: Vec<_> = const_param_idents
                    .iter()
                    .enumerate()
                    .map(|(j, id)| if j == i { quote!(true) } else { quote!(#id) })
                    .collect();

                let move_fields: Vec<_> = field_names
                    .iter()
                    .map(|n| {
                        if n.to_string() == fname.to_string() {
                            quote!(#n: ::std::option::Option::Some(value.into()))
                        } else {
                            quote!(#n: self.#n)
                        }
                    })
                    .collect();

                Ok(quote! {
                    impl <#(#lifetime_params,)* #(#type_params,)* #(#other_const_params,)*> #build_ident <#(#lifetime_params,)* #(#type_param_idents,)* #(#impl_const_refs,)*> #where_generics {
                        pub fn #method_name<V: Into<#ty>>(self, value: V) -> #build_ident <#(#lifetime_params,)* #(#type_param_idents,)* #(#ret_const_refs,)*> {
                            #build_ident {
                                #(#move_fields,)*
                            }
                        }
                    }
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        let optional_setters: Vec<TokenStream> = optional
            .iter()
            .map(|field| -> syn::Result<TokenStream> {
                let fname = field.name();
                let ty = field.ty();
                let method_name = match self.render_custom_method_name(field)? {
                    Some(id) => quote!(#id),
                    None => quote!(#fname),
                };

                Ok(quote! {
                    pub fn #method_name<V: Into<#ty>>(mut self, value: V) -> Self {
                        self.#fname = ::std::option::Option::Some(value.into());
                        self
                    }
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        let option_setters: Vec<TokenStream> = option_fields
            .iter()
            .map(|field| -> syn::Result<TokenStream> {
                let fname = field.name();
                let inner_ty = self.render_option_inner(field).unwrap();
                let method_name = match self.render_custom_method_name(field)? {
                    Some(id) => quote!(#id),
                    None => quote!(#fname),
                };

                Ok(quote! {
                    pub fn #method_name<V: Into<#inner_ty>>(mut self, value: V) -> Self {
                        self.#fname = ::std::option::Option::Some(value.into());
                        self
                    }
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        let all_const_generic_params: Vec<_> = const_param_idents
            .iter()
            .map(|id| quote!(const #id: bool))
            .collect();

        let all_optional_setters: Vec<&TokenStream> = optional_setters
            .iter()
            .chain(option_setters.iter())
            .collect();

        let optional_setters_block = if all_optional_setters.is_empty() {
            quote!()
        } else {
            quote! {
                impl <#(#lifetime_params,)* #(#type_params,)* #(#all_const_generic_params,)*> #build_ident <#(#lifetime_params,)* #(#type_param_idents,)* #(#const_param_refs,)*> #where_generics {
                    #(#all_optional_setters)*
                }
            }
        };

        let build_fields_assign: Vec<_> = fields
            .iter()
            .map(|field| -> syn::Result<TokenStream> {
                let fname = field.name();
                let default = self.render_default_tokens(field)?;
                let is_option = self.render_option_inner(field).is_some() && default.is_none();

                Ok(if is_option {
                    quote!(#fname: self.#fname)
                } else if let Some(default) = default {
                    quote!(#fname: self.#fname.unwrap_or_else(|| #default.into()))
                } else {
                    quote!(#fname: self.#fname.unwrap())
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        let build_impl = quote! {
            impl <#(#lifetime_params,)* #(#type_params,)*> #build_ident <#(#lifetime_params,)* #(#type_param_idents,)* #(#const_param_true,)*> #where_generics {
                pub fn build(self) -> #ident #type_generics {
                    #ident {
                        #(#build_fields_assign,)*
                        ..::std::default::Default::default()
                    }
                }
            }
        };

        let new_impl = quote! {
            impl #impl_generics #ident #type_generics #where_generics {
                pub fn new() -> #build_ident <#(#lifetime_params,)* #(#type_param_idents,)*> {
                    #build_ident {
                        #(#init_fields,)*
                    }
                }
            }
        };

        Ok(quote! {
            #builder_struct
            #(#required_setters)*
            #optional_setters_block
            #build_impl
            #new_impl
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
        let build_args = field.attrs().get("build")?;
        Ok(build_args
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

    fn render_default_tokens(
        &self,
        field: &Field,
    ) -> syn::Result<Option<proc_macro2::TokenStream>> {
        let build_args = field.attrs().get("build")?;
        Ok(build_args
            .iter()
            .find_map(|arg| arg.as_attr())
            .and_then(|attr| {
                attr.args()
                    .iter()
                    .find(|a| a.path().is_ident("default"))
                    .and_then(|a| a.as_value_tokens())
            }))
    }
}
