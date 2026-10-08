#[proc_macro]
pub fn proc_macro2_span(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: Vec<_> = proc_macro2::TokenStream::from(input).into_iter().collect();
    let error = syn::Error::new(tokens[2].span(), "this token is wrong").to_compile_error();
    let out: proc_macro2::TokenStream = moxy::template! { {{ &error }} }.into();
    out.into()
}
