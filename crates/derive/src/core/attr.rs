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
    /// the given attribute name, merging duplicate nested attrs and
    /// returning an error on conflicting values
    #[allow(unused)]
    pub fn get(&self, name: &str) -> syn::Result<Vec<Arg>> {
        let raw: Vec<Arg> = self
            .0
            .iter()
            .filter(|a| a.path().is_ident("moxy") && a.exists(name))
            .flat_map(|a| a.get(name).cloned())
            .collect();

        let mut result: Vec<Arg> = vec![];

        for arg in raw {
            match &arg {
                Arg::Attr(incoming) => {
                    let pos = result.iter().position(|existing| {
                        matches!(existing, Arg::Attr(ea)
                            if ea.path().get_ident() == incoming.path().get_ident())
                    });

                    if let Some(i) = pos {
                        if let Arg::Attr(existing) = result[i].clone() {
                            result[i] = Arg::Attr(existing.merge(incoming)?);
                        }
                    } else {
                        result.push(arg);
                    }
                }
                _ => {
                    if !result.contains(&arg) {
                        result.push(arg);
                    }
                }
            }
        }

        Ok(result)
    }
}

impl quote::ToTokens for Attrs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let items = &self.0;
        tokens.extend(quote!(#(#items,)*));
    }
}

impl std::ops::Deref for Attrs {
    type Target = [Attr];

    fn deref(&self) -> &Self::Target {
        &self.0
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

    /// Merge another `Attr`'s args into `self`, returning a new merged `Attr`.
    /// - Identical args are deduplicated silently.
    /// - Args with the same path ident but different values produce a `syn::Error`.
    /// - Novel args are appended.
    pub fn merge(&self, other: &Attr) -> syn::Result<Attr> {
        let mut merged: Punctuated<Arg, syn::Token![,]> = self.args.clone();

        for arg in other.args.iter() {
            let existing = merged
                .iter()
                .find(|a| a.path().get_ident() == arg.path().get_ident());

            match existing {
                Some(a) if a == arg => {
                    // identical — deduplicate silently
                }
                Some(_) => {
                    return Err(syn::Error::new_spanned(
                        arg.path(),
                        format!(
                            "conflicting values for `{}`",
                            arg.path()
                                .get_ident()
                                .map(|i| i.to_string())
                                .unwrap_or_default()
                        ),
                    ));
                }
                None => {
                    merged.push(arg.clone());
                }
            }
        }

        Ok(Attr::new(self.path.clone(), merged))
    }
}

impl quote::ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = self.path();
        let args = self.args();

        tokens.extend(quote!(#path(#args)));
    }
}
