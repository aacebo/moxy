use crate::Attribute;

pub trait Query {
    fn exec(&self, attributes: &[Attribute]) -> &[Attribute];
}
