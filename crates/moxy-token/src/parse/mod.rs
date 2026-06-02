mod error;
mod stream;

pub use error::*;
pub use stream::*;

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError>;
}

pub trait Peek: Sized {
    fn peek(stream: &mut ParseStream) -> Option<Self>;
}

impl<T: Parse> Peek for T {
    fn peek(stream: &mut ParseStream) -> Option<Self> {
        Self::parse(stream).ok()
    }
}
