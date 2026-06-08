extern crate proc_macro;

mod ast;

use moxy_ast::sig::FnParam;
use moxy_ast::{Meta, item};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Parse, ToTokenStream, ToTokens, TokenStream, TokenTree};

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

#[doc = "A parsed template: a sequence of nodes (literal tokens, interpolations, and control flow)."]
#[derive(Debug, Clone)]
struct Template {
    pub nodes: Vec<ast::Node>,
}

impl Template {
    pub fn expand(&self) -> TokenStream {
        use std::str::FromStr;

        let mut body = TokenStream::from_str("let mut __moxy_tmpl = ::moxy_token::TokenStream::new();").unwrap();
        self.to_tokens(&mut body);
        body.extend(TokenStream::from_str("__moxy_tmpl").unwrap());
        TokenStream::from(vec![TokenTree::Group(Group::new(Delim::Brace, body))])
    }
}

impl Parse for Template {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let nodes = stream.parse::<Vec<ast::Node>>()?;
        Ok(Self { nodes })
    }
}

impl ToTokens for Template {
    fn to_tokens(&self, out: &mut TokenStream) {
        for node in &self.nodes {
            node.to_tokens(out);
        }
    }
}

#[proc_macro_attribute]
pub fn expand(_args: proc_macro::TokenStream, target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stream = target.into_token_stream();
    let mut stream = stream.parse();
    let mut target: item::ItemFn = match stream.parse() {
        Err(err) => return err.to_compile_error().into(),
        Ok(v) => v,
    };

    for param in &mut target.sig.params.inputs {
        if let FnParam::Typed(p) = param {
            p.attrs.retain(|attr| match &attr.meta.inner {
                Meta::Path(v) => v.to_token_stream().to_string() != "parse",
                _ => true,
            });
        }
    }

    target.into_token_stream().into()
}
