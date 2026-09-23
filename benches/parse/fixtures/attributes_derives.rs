#[repr(C)]
#[derive(Clone, Debug)]
pub struct Header {
    #[cfg(unix)]
    pub bytes: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(Clone, Debug))]
pub struct Conditional {
    #[allow(dead_code)]
    pub value: usize,
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    Empty = 0,
    Data = 1,
    Error = 2,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "extra", derive(PartialEq, Eq))]
pub struct Envelope {
    #[cfg(feature = "network")]
    pub endpoint: String,
    #[cfg(feature = "filesystem")]
    pub path: String,
    #[allow(clippy::type_complexity)]
    pub payload: Option<Result<Vec<u8>, String>>,
}

#[allow(clippy::missing_const_for_fn)]
#[inline(always)]
#[must_use]
pub fn build(value: usize) -> usize {
    value + 1
}

#[cfg(any(feature = "network", feature = "filesystem"))]
pub mod platform {
    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct Config {
        #[cfg(unix)]
        pub mode: u32,
        #[cfg(windows)]
        pub flags: u32,
    }
}

#[deprecated(note = "benchmark fixture")]
pub const LEGACY_LIMIT: usize = 8;

#[allow(dead_code)]
pub static mut ATTRIBUTED_STATE: usize = 0;
