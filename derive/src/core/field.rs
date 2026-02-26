use quote::quote;

use crate::core::Attrs;

#[derive(Clone)]
pub struct Field {
    attrs: Attrs,
    vis: syn::Visibility,
    name: FieldName,
    ty: syn::Type,
}

impl Field {
    pub fn parse(i: usize, field: &syn::Field) -> syn::Result<Self> {
        Ok(Self {
            attrs: Attrs::parse(&field.attrs)?,
            vis: field.vis.clone(),
            name: match &field.ident {
                None => syn::Index::from(i).into(),
                Some(v) => v.clone().into(),
            },
            ty: field.ty.clone(),
        })
    }

    pub fn attrs(&self) -> &Attrs {
        &self.attrs
    }

    pub fn vis(&self) -> &syn::Visibility {
        &self.vis
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn ty(&self) -> &syn::Type {
        &self.ty
    }

    pub fn display_name(&self) -> String {
        let display = self.attrs.get("display");

        display
            .iter()
            .find_map(|a| a.as_attr())
            .and_then(|attr| {
                attr.args().iter().find_map(|arg| {
                    if arg.path().is_ident("alias") {
                        arg.as_lit().and_then(|lit| match lit {
                            syn::Lit::Str(s) => Some(s.value()),
                            _ => None,
                        })
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_else(|| self.name.to_string())
    }
}

impl quote::ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = self.attrs();
        let vis = self.vis();
        let name = self.name();
        let ty = self.ty();

        tokens.extend(quote! {
            #[#attrs] #vis #name : #ty,
        });
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum FieldName {
    Index(syn::Index),
    Ident(syn::Ident),
}

impl FieldName {
    #[allow(unused)]
    pub fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }

    #[allow(unused)]
    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(_))
    }
}

impl From<syn::Index> for FieldName {
    fn from(value: syn::Index) -> Self {
        Self::Index(value)
    }
}

impl From<syn::Ident> for FieldName {
    fn from(value: syn::Ident) -> Self {
        Self::Ident(value)
    }
}

impl quote::ToTokens for FieldName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Self::Index(v) => quote!(#v),
            Self::Ident(v) => quote!(#v),
        });
    }
}

impl std::fmt::Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(v) => write!(f, "{}", v),
            Self::Index(v) => write!(f, "{}", v.index),
        }
    }
}
