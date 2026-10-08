use proc_macro::TokenStream;

fn error_on_third_token(input: TokenStream) -> proc_macro2::TokenStream {
    let tokens: Vec<_> = proc_macro2::TokenStream::from(input).into_iter().collect();
    syn::Error::new(tokens[2].span(), "this token is wrong").to_compile_error()
}

#[proc_macro]
pub fn via_moxy(input: TokenStream) -> TokenStream {
    let error = error_on_third_token(input);
    let out: proc_macro2::TokenStream = moxy::template! { {{ &error }} }.into();
    out.into()
}
