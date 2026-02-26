pub(crate) trait Render {
    type Args;

    fn render(&self, args: Self::Args) -> syn::Result<proc_macro2::TokenStream>;
}

impl<T: Render> Render for syn::Result<T> {
    type Args = T::Args;

    fn render(&self, args: Self::Args) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Err(err) => Err(err.clone()),
            Ok(v) => v.render(args),
        }
    }
}

pub(crate) trait Error: syn::spanned::Spanned {
    fn error(&self, message: &str) -> syn::Error {
        syn::Error::new(self.span(), message)
    }
}

impl<T: syn::spanned::Spanned> Error for T {}
