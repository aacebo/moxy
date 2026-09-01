use crate::{Attributes, Meta, Visit};

#[derive(Debug, Clone, Copy)]
pub enum QueryTarget<'a> {
    Attributes(&'a Attributes),
    Meta(&'a Meta),
}

impl<'a> From<&'a Attributes> for QueryTarget<'a> {
    fn from(a: &'a Attributes) -> Self {
        Self::Attributes(a)
    }
}

impl<'a> From<&'a Meta> for QueryTarget<'a> {
    fn from(m: &'a Meta) -> Self {
        Self::Meta(m)
    }
}

impl<'a> QueryTarget<'a> {
    pub fn walk<T>(self, c: &mut super::Collector<'a, '_, T>) {
        match self {
            Self::Attributes(attrs) => {
                for attr in attrs {
                    c.visit_meta(&attr.meta);
                }
            }
            Self::Meta(meta) => c.visit_meta(meta),
        }
    }
}
