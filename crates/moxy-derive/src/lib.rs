//! # Moxy derive
//!
//! Derive support for `moxy::token::ToTokens`.
//!
//! ## Syntax
//!
//! `#[derive(ToTokens)]` requires `#[moxy(template { ... })]`. Its template is
//! expanded by `moxy::template!`, and `self` is available in the generated
//! `to_tokens` method.
//!
//! ```ignore
//! #[derive(moxy::ToTokens)]
//! #[moxy(template { struct {{ self.name }}; })]
//! struct Generated { name: String }
//! ```
//!
//! ## Debugging expansions
//!
//! Add `#[moxy(debug)]` alongside the template attribute to emit compiler notes
//! with the parsed input declaration and the generated `ToTokens`
//! implementation. The option is intended for inspecting derive output during
//! development and does not change the generated implementation.
//!
//! ```ignore
//! #[derive(moxy::ToTokens)]
//! #[moxy(template { struct {{ self.name }}; })]
//! #[moxy(debug)]
//! struct Generated { name: String }
//! ```

extern crate self as moxy;

mod token {
    pub use moxy_token::*;
}

use moxy_ast::item::ItemImpl;
use moxy_ast::{Declaration, MetaContent, Parse, Parser};
use moxy_diagnostic::SpanExt;
use moxy_fmt::fmt;
use moxy_template::template;
use moxy_token::{Spanner, TokenStream};

/// Derives [`moxy::token::ToTokens`] from a token template.
///
/// The derive generates an implementation of `ToTokens` for the annotated
/// type. Its required `#[moxy(template { ... })]` attribute contains a
/// [`moxy::template!`] body; the generated method makes `self` available to
/// template interpolations.
///
/// # Example
///
/// ```ignore
/// use moxy::token::ToTokenStream;
///
/// #[derive(moxy::ToTokens)]
/// #[moxy(template {
///     pub const GENERATED: &str = {{ self.value }};
/// })]
/// struct Model {
///     value: String,
/// }
///
/// let tokens = Model { value: "ready".into() }.to_token_stream();
/// assert_eq!(tokens.to_string(), "pub const GENERATED : & str = \"ready\" ;");
/// ```
///
/// # Attributes
///
/// - `#[moxy(template { ... })]` is required exactly once. Its value must be a
///   braced Rust token block accepted by `moxy::template!`.
/// - `#[moxy(debug)]` is optional. It emits compiler notes containing the parsed
///   input declaration and the generated `ToTokens` implementation, which is
///   useful when inspecting an expansion during development.
///
/// A missing, repeated, or malformed template attribute produces a
/// span-targeted compiler error.
#[proc_macro_derive(ToTokens, attributes(moxy))]
pub fn derive_to_tokens(target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let target = TokenStream::from(target);
    let object: Declaration = match Declaration::parse(&Parser::from_tokens(&target)) {
        Err(err) => return err.to_compile_error().into(),
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
        return err.to_compile_error().into();
    }

    let Some(tpl_meta) = tpl_meta_list.first() else {
        return object.attrs().span().error("template required").emit().into();
    };

    let content = match &tpl_meta.content {
        MetaContent::List(v) if v.delim.is_brace() => &v.tokens,
        _ => {
            return tpl_meta
                .content
                .span()
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

    if let Some(debug) = debug_meta_list.first() {
        let impl_item = match ItemImpl::parse(&Parser::from_tokens(&output)) {
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
