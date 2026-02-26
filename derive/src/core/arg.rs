use quote::quote;
use syn::punctuated::Punctuated;

use crate::{Error, core::Attr};

#[derive(Clone, PartialEq, Eq)]
pub enum Arg {
    /// `#[moxy(deref)]`
    Flag(syn::Path),
    /// `#[moxy(display(format = "hi {name}!"))]`
    Literal(syn::Path, syn::Lit),
    /// `#[moxy(display(format = compact))]`
    Ident(syn::Path, syn::Ident),
    /// `#[moxy(display(explicit, format = compact))]`
    Attr(Attr),
    /// `#[moxy(display("{}", self.sign()))]`
    Expr(syn::Path, syn::Expr),
}

impl Arg {
    pub fn from_flag(path: syn::Path) -> Self {
        Self::Flag(path)
    }

    pub fn from_lit(path: syn::Path, value: syn::Lit) -> Self {
        Self::Literal(path, value)
    }

    pub fn from_ident(path: syn::Path, value: syn::Ident) -> Self {
        Self::Ident(path, value)
    }

    pub fn from_attr(attr: Attr) -> Self {
        Self::Attr(attr)
    }

    pub fn from_expr(path: syn::Path, expr: syn::Expr) -> Self {
        Self::Expr(path, expr)
    }

    #[allow(unused)]
    pub fn is_flag(&self) -> bool {
        matches!(self, Self::Flag(_))
    }

    #[allow(unused)]
    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Literal(_, _))
    }

    #[allow(unused)]
    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(_, _))
    }

    #[allow(unused)]
    pub fn is_attr(&self) -> bool {
        matches!(self, Self::Attr(_))
    }

    #[allow(unused)]
    pub fn is_expr(&self) -> bool {
        matches!(self, Self::Expr(_, _))
    }

    pub fn path(&self) -> &syn::Path {
        match self {
            Self::Flag(path) => path,
            Self::Literal(path, _) => path,
            Self::Ident(path, _) => path,
            Self::Attr(attr) => attr.path(),
            Self::Expr(path, _) => path,
        }
    }

    #[allow(unused)]
    pub fn as_lit(&self) -> Option<&syn::Lit> {
        match self {
            Self::Literal(_, v) => Some(v),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn as_ident(&self) -> Option<&syn::Ident> {
        match self {
            Self::Ident(_, v) => Some(v),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn as_attr(&self) -> Option<&Attr> {
        match self {
            Self::Attr(attr) => Some(attr),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn to_lit(self) -> Option<syn::Lit> {
        match self {
            Self::Literal(_, v) => Some(v),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn to_ident(self) -> Option<syn::Ident> {
        match self {
            Self::Ident(_, v) => Some(v),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn to_attr(self) -> Option<Attr> {
        match self {
            Self::Attr(attr) => Some(attr),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn as_expr(&self) -> Option<&syn::Expr> {
        match self {
            Self::Expr(_, expr) => Some(expr),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn error(&self, message: &str) -> proc_macro2::TokenStream {
        self.path().error(message).to_compile_error()
    }
}

impl quote::ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Self::Flag(path) => quote!(#path),
            Self::Literal(path, value) => quote!(#path = #value),
            Self::Ident(path, ident) => quote!(#path = #ident),
            Self::Attr(attr) => quote!(#attr),
            Self::Expr(_, expr) => quote!(#expr),
        });
    }
}

impl syn::parse::Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            let lit: syn::Lit = input.parse()?;
            let path: syn::Path = syn::parse_quote!(__value);
            return Ok(Arg::from_lit(path, lit));
        }

        let fork = input.fork();
        if fork.parse::<syn::Path>().is_ok() {
            if fork.peek(syn::Token![=]) {
                let path: syn::Path = input.parse()?;
                input.parse::<syn::Token![=]>()?;
                if input.peek(syn::Ident) {
                    return Ok(Arg::from_ident(path, input.parse()?));
                } else {
                    return Ok(Arg::from_lit(path, input.parse()?));
                }
            }

            if fork.peek(syn::token::Paren) {
                let path: syn::Path = input.parse()?;
                let list;
                let _ = syn::parenthesized!(list in input);
                let items = Punctuated::<Arg, syn::Token![,]>::parse_terminated(&list)?;
                return Ok(Arg::from_attr(Attr::new(path, items)));
            }

            if fork.is_empty() || fork.peek(syn::Token![,]) {
                let path: syn::Path = input.parse()?;
                return Ok(Arg::from_flag(path));
            }
        }

        let expr: syn::Expr = input.parse()?;
        let path: syn::Path = syn::parse_quote!(__expr);
        Ok(Arg::from_expr(path, expr))
    }
}
