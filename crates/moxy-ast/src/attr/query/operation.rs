#[derive(Default)]
pub enum Operation<T> {
    #[default]
    Always,
    Predicate(Box<dyn Fn(&T) -> bool>),
    And(Box<Operation<T>>, Box<Operation<T>>),
    Or(Box<Operation<T>>, Box<Operation<T>>),
    Not(Box<Operation<T>>),
}

impl<T> Operation<T> {
    pub fn and(self, other: Self) -> Self {
        match self {
            Self::Always => other,
            _ => Self::And(Box::new(self), Box::new(other)),
        }
    }

    pub fn or(self, other: Self) -> Self {
        match self {
            Self::Always => other,
            _ => Self::Or(Box::new(self), Box::new(other)),
        }
    }

    pub fn eval(&self, node: &T) -> bool {
        match self {
            Self::Always => true,
            Self::Predicate(f) => f(node),
            Self::And(a, b) => a.eval(node) && b.eval(node),
            Self::Or(a, b) => a.eval(node) || b.eval(node),
            Self::Not(p) => !p.eval(node),
        }
    }
}
