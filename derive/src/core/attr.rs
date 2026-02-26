use quote::quote;
use syn::punctuated::Punctuated;

use crate::core::Arg;

#[repr(transparent)]
#[derive(Clone)]
pub struct Attrs(Vec<Attr>);

impl Attrs {
    pub fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        Ok(Self(
            attrs
                .iter()
                .filter_map(|attr| Attr::parse(attr).ok())
                .collect(),
        ))
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[allow(unused)]
    pub fn iter(&self) -> std::slice::Iter<'_, Attr> {
        self.0.iter()
    }

    pub fn exists(&self, name: &str) -> bool {
        self.0.iter().any(|a| a.exists(name))
    }

    /// get all the arguments that belong to
    /// the given attribute name
    #[allow(unused)]
    pub fn get(&self, name: &str) -> Box<[Arg]> {
        self.0
            .iter()
            .filter(|a| a.exists(name))
            .flat_map(|a| a.args.clone())
            .fold(vec![], |mut acc, arg| {
                if !acc.contains(&arg) {
                    acc.push(arg);
                }

                acc
            })
            .into_boxed_slice()
    }
}

impl quote::ToTokens for Attrs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let items = &self.0;
        tokens.extend(quote!(#(#items,)*));
    }
}

///
/// ## Attr
/// represents a moxy attribute and its
/// arguments list
/// ### Example
/// `#[moxy(display(format = compact))]`
///
#[derive(Clone, PartialEq, Eq)]
pub struct Attr {
    path: syn::Path,
    args: Punctuated<Arg, syn::Token![,]>,
}

impl Attr {
    pub fn new(path: syn::Path, args: Punctuated<Arg, syn::Token![,]>) -> Self {
        Self { path, args }
    }

    pub fn parse(attr: &syn::Attribute) -> syn::Result<Self> {
        let args = attr.parse_args_with(
            syn::punctuated::Punctuated::<Arg, syn::Token![,]>::parse_terminated,
        )?;

        Ok(Self {
            path: attr.path().clone(),
            args,
        })
    }

    pub fn path(&self) -> &syn::Path {
        &self.path
    }

    pub fn args(&self) -> &Punctuated<Arg, syn::Token![,]> {
        &self.args
    }

    pub fn exists(&self, ident: &str) -> bool {
        self.args.iter().any(|arg| arg.path().is_ident(ident))
    }

    #[allow(unused)]
    pub fn get(&self, ident: &str) -> Option<&Arg> {
        self.args.iter().find(|arg| arg.path().is_ident(ident))
    }
}

impl quote::ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = self.path();
        let args = self.args();

        tokens.extend(quote!(#path(#args)));
    }
}
