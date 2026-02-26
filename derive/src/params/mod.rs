mod structs;

pub use structs::*;

#[derive(Clone)]
pub enum Params {
    Struct(structs::StructParams),
}

impl Params {
    #[allow(unused)]
    pub fn input(&self) -> &syn::DeriveInput {
        match self {
            Self::Struct(v) => &v.input,
        }
    }

    #[allow(unused)]
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn as_struct(&self) -> Option<&structs::StructParams> {
        match self {
            Self::Struct(v) => Some(v),
        }
    }
}

impl From<structs::StructParams> for Params {
    fn from(value: structs::StructParams) -> Self {
        Self::Struct(value)
    }
}

impl syn::parse::Parse for Params {
    fn parse(tokens: syn::parse::ParseStream) -> syn::Result<Self> {
        let input = syn::DeriveInput::parse(&tokens)?;

        match input.data.clone() {
            syn::Data::Struct(data) => Ok(structs::StructParams { input, data }.into()),
            _ => panic!("invalid type"),
        }
    }
}
