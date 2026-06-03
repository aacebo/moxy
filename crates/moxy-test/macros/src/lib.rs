use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn compile_error(_args: TokenStream, target: TokenStream) -> TokenStream {
    target
}

#[proc_macro_attribute]
pub fn error(_args: TokenStream, target: TokenStream) -> TokenStream {
    target
}

#[proc_macro_attribute]
pub fn warn(_args: TokenStream, target: TokenStream) -> TokenStream {
    target
}

#[proc_macro_attribute]
pub fn note(_args: TokenStream, target: TokenStream) -> TokenStream {
    target
}

#[proc_macro_attribute]
pub fn help(_args: TokenStream, target: TokenStream) -> TokenStream {
    target
}
