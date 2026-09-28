use moxy_diagnostic::SpanExt;
use moxy_token::{Spanner, TokenStream};

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = if attr.is_empty() {
        None
    } else {
        match moxy::parse!(attr as moxy::ast::Meta) {
            Err(err) => return err.to_compile_error(),
            Ok(v) => Some(v),
        }
    };

    let mut item = match moxy::parse!(item as moxy::ast::ItemFn) {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let mut name = None;
    let mut debug = false;

    if let Some(meta) = attr {
        if let Some(ident) = meta.path.as_ident()
            && ident == "debug"
            && let moxy::ast::MetaContent::Unit = &meta.content
        {
            debug = true;
        }

        if let Some(ident) = meta.path.as_ident()
            && let moxy::ast::MetaContent::Unit = &meta.content
        {
            name = Some(ident.clone());
        }
    }

    if !item.vis.is_public() {
        return item.vis.span().error("proc macros must be pub").emit();
    }

    if item.sig.params.inputs.len() != 1 {
        return item.sig.params.span().error("proc macro derive signature invalid").emit();
    }

    let moxy::ast::ReturnType::Type(_, _) = &item.sig.output else {
        return item.sig.output.span().error("proc macro derive signature invalid").emit();
    };

    item.sig.ident = match moxy::parse!("__call__") {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let out = moxy::template! {
        #[proc_macro_derive({{ &name }})]
        pub fn {{ &item.sig.ident }}(tokens: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            {{ &item }}

            let value = match ::moxy::parse!(tokens) {
                Ok(v) => v,
                Err(err) => return err.to_compile_error().into(),
            };

            match __call__(value) {
                Err(err) => err.to_compile_error().into(),
                Ok(v) => v.into(),
            }
        }
    };

    if debug {
        let parsed = match moxy::parse!(out as moxy::ast::ItemFn) {
            Err(err) => return err.to_compile_error(),
            Ok(v) => v,
        };

        let message = match moxy::fmt!(&parsed) {
            Err(err) => return err.to_compile_error(),
            Ok(v) => v,
        };

        item.sig.ident.span().note(message).emit();
    }

    out
}
