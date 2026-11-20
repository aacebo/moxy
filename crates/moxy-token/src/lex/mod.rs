mod cursor;
pub(crate) mod depth;
mod error;

pub use cursor::*;
pub use depth::{MAX_DEPTH, MAX_NESTED_NODES};
pub use error::*;

/// Parses one token value from the beginning of a lexer cursor.
pub trait Scan: Sized {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError>;
}
