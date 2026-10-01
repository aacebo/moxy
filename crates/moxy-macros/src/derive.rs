use moxy_ast::List;
use moxy_diagnostic::SpanExt;
use moxy_token::{Spanner, TokenStream};

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = moxy::ast::Parser::from_tokens(&attr);
    let list = match List::<moxy::ast::Meta, moxy::ast::Token![,]>::parse_separated_nonempty(&parser) {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let mut item = match moxy::parse!(item as moxy::ast::ItemFn) {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let mut name = None;
    let mut debug = false;
    let mut attributes = None;

    for meta in list {
        if let Some(ident) = meta.path.as_ident() {
            if ident == "debug"
                && let moxy::ast::MetaContent::Unit = &meta.content
            {
                debug = true;
            } else if ident == "attributes" {
                attributes = Some(meta.clone());
            } else if let moxy::ast::MetaContent::Unit = &meta.content {
                name = Some(ident.clone());
            }
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
        #[proc_macro_derive(
            {{ &name }}

            @if (attributes.is_some()) {
                , {{ &attributes }}
            }
        )]
        pub fn {{ name.map(|v| v.to_snake_case()) }}(tokens: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            {{ &item }}

            let value = match ::moxy::parse!(tokens) {
                Ok(v) => v,
                Err(err) => return err.to_compile_error().into(),
            };

            match __call__(value) {
                Err(err) => err.to_compile_error().into(),
                Ok(v) => ::moxy::token::ToTokenStream::into_token_stream(v).into(),
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
