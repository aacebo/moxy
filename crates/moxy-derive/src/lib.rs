extern crate self as moxy;

mod token {
    pub use moxy_token::*;
}

use moxy_ast::Declaration;
use moxy_ast::item::ItemImpl;
use moxy_diagnostic::SpanExt;
use moxy_fmt::fmt;
use moxy_template::template;
use moxy_token::{Spanner, TokenStream};

#[proc_macro_derive(ToTokens, attributes(moxy))]
pub fn derive_to_tokens(target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let target = TokenStream::from(target);
    let object: Declaration = match target.parse().parse() {
        Err(err) => return err.to_compile_error().into(),
        Ok(v) => v,
    };

    let meta_list = object.attrs().query().path("moxy").collect();
    let tpl_meta_list: Vec<_> = meta_list.iter().flat_map(|a| a.query().path("template").collect()).collect();

    if tpl_meta_list.len() > 1 {
        return tpl_meta_list[0].span().error("exactly 1 template required").emit().into();
    }

    let Some(tpl_meta) = tpl_meta_list.first() else {
        return object.attrs().span().error("template required").emit().into();
    };

    let content = match tpl_meta.as_value().and_then(|m| m.as_verbatim()) {
        Some(content) => content,
        None => {
            return tpl_meta
                .content
                .span()
                .unwrap_or(tpl_meta.span())
                .error("template attribute must contain a code block `{ ... }`")
                .emit()
                .into();
        }
    };

    let output = template! {
        impl ::moxy::token::ToTokens for {{ object.ident() }} {
            fn to_tokens(&self, tokens: &mut ::moxy::token::TokenStream) {
                ::moxy::template::template!({{ content }}).to_tokens(tokens);
            }
        }
    };

    let debug_meta_list: Vec<_> = meta_list.iter().flat_map(|a| a.query().path("debug").collect()).collect();

    if let Some(debug) = debug_meta_list.first() {
        let impl_item = match output.parse().parse::<ItemImpl>() {
            Err(err) => return err.to_compile_error().into(),
            Ok(v) => v,
        };

        let object_formatted = match fmt!(&object) {
            Err(err) => return err.to_compile_error().into(),
            Ok(v) => v,
        };

        let impl_formatted = match fmt!(&impl_item) {
            Err(err) => return err.to_compile_error().into(),
            Ok(v) => v,
        };

        object.span().note(object_formatted).emit();
        debug.span().note(impl_formatted).emit();
    }

    output.into()
}
