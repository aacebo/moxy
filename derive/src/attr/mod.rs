mod arg;

pub use arg::*;

use quote::quote;
use syn::punctuated::Punctuated;

///
/// ## Attr
/// represents a moxy attribute and its
/// arguments list
/// ### Example
/// `#[moxy(display(format = compact))]`
///
#[derive(Clone)]
pub struct Attr {
    pub path: syn::Path,
    pub args: Punctuated<Arg, syn::Token![,]>,
}

impl Attr {
    pub fn parse(attr: &syn::Attribute) -> syn::Result<Self> {
        let args = attr.parse_args_with(
            syn::punctuated::Punctuated::<Arg, syn::Token![,]>::parse_terminated,
        )?;

        Ok(Self {
            path: attr.path().clone(),
            args,
        })
    }

    pub fn is_moxy(&self) -> bool {
        self.path.is_ident("moxy")
    }

    pub fn get(&self, ident: &str) -> Option<&Arg> {
        self.args.iter().find(|arg| arg.path().is_ident(ident))
    }
}

impl quote::ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = &self.path;
        let args = &self.args;

        tokens.extend(quote!(#path(#args)));
    }
}
