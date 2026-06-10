use moxy_ast::Declaration;
use moxy_token::TokenStream;

#[proc_macro_derive(Moxy, attributes(moxy))]
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stream = TokenStream::from(tokens);
    let declr: Declaration = match stream.parse().parse() {
        Err(err) => return err.to_compile_error().into(),
        Ok(v) => v,
    };

    for _attr in declr.attrs() {}

    proc_macro::TokenStream::new()
}
