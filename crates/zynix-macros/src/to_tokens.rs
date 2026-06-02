use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Index, Type};

use crate::options::{ToTokenOptions, type_is};

pub fn expand(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let (imp, ty, wher) = input.generics.split_for_impl();
    let tokens = format_ident!("tokens");

    let body = match &input.data {
        Data::Struct(s) => fields(&s.fields, &tokens, true)?,
        Data::Enum(e) => {
            let mut arms = Vec::new();

            for variant in &e.variants {
                let vname = &variant.ident;
                let (pat, emit) = variant_arm(&variant.fields, &tokens)?;
                arms.push(quote! { Self::#vname #pat => { #emit } });
            }

            quote! { match self { #(#arms)* } }
        }
        Data::Union(_) => return Err(syn::Error::new(input.span(), "unions are not supported")),
    };

    Ok(quote! {
        impl #imp ::zynix::token::ToTokens for #name #ty #wher {
            fn to_tokens(&self, #tokens: &mut ::zynix::TokenStream) {
                #body
            }
        }
    })
}

fn fields(fields: &Fields, tokens: &syn::Ident, via_self: bool) -> syn::Result<proc_macro2::TokenStream> {
    let mut emits = Vec::new();

    match fields {
        Fields::Named(named) => {
            for field in &named.named {
                let ident = field.ident.clone().unwrap();
                let opts = ToTokenOptions::parse(&field.attrs)?;
                let access = if via_self {
                    quote! { &self.#ident }
                } else {
                    quote! { #ident }
                };
                emits.push(one(&access, &field.ty, &opts, tokens));
            }
        }
        Fields::Unnamed(unnamed) => {
            for (i, field) in unnamed.unnamed.iter().enumerate() {
                let opts = ToTokenOptions::parse(&field.attrs)?;
                let access = if via_self {
                    let idx = Index::from(i);
                    quote! { &self.#idx }
                } else {
                    let ident = format_ident!("f{i}");
                    quote! { #ident }
                };
                emits.push(one(&access, &field.ty, &opts, tokens));
            }
        }
        Fields::Unit => {}
    }

    Ok(quote! { #(#emits)* })
}

fn variant_arm(fields: &Fields, tokens: &syn::Ident) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let mut emits = Vec::new();

    let pat = match fields {
        Fields::Named(named) => {
            let mut binds = Vec::new();

            for field in &named.named {
                let ident = field.ident.clone().unwrap();
                let opts = ToTokenOptions::parse(&field.attrs)?;
                emits.push(one(&quote! { #ident }, &field.ty, &opts, tokens));
                binds.push(ident);
            }

            quote! { { #(#binds),* } }
        }
        Fields::Unnamed(unnamed) => {
            let mut binds = Vec::new();

            for (i, field) in unnamed.unnamed.iter().enumerate() {
                let ident = format_ident!("f{i}");
                let opts = ToTokenOptions::parse(&field.attrs)?;
                emits.push(one(&quote! { #ident }, &field.ty, &opts, tokens));
                binds.push(ident);
            }

            quote! { ( #(#binds),* ) }
        }
        Fields::Unit => quote! {},
    };

    Ok((pat, quote! { #(#emits)* }))
}

fn one(access: &proc_macro2::TokenStream, ty: &Type, opts: &ToTokenOptions, tokens: &syn::Ident) -> proc_macro2::TokenStream {
    if opts.skip || opts.value {
        return quote! {};
    }

    let prefix = opts.prefix.as_ref().map(|tok| {
        quote! { <#tok as ::core::default::Default>::default().to_tokens(#tokens); }
    });
    let suffix = opts.suffix.as_ref().map(|tok| {
        quote! { <#tok as ::core::default::Default>::default().to_tokens(#tokens); }
    });

    let core = if let Some(tok) = &opts.peek {
        quote! {
            if *#access {
                <#tok as ::core::default::Default>::default().to_tokens(#tokens);
            }
        }
    } else if let Some(delim) = opts.group {
        // Re-wrap the field's tokens in its delimiter, mirroring the parse side.
        let delim = format_ident!("{delim}");
        quote! {
            {
                let mut inner = ::zynix::TokenStream::new();
                ::zynix::token::ToTokens::to_tokens(#access, &mut inner);
                #tokens.extend_one(::zynix::TokenTree::Group(
                    ::zynix::token::Group::new(::zynix::token::Delim::#delim, inner),
                ));
            }
        }
    } else if type_is(ty, "Option") {
        quote! {
            if let ::core::option::Option::Some(inner) = #access {
                ::zynix::token::ToTokens::to_tokens(inner, #tokens);
            }
        }
    } else {
        quote! { ::zynix::token::ToTokens::to_tokens(#access, #tokens); }
    };

    quote! { #prefix #core #suffix }
}
