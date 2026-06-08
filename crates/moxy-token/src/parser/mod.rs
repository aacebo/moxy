mod error;
mod stream;

#[doc(inline)]
pub use error::*;
#[doc(inline)]
pub use stream::*;

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError>;
}

pub trait Peek: Sized {
    fn peek(stream: &mut ParseStream) -> bool;
}

impl<T: Parse> Peek for T {
    fn peek(stream: &mut ParseStream) -> bool {
        Self::parse(stream).is_ok()
    }
}

impl<T: Parse> Parse for Option<T> {
    #[inline]
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let mut fork = stream.fork();

        match T::parse(&mut fork) {
            Ok(v) => {
                stream.seek(&fork);
                Ok(Some(v))
            }
            Err(_) => Ok(None),
        }
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(stream.parse_while::<T>())
    }
}
