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

mod ast {
    pub use moxy_ast::*;
}

mod token {
    pub use moxy_token::*;
}

use moxy_ast::{Declaration, ItemImpl, MetaContent, Parse, Parser, parse};
use moxy_diagnostic::SpanExt;
use moxy_fmt::fmt;
use moxy_template::template;
use moxy_token::{Spanner, ToTokenStream, TokenStream};

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

/// Turns a public token-to-token function into a function-like procedural macro.
///
/// The annotated function must accept one [`TokenStream`] argument and return
/// `Result<TokenStream, ParseError>`. By default, the generated macro has the
/// same name as the function. Set `name` to export it under another identifier.
///
/// # Examples
///
/// ```ignore
/// use moxy::ast::ParseError;
/// use moxy::token::TokenStream;
///
/// #[moxy::function(name = "hello")]
/// pub fn expand(tokens: TokenStream) -> Result<TokenStream, ParseError> {
///     Ok(moxy::template! { println!("hello"); })
/// }
///
/// hello!();
/// ```
#[proc_macro_attribute]
pub fn function(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr: TokenStream = attr.into();
    let attr = if attr.is_empty() {
        None
    } else {
        match parse!(attr as moxy::ast::Meta) {
            Err(err) => return err.into_token_stream().into(),
            Ok(v) => Some(v),
        }
    };

    let item: TokenStream = item.into();
    let mut item = match parse!(item as moxy::ast::ItemFn) {
        Err(err) => return err.into_token_stream().into(),
        Ok(v) => v,
    };

    let mut name = item.sig.ident.clone();
    if let Some(meta) = attr {
        if let Some(ident) = meta.path.as_ident()
            && ident == "name"
            && let MetaContent::Expr { eq: _, expr } = &meta.content
        {
            let expr = match parse!(expr as moxy::ast::Expr) {
                Err(err) => return err.into_token_stream().into(),
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
                return moxy::ast::ParseError::new(expr.span(), "name can be a string literal or identifier")
                    .into_token_stream()
                    .into();
            };
        }
    }

    if !item.vis.is_public() {
        return item.vis.span().error("proc macros must be pub").emit().into();
    }

    let Some(_) = item.sig.params.inputs.first() else {
        return item
            .sig
            .params
            .span()
            .error("proc macro function signature invalid")
            .emit()
            .into();
    };

    let moxy::ast::ReturnType::Type(_, _) = &item.sig.output else {
        return item
            .sig
            .output
            .span()
            .error("proc macro function signature invalid")
            .emit()
            .into();
    };

    if item.sig.params.inputs.len() > 1 {
        return item
            .sig
            .params
            .span()
            .error("proc macro function signature invalid")
            .emit()
            .into();
    }

    item.sig.ident = match parse!("__call__") {
        Err(err) => return err.to_compile_error().into(),
        Ok(v) => v,
    };

    template! {
        #[proc_macro]
        pub fn {{ &name }}(tokens: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            {{ &item }}

            match __call__(tokens.into()) {
                Err(err) => err.to_compile_error().into(),
                Ok(v) => v.into(),
            }
        }
    }
    .into()
}
