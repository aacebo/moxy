extern crate proc_macro;

mod ast;

use ast::Template;
use moxy_token::{Parse, ToTokens, TokenStream};

/// Build a [`moxy_token::TokenStream`] at runtime from a template, in the style
/// of `quote!`.
///
/// The macro expands to a block that constructs and returns a `TokenStream`.
/// Interpolations and control flow are evaluated against the surrounding scope:
///
/// - `{{ expr }}` splices the runtime value of `expr` (via [`moxy_token::ToTokens`]),
///   preserving its source spans.
/// - `@for (binding in iter) { … }`, `@if (cond) { … } @else { … }`, and
///   `@match (expr) { pat => { … }, … }` run as real control flow, appending to
///   the output as they execute.
///
/// # Example
///
/// ```ignore
/// let items = vec!["a", "b", "c"];
/// let tokens = template! {
///     @for (item in items) {
///         {{ item }}
///     }
/// };
/// assert_eq!(tokens.to_string(), "a b c");
/// ```
///
/// A malformed template produces a span-targeted compile error.
#[proc_macro]
pub fn template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ts = TokenStream::new();
    input.to_tokens(&mut ts);

    let expanded = match Template::parse(&mut ts.parse()) {
        Ok(tmpl) => tmpl.expand(),
        Err(e) => e.to_compile_error(),
    };

    let mut out = proc_macro::TokenStream::new();
    expanded.to_tokens(&mut out);
    out
}

// #[proc_macro_attribute]
// pub fn expand(_args: proc_macro::TokenStream, target: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let stream = target.into_token_stream();
//     let mut stream = stream.parse();
//     let mut target: item::ItemFn = match stream.parse() {
//         Err(err) => return err.to_compile_error().into(),
//         Ok(v) => v,
//     };

//     for param in &mut target.sig.params.inputs {
//         if let FnParam::Typed(p) = param {
//             p.attrs.retain(|attr| match &attr.meta.inner {
//                 Meta::Path(v) => v.to_token_stream().to_string() != "parse",
//                 _ => true,
//             });
//         }
//     }

//     target.into_token_stream().into()
// }
