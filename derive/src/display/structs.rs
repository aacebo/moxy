use proc_macro2::TokenStream;

use crate::{Render, params};

#[derive(Clone, Default)]
pub struct StructSyntax;

impl Render for StructSyntax {
    type Args = params::StructParams;

    fn render(&self, _args: Self::Args) -> syn::Result<TokenStream> {
        todo!()
    }
}
