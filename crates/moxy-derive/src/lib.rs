use moxy_ast::Declaration;
use moxy_ast::item::ItemImpl;
use moxy_diagnostic::SpanExt;
use moxy_fmt::fmt;
use moxy_template::template;
use moxy_token::{Spanner, TokenStream};

#[proc_macro_derive(ToTokens, attributes(template, debug))]
pub fn derive_to_tokens(target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let target = TokenStream::from(target);
    let object: Declaration = match target.parse().parse() {
        Err(err) => {
            return err.span().error(target).emit().into();
        }
        Ok(v) => v,
    };

    let tpl_meta_list = object.attrs().query().path("template").collect();
    let tpl_meta = tpl_meta_list.first().unwrap();
    let content = tpl_meta.as_value().unwrap().as_verbatim().unwrap().clone();
    let output = template! {
        impl moxy_token::ToTokens for {{ object.ident() }} {
            fn to_tokens(&self, tokens: &mut moxy_token::TokenStream) {
                {{ content }}
            }
        }
    };

    let debug_meta_list = object.attrs().query().path("debug").collect();

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
