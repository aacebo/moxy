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

#[macro_export]
macro_rules! parse {
    ($tokens:ident as $ty:ty) => {{
        let mut stream = $tokens.parse();
        match <$ty as $crate::Parse>::parse(&mut stream) {
            Ok(v) => v,
            Err(e) => return e.to_compile_error().into_iter().collect(),
        }
    }};
    (? $tokens:ident as $ty:ty) => {{
        let mut stream = $tokens.parse();
        <$ty as $crate::Parse>::parse(&mut stream)
    }};
}
