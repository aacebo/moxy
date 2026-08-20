mod operation;

use operation::*;

use crate::visit::{Visit, walk_meta};
use crate::{Attributes, Meta};

type Project<'p, T> = Box<dyn for<'x> Fn(&'x Meta) -> Option<&'x T> + 'p>;

impl Attributes {
    pub fn query(&self) -> QueryBuilder<'_, Meta> {
        QueryBuilder {
            target: self.into(),
            project: Box::new(|m| Some(m)),
            operation: Operation::Always,
        }
    }
}

impl Meta {
    pub fn query(&self) -> QueryBuilder<'_, Self> {
        QueryBuilder {
            target: self.into(),
            project: Box::new(|m| Some(m)),
            operation: Operation::Always,
        }
    }
}

#[derive(Clone, Copy)]
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
    fn walk<T>(self, c: &mut Collector<'a, '_, T>) {
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

pub struct QueryBuilder<'a, T = Meta> {
    target: QueryTarget<'a>,
    project: Project<'a, T>,
    operation: Operation<T>,
}

impl<'a, T: 'a> QueryBuilder<'a, T> {
    pub fn filter<P: Fn(&T) -> bool + 'static>(mut self, operation: P) -> Self {
        self.operation = self.operation.and(Operation::Predicate(Box::new(operation)));
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(mut self) -> Self {
        self.operation = Operation::Not(Box::new(self.operation));
        self
    }

    pub fn and<F: FnOnce(Self) -> Self>(mut self, f: F) -> Self {
        let group = f(QueryBuilder {
            target: self.target,
            project: Box::new(|_| None),
            operation: Operation::Always,
        });

        self.operation = self.operation.and(group.operation);
        self
    }

    pub fn or<F: FnOnce(Self) -> Self>(mut self, f: F) -> Self {
        let group = f(QueryBuilder {
            target: self.target,
            project: Box::new(|_| None),
            operation: Operation::Always,
        });

        self.operation = self.operation.or(group.operation);
        self
    }

    pub fn filter_map<U, F>(self, f: F) -> QueryBuilder<'a, U>
    where
        U: 'a,
        F: for<'x> Fn(&'x Meta) -> Option<&'x U> + 'a,
    {
        let project: Project<'a, U> = Box::new(move |m: &Meta| (self.project)(m).filter(|v| self.operation.eval(v)).and(f(m)));

        QueryBuilder {
            target: self.target,
            project,
            operation: Operation::Always,
        }
    }
}

impl<'a> QueryBuilder<'a, Meta> {
    pub fn path(self, name: &str) -> Self {
        let want = crate::Path::try_from_str(name).ok();
        self.filter(move |m: &Meta| {
            want.as_ref().is_some_and(|w| {
                w.leading_colon().is_some() == m.path.leading_colon().is_some()
                    && w.len() == m.path.len()
                    && w.iter().zip(m.path.iter()).all(|(x, y)| x.ident.text() == y.ident.text())
            })
        })
    }
}

impl<'a, T> QueryBuilder<'a, T> {
    pub fn collect(&self) -> Vec<&'a T> {
        let mut c = Collector {
            project: &self.project,
            operation: &self.operation,
            out: Vec::new(),
        };

        self.target.walk(&mut c);

        c.out
    }
}

struct Collector<'a, 'q, T> {
    project: &'q Project<'a, T>,
    operation: &'q Operation<T>,
    out: Vec<&'a T>,
}

impl<'ast, T> Visit<'ast> for Collector<'ast, '_, T> {
    fn visit_meta(&mut self, node: &'ast Meta) {
        if let Some(v) = (self.project)(node)
            && self.operation.eval(v)
        {
            self.out.push(v);
        }

        walk_meta(self, node);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::TokenStream;
    use moxy_token::parser::Parse;

    use super::*;
    use crate::Attribute;

    fn attrs(src: &str) -> Attributes {
        let ts = TokenStream::from_str(src).unwrap();
        let mut stream = ts.parse();
        let mut out = Vec::new();

        while !stream.is_empty() {
            out.push(Attribute::parse(&mut stream).unwrap());
        }

        out.into()
    }

    #[test]
    fn empty_query_matches_every_meta() {
        let a = attrs("#[cfg(feature = \"x\")]");
        assert!(!a.query().collect().is_empty());
    }

    #[test]
    fn path_matches_single_ident() {
        let a = attrs("#[cfg(feature = \"x\")]");
        assert!(!a.query().path("cfg").collect().is_empty());
        assert!(a.query().path("derive").collect().is_empty());
    }

    #[test]
    fn path_matches_multi_segment() {
        let a = attrs("#[serde::skip]");
        assert!(!a.query().path("serde::skip").collect().is_empty());
        // a prefix is not the whole path
        assert!(a.query().path("serde").collect().is_empty());
        assert!(a.query().path("skip").collect().is_empty());
    }

    #[test]
    fn path_leading_colon_is_significant() {
        let a = attrs("#[::core::mem]");
        assert!(!a.query().path("::core::mem").collect().is_empty());
        assert!(a.query().path("core::mem").collect().is_empty());
    }

    #[test]
    fn path_invalid_input_matches_nothing() {
        let a = attrs("#[cfg(feature = \"x\")]");
        assert!(a.query().path("a b").collect().is_empty());
        assert!(a.query().path("").collect().is_empty());
    }

    #[test]
    fn queries_across_multiple_attributes() {
        let a = attrs("#[cfg(feature = \"x\")] #[derive(Clone)]");
        assert!(!a.query().path("cfg").collect().is_empty());
        assert!(!a.query().path("derive").collect().is_empty());
    }

    #[test]
    fn and_sub_query_groups() {
        let a = attrs("#[cfg(feature = \"x\")]");
        let lists = a.query().filter(|m: &Meta| m.is_list()).and(|q| q.path("cfg")).collect();
        assert!(!lists.is_empty());
    }

    #[test]
    fn or_matches_either_branch() {
        let a = attrs("#[cfg(feature = \"x\")]");
        let found = a.query().path("derive").or(|q| q.path("cfg")).collect();
        assert!(!found.is_empty());
    }

    #[test]
    fn not_inverts() {
        let a = attrs("#[cfg(feature = \"x\")]");
        assert!(a.query().path("cfg").not().path("cfg").collect().is_empty());
    }

    #[test]
    fn meta_query_searches_within_a_single_meta() {
        let a = attrs("#[cfg(all(feature = \"x\", unix))]");
        let cfg = a.query().path("cfg").collect();
        let cfg = *cfg.first().unwrap();

        assert!(!cfg.query().path("all").collect().is_empty());
        assert!(!cfg.query().path("feature").collect().is_empty());
        assert!(cfg.query().path("derive").collect().is_empty());
    }

    #[test]
    fn meta_query_includes_itself() {
        let a = attrs("#[cfg(feature = \"x\")]");
        let cfg = *a.query().path("cfg").collect().first().unwrap();
        assert!(!cfg.query().path("cfg").collect().is_empty());
    }
}
