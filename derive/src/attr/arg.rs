use quote::quote;
use syn::punctuated::Punctuated;

#[derive(Clone)]
pub enum Arg {
    /// `#[moxy(deref)]`
    Flag(syn::Path),
    /// `#[moxy(display(format = "hi {name}!"))]`
    Literal(syn::Path, syn::Lit),
    /// `#[moxy(display(format = compact))]`
    Ident(syn::Path, syn::Ident),
    /// `#[moxy(display(explicit, format = compact))]`
    List(syn::Path, Punctuated<Self, syn::Token![,]>),
}

impl Arg {
    pub fn from_flag(path: syn::Path) -> Self {
        Self::Flag(path)
    }

    pub fn from_literal(path: syn::Path, value: syn::Lit) -> Self {
        Self::Literal(path, value)
    }

    pub fn from_ident(path: syn::Path, value: syn::Ident) -> Self {
        Self::Ident(path, value)
    }

    pub fn from_list(path: syn::Path, value: Punctuated<Self, syn::Token![,]>) -> Self {
        Self::List(path, value)
    }

    pub fn path(&self) -> &syn::Path {
        match self {
            Self::Flag(path) => path,
            Self::Literal(path, _) => path,
            Self::Ident(path, _) => path,
            Self::List(path, _) => path,
        }
    }

    pub fn lit(&self) -> Option<&syn::Lit> {
        match self {
            Self::Literal(_, v) => Some(v),
            _ => None,
        }
    }

    pub fn ident(&self) -> Option<&syn::Ident> {
        match self {
            Self::Ident(_, v) => Some(v),
            _ => None,
        }
    }

    pub fn args(&self) -> Option<&Punctuated<Self, syn::Token![,]>> {
        match self {
            Self::List(_, v) => Some(v),
            _ => None,
        }
    }

    pub fn error(&self, message: &str) -> proc_macro2::TokenStream {
        syn::Error::new_spanned(self.path(), message).to_compile_error()
    }
}

impl quote::ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Self::Flag(path) => quote!(#path),
            Self::Literal(path, value) => quote!(#path = #value),
            Self::Ident(path, ident) => quote!(#path = #ident),
            Self::List(path, args) => quote!(#path(#args)),
        });
    }
}

impl syn::parse::Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: syn::Path = input.parse()?;

        if input.peek(syn::Token![=]) {
            input.parse::<syn::Token![=]>()?;

            if input.peek(syn::Ident) {
                Ok(Arg::from_ident(path, input.parse()?))
            } else {
                Ok(Arg::from_literal(path, input.parse()?))
            }
        } else if input.peek(syn::token::Paren) {
            let list;
            let _ = syn::parenthesized!(list in input);
            Ok(Arg::from_list(path, Punctuated::parse_terminated(&list)?))
        } else {
            Ok(Arg::from_flag(path))
        }
    }
}
