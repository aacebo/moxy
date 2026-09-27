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

    let mut name = item.sig.ident.clone();
    let mut debug = false;

    if let Some(meta) = attr {
        if let Some(ident) = meta.path.as_ident()
            && ident == "debug"
            && let moxy::ast::MetaContent::Unit = &meta.content
        {
            debug = true;
        }

        if let Some(ident) = meta.path.as_ident()
            && ident == "name"
            && let moxy::ast::MetaContent::Expr { eq: _, expr } = &meta.content
        {
            let expr = match moxy::parse!(expr as moxy::ast::Expr) {
                Err(err) => return err.to_compile_error(),
                Ok(v) => v,
            };

            name = if let moxy::ast::Expr::Lit(expr) = &expr
                && let moxy::token::Lit::Str(lit) = &expr.lit
            {
                moxy::token::Ident::new(lit.value()).with_span(lit.span())
            } else if let moxy::ast::Expr::Path(expr) = &expr
                && let Some(ident) = expr.path.as_ident()
            {
                ident.clone()
            } else {
                return moxy::ast::ParseError::new(expr.span(), "name can be a string literal or identifier").to_compile_error();
            };
        }
    }

    if !item.vis.is_public() {
        return item.vis.span().error("proc macros must be pub").emit();
    }

    if item.sig.params.inputs.len() != 2 {
        return item.sig.params.span().error("proc macro attribute signature invalid").emit();
    }

    let moxy::ast::ReturnType::Type(_, _) = &item.sig.output else {
        return item.sig.output.span().error("proc macro attribute signature invalid").emit();
    };

    item.sig.ident = match moxy::parse!("__call__") {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let out = moxy::template! {
        #[proc_macro_attribute]
        pub fn {{ &name }}(meta: ::proc_macro::TokenStream, item: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            {{ &item }}

            match __call__(meta.into(), item.into()) {
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
