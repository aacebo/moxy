use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Type};

use crate::options::{ParseOptions, type_is};

pub fn expand(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let (imp, ty, wher) = input.generics.split_for_impl();
    let stream = format_ident!("stream");

    let body = match &input.data {
        Data::Struct(s) => {
            let (stmts, ctor) = fields(&s.fields, &stream)?;
            quote! { #(#stmts)* Ok(Self #ctor) }
        }
        Data::Enum(e) => {
            let mut arms = Vec::new();

            for variant in &e.variants {
                let vname = &variant.ident;
                let vopts = ParseOptions::parse(&variant.attrs)?;
                let fork = format_ident!("fork");
                let (stmts, ctor) = fields(&variant.fields, &fork)?;

                let gate = match &vopts.peek {
                    Some(tok) => quote! {
                        if #fork.peek::<#tok>().is_none() {
                            return Err(::zynix::parse::ParseError::from(
                                ::zynix::token::LexError::new(#fork.span())
                            ));
                        }
                    },
                    None => quote! {},
                };

                arms.push(quote! {
                    {
                        let mut #fork = #stream.fork();
                        let attempt = (|| -> Result<Self, ::zynix::parse::ParseError> {
                            #gate
                            #(#stmts)*
                            Ok(Self::#vname #ctor)
                        })();

                        if let Ok(value) = attempt {
                            #stream.seek(&#fork);
                            return Ok(value);
                        }
                    }
                });
            }

            let nm = name.to_string();
            quote! {
                #(#arms)*
                Err(::zynix::parse::ParseError::from(
                    ::zynix::token::LexError::new(#stream.span())
                        .message(concat!("expected `", #nm, "`"))
                ))
            }
        }
        Data::Union(_) => return Err(syn::Error::new(input.span(), "unions are not supported")),
    };

    Ok(quote! {
        impl #imp ::zynix::Parse for #name #ty #wher {
            fn parse(#stream: &mut ::zynix::parse::ParseStream) -> Result<Self, ::zynix::parse::ParseError> {
                #body
            }
        }
    })
}

/// Returns (per-field binding statements, constructor body).
fn fields(fields: &Fields, stream: &syn::Ident) -> syn::Result<(Vec<proc_macro2::TokenStream>, proc_macro2::TokenStream)> {
    let mut stmts = Vec::new();

    match fields {
        Fields::Named(named) => {
            let mut names = Vec::new();

            for field in &named.named {
                let ident = field.ident.clone().unwrap();
                let opts = ParseOptions::parse(&field.attrs)?;
                stmts.push(one(&ident, &field.ty, &opts, stream)?);
                names.push(ident);
            }

            Ok((stmts, quote! { { #(#names),* } }))
        }
        Fields::Unnamed(unnamed) => {
            let mut names = Vec::new();

            for (i, field) in unnamed.unnamed.iter().enumerate() {
                let ident = format_ident!("f{i}");
                let opts = ParseOptions::parse(&field.attrs)?;
                stmts.push(one(&ident, &field.ty, &opts, stream)?);
                names.push(ident);
            }

            Ok((stmts, quote! { ( #(#names),* ) }))
        }
        Fields::Unit => Ok((stmts, quote! {})),
    }
}

fn one(binding: &syn::Ident, ty: &Type, opts: &ParseOptions, stream: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    if opts.skip {
        return Ok(quote! { let #binding = ::core::default::Default::default(); });
    }

    if let Some(expr) = &opts.value {
        return Ok(quote! { let #binding = #expr; });
    }

    let prefix = opts.prefix.as_ref().map(|tok| quote! { let _ = #stream.parse::<#tok>()?; });
    let suffix = opts.suffix.as_ref().map(|tok| quote! { let _ = #stream.parse::<#tok>()?; });

    let core = if let Some(tok) = &opts.peek {
        quote! {
            let #binding = if #stream.peek::<#tok>().is_some() {
                let _ = #stream.parse::<#tok>()?;
                true
            } else {
                false
            };
        }
    } else if let Some(delim) = opts.group {
        // Parse the field from inside the delimiter; honor separated/terminated
        // for a `Punctuated` field, else parse a single `T`.
        let delim = format_ident!("{delim}");
        let inner = value_expr(opts, ty, stream);
        quote! {
            let #binding = {
                let group = #stream.parse_group(::zynix::token::Delim::#delim)?;
                let mut group_stream = group.parse();
                let #stream = &mut group_stream;
                #inner
            };
        }
    } else {
        let value = value_expr(opts, ty, stream);
        quote! { let #binding = #value; }
    };

    Ok(quote! { #prefix #core #suffix })
}

/// The expression that produces a field's value from `stream`, honoring
/// `call`/`separated`/`terminated` and the `Option<T>`/`Box<T>` field shapes.
fn value_expr(opts: &ParseOptions, ty: &Type, stream: &syn::Ident) -> proc_macro2::TokenStream {
    if let Some(path) = &opts.call {
        quote! { #path(#stream)? }
    } else if opts.separated {
        quote! { ::zynix::ast::Punctuated::parse_separated_nonempty(#stream)? }
    } else if opts.terminated {
        quote! { ::zynix::ast::Punctuated::parse_terminated(#stream)? }
    } else if type_is(ty, "Option") {
        quote! {{
            let mut fork = #stream.fork();
            match ::zynix::Parse::parse(&mut fork) {
                Ok(v) => { #stream.seek(&fork); Some(v) }
                Err(_) => None,
            }
        }}
    } else if type_is(ty, "Vec") {
        // Greedily parse elements until one stops matching.
        quote! {{
            let mut items = ::std::vec::Vec::new();
            loop {
                let mut fork = #stream.fork();
                match ::zynix::Parse::parse(&mut fork) {
                    Ok(v) => { #stream.seek(&fork); items.push(v); }
                    Err(_) => break,
                }
            }
            items
        }}
    } else if type_is(ty, "Box") {
        quote! { ::std::boxed::Box::new(#stream.parse()?) }
    } else {
        quote! { #stream.parse()? }
    }
}
