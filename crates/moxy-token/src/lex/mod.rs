mod cursor;
mod depth;
mod error;

pub use cursor::*;
pub use depth::*;
pub use error::*;

/// Parses one token value from the beginning of a lexer cursor.
pub trait Scan: Sized {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError>;
}
