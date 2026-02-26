use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    Render,
    core::{Attrs, Field, FieldName},
    params,
};

#[derive(Clone, Default)]
pub struct StructSyntax;

impl Render for StructSyntax {
    type Args = params::StructParams;

    fn render(&self, args: Self::Args) -> syn::Result<TokenStream> {
        let ident = &args.input.ident;
        let (impl_generics, type_generics, where_generics) = &args.input.generics.split_for_impl();
        let attributes = Attrs::parse(&args.input.attrs)?;
        let display = attributes.get("display");
        let display_attr = display.iter().find_map(|arg| arg.as_attr());
        let custom_fmt = display_attr.and_then(|attr| {
            attr.args().iter().find_map(|arg| {
                if arg.path().is_ident("__value") {
                    arg.as_lit().and_then(|lit| match lit {
                        syn::Lit::Str(s) => Some(s.value()),
                        _ => None,
                    })
                } else {
                    None
                }
            })
        });

        let style = display_attr.and_then(|attr| {
            attr.args()
                .iter()
                .filter(|arg| arg.path().is_ident("style"))
                .find_map(|arg| arg.as_ident().map(|v| v.to_string()))
        });

        let fields: Vec<_> = args
            .data
            .fields
            .iter()
            .enumerate()
            .filter_map(|(i, field)| Field::parse(i, field).ok())
            .collect();

        let visible_fields: Vec<&Field> = fields
            .iter()
            .filter(|f| {
                let field_display = f.attrs().get("display");
                let field_attr = field_display.iter().find_map(|a| a.as_attr());
                !field_attr.map(|a| a.exists("ignore")).unwrap_or(false)
            })
            .collect();

        let is_named = matches!(args.data.fields, syn::Fields::Named(_));
        let is_unit = matches!(args.data.fields, syn::Fields::Unit);
        let name_str = ident.to_string();
        let body = if is_unit || visible_fields.is_empty() {
            quote! { ::std::write!(f, #name_str) }
        } else if let Some(fmt_str) = custom_fmt {
            render_custom_fmt(&visible_fields, is_named, &fmt_str)
        } else if let Some(mode) = style {
            match mode.as_str() {
                "debug" => render_debug(&visible_fields, is_named, ident.to_string().as_str()),
                "compact" => render_compact(&visible_fields),
                "keyvalue" => render_keyvalue(&visible_fields),
                "map" => render_map(&visible_fields),
                _ => unreachable!(),
            }
        } else {
            render_default(&visible_fields, is_named, ident.to_string().as_str())
        };

        Ok(quote! {
            impl #impl_generics ::std::fmt::Display for #ident #type_generics #where_generics {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    #body
                }
            }
        })
    }
}

fn render_default(fields: &[&Field], is_named: bool, name: &str) -> TokenStream {
    if is_named {
        let parts: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let fname = f.name();
                let fname_str = fname.to_string();
                let sep = if i + 1 < fields.len() { ", " } else { "" };
                quote! { ::std::write!(f, "{}: {}{}", #fname_str, self.#fname, #sep)?; }
            })
            .collect();

        quote! {
            ::std::write!(f, "{} {{ ", #name)?;
            #(#parts)*
            ::std::write!(f, " }}")
        }
    } else {
        let parts: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let fname = f.name();
                let sep = if i + 1 < fields.len() { ", " } else { "" };
                quote! { ::std::write!(f, "{}{}", self.#fname, #sep)?; }
            })
            .collect();

        quote! {
            ::std::write!(f, "{}(", #name)?;
            #(#parts)*
            ::std::write!(f, ")")
        }
    }
}

fn render_debug(fields: &[&Field], is_named: bool, name: &str) -> TokenStream {
    if is_named {
        let entries: Vec<_> = fields
            .iter()
            .map(|f| {
                let fname = f.name();
                let fname_str = fname.to_string();
                quote! { .field(#fname_str, &self.#fname) }
            })
            .collect();

        quote! {
            f.debug_struct(#name)
                #(#entries)*
                .finish()
        }
    } else {
        let entries: Vec<_> = fields
            .iter()
            .map(|f| {
                let fname = f.name();
                quote! { .field(&self.#fname) }
            })
            .collect();

        quote! {
            f.debug_tuple(#name)
                #(#entries)*
                .finish()
        }
    }
}

fn render_compact(fields: &[&Field]) -> TokenStream {
    let parts: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let fname = f.name();
            if i + 1 < fields.len() {
                quote! { ::std::write!(f, "{} ", self.#fname)?; }
            } else {
                quote! { ::std::write!(f, "{}", self.#fname)?; }
            }
        })
        .collect();

    quote! {
        #(#parts)*
        ::std::result::Result::Ok(())
    }
}

fn render_keyvalue(fields: &[&Field]) -> TokenStream {
    let parts: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let fname = f.name();
            let fname_str = fname.to_string();
            if i + 1 < fields.len() {
                quote! { ::std::write!(f, "{}={} ", #fname_str, self.#fname)?; }
            } else {
                quote! { ::std::write!(f, "{}={}", #fname_str, self.#fname)?; }
            }
        })
        .collect();

    quote! {
        #(#parts)*
        ::std::result::Result::Ok(())
    }
}

fn render_map(fields: &[&Field]) -> TokenStream {
    let parts: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let fname = f.name();
            let fname_str = fname.to_string();
            let sep = if i + 1 < fields.len() { ", " } else { "" };
            quote! { ::std::write!(f, "{}: {}{}", #fname_str, self.#fname, #sep)?; }
        })
        .collect();

    quote! {
        ::std::write!(f, "{{ ")?;
        #(#parts)*
        ::std::write!(f, " }}")
    }
}

fn render_custom_fmt(fields: &[&Field], is_named: bool, fmt_str: &str) -> TokenStream {
    if is_named {
        let field_idents: Vec<_> = fields
            .iter()
            .filter_map(|f| match f.name() {
                FieldName::Ident(id) => Some(id.clone()),
                _ => None,
            })
            .collect();

        quote! {
            #[allow(unused_variables)]
            let Self { #(#field_idents,)* .. } = self;
            ::std::write!(f, #fmt_str)
        }
    } else {
        let field_indices: Vec<_> = fields.iter().map(|f| f.name().clone()).collect();

        quote! {
            ::std::write!(f, #fmt_str, #(self.#field_indices,)*)
        }
    }
}
