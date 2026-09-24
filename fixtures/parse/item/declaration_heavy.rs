#[repr(transparent)]
#[derive(Clone, Debug)]
pub(crate) struct Registry<'a, T: Clone + Send, const N: usize>
where
    T: 'a,
{
    pub entries: &'a [T; N],
}

pub trait Inspect<'a, T>: Send
where
    T: Clone + 'a,
{
    type Output: Clone;
    const LIMIT: usize;

    fn inspect(&'a self, value: T) -> Result<Self::Output, T>;
}

impl<'a, T: Clone + Send, const N: usize> Inspect<'a, T> for Registry<'a, T, N>
where
    T: 'a,
{
    type Output = T;
    const LIMIT: usize = N;

    fn inspect(&'a self, value: T) -> Result<Self::Output, T> {
        Ok(value)
    }
}
