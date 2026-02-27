use crate::params::Params;

#[derive(Clone)]
pub struct StructParams {
    pub input: syn::DeriveInput,
    pub data: syn::DataStruct,
}

impl From<Params> for StructParams {
    fn from(value: Params) -> Self {
        value.as_struct().expect("expected struct").clone()
    }
}
