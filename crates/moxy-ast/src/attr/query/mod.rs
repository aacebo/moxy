use crate::Meta;

pub fn new<T>() -> QueryBuilder<T> {
    QueryBuilder::<T>::default()
}

pub struct QueryBuilder<T = Meta>(Vec<Box<dyn Fn(&T) -> bool>>);

impl<T> QueryBuilder<T> {
    pub fn and<P: Fn(&T) -> bool + 'static>(mut self, predicate: P) -> Self {
        self.0.push(Box::new(predicate));
        self
    }
}

impl<T> Default for QueryBuilder<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

pub struct And<T = Meta>(Vec<Box<dyn Fn(&T) -> bool>>);

impl<T> From<Vec<Box<dyn Fn(&T) -> bool>>> for And<T> {
    fn from(value: Vec<Box<dyn Fn(&T) -> bool>>) -> Self {
        Self(value)
    }
}

impl<T> std::ops::Deref for And<T> {
    type Target = Vec<Box<dyn Fn(&T) -> bool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for And<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Or<T = Meta>(Vec<Box<dyn Fn(&T) -> bool>>);

impl<T> From<Vec<Box<dyn Fn(&T) -> bool>>> for Or<T> {
    fn from(value: Vec<Box<dyn Fn(&T) -> bool>>) -> Self {
        Self(value)
    }
}

impl<T> std::ops::Deref for Or<T> {
    type Target = Vec<Box<dyn Fn(&T) -> bool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Or<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
