use crate::{Attribute, Expr, Path};

pub struct Query<'a> {
    #[allow(unused)]
    attributes: &'a [Attribute],

    #[allow(unused)]
    predicates: Vec<Predicate>,
}

pub enum Predicate {
    Equal(Path, Expr),
}
