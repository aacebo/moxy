mod cursor;
mod error;

pub use cursor::*;
pub use error::*;

pub trait Scan: Sized {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError>;
}
