#[macro_use]
extern crate moxy_template;

#[macro_use]
extern crate moxy_diagnostic;

use moxy_ast::Declaration;
use moxy_token::{Spanner, TokenStream};

#[proc_macro_derive(Moxy, attributes(moxy))]
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stream = TokenStream::from(tokens);
    let declr: Declaration = match stream.parse().parse() {
        Err(err) => return err.to_compile_error().into(),
        Ok(v) => v,
    };

    let out = template! {
        impl {{ declr.ident() }} {
            pub fn {{ declr.ident() }}_test(&self) -> bool {
                true
            }
        }
    };

    note!(&out, span = declr.span()).emit();
    out.into()
}
