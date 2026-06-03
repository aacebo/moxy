extern crate proc_macro;

mod ast;
mod template;

use moxy_token::{Parse, ToTokenStream, ToTokens, TokenStream};

use crate::template::Template;

#[proc_macro]
pub fn template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ts = TokenStream::new();
    input.to_tokens(&mut ts);

    let expanded = match Template::parse(&mut ts.parse()) {
        Ok(tmpl) => tmpl.expand(),
        Err(e) => e.into_token_stream(),
    };

    let mut out = proc_macro::TokenStream::new();
    expanded.to_tokens(&mut out);
    out
}
