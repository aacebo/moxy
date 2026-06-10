use moxy_ast::Declaration;
use moxy_token::TokenStream;

#[proc_macro_derive(ToTokens, attributes(template))]
pub fn derive_to_tokens(target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let target = TokenStream::from(target);
    let _: Declaration = match target.parse().parse() {
        Err(err) => return err.to_compile_error().into(),
        Ok(v) => v,
    };

    Default::default()
}
