#[derive(Clone, Debug)]
pub struct Envelope<'a, T: Clone> {
    pub id: u64,
    pub payload: &'a T,
}

pub enum Message<T> {
    Empty,
    Value(T),
    Record { id: u64, value: T },
}

pub trait Service<T>: Send {
    type Error;
    fn call(&self, value: T) -> Result<T, Self::Error>;
}

impl<'a, T: Clone + Send> Envelope<'a, T> {
    pub fn map<U: Clone>(&self, value: U) -> Envelope<'_, U> {
        Envelope { id: self.id, payload: &value }
    }
}

pub fn transform<T: Clone>(items: Vec<T>) -> Option<T> {
    items.into_iter().next()
}

macro_rules! passthrough {
    ($value:expr) => { $value };
}
