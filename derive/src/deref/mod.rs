mod structs;

use crate::Render;

pub fn parse(input: syn::DeriveInput) -> Macro {
    match input.data.clone() {
        syn::Data::Struct(v) => structs::StructMacro::new(input, v).into(),
        _ => panic!("invalid type"),
    }
}

#[derive(Clone)]
pub enum Macro {
    Struct(structs::StructMacro),
}

impl From<structs::StructMacro> for Macro {
    fn from(value: structs::StructMacro) -> Self {
        Self::Struct(value)
    }
}

impl Render for Macro {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::Struct(v) => v.render(),
        }
    }
}

impl quote::ToTokens for Macro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Struct(v) => v.to_tokens(tokens),
        }
    }
}
