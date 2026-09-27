use moxy_diagnostic::SpanExt;
use moxy_token::{Spanner, TokenStream};

pub fn expand(tokens: TokenStream) -> TokenStream {
    let object: moxy::ast::Declaration = match moxy::parse!(tokens) {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let mut tpl_meta_list = vec![];
    let mut debug_meta_list = vec![];
    let result = object.attrs().for_each(|attr| {
        if let Some(ident) = attr.path.as_ident()
            && ident == "moxy"
        {
            attr.for_each(|meta| {
                if let Some(ident) = meta.path.as_ident() {
                    if ident == "template" {
                        tpl_meta_list.push(meta.clone());
                    } else if ident == "debug" {
                        debug_meta_list.push(meta.clone());
                    }
                }

                Ok(())
            })?;
        }

        Ok(())
    });

    if let Err(err) = result {
        return err.to_compile_error();
    }

    let Some(tpl_meta) = tpl_meta_list.first() else {
        return object.attrs().span().error("template required").emit();
    };

    let content = match &tpl_meta.content {
        moxy::ast::MetaContent::List(v) if v.delim.is_brace() => &v.tokens,
        _ => {
            return tpl_meta
                .content
                .span()
                .error("template attribute must contain a code block `{ ... }`")
                .emit();
        }
    };

    let output = moxy::template! {
        impl ::moxy::token::ToTokens for {{ object.ident() }} {
            fn to_tokens(&self, tokens: &mut ::moxy::token::TokenStream) {
                ::moxy::template::template!({{ content }}).to_tokens(tokens);
            }
        }
    };

    if let Some(debug) = debug_meta_list.first() {
        let impl_item = match moxy::parse!(output as moxy::ast::ImplItem) {
            Err(err) => return err.to_compile_error(),
            Ok(v) => v,
        };

        let object_formatted = match moxy::fmt!(&object) {
            Err(err) => return err.to_compile_error(),
            Ok(v) => v,
        };

        let impl_formatted = match moxy::fmt!(&impl_item) {
            Err(err) => return err.to_compile_error(),
            Ok(v) => v,
        };

        object.span().note(object_formatted).emit();
        debug.span().note(impl_formatted).emit();
    }

    output
}
