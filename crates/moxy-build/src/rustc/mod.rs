/// Active-rustc configuration discovery and Cargo directive emission.
pub mod config;
/// Typed Cargo build-script instructions.
pub mod instruction;
/// rustc semantic-version and channel detection.
pub mod version;

#[doc(inline)]
pub use config::Config;
#[doc(inline)]
pub use instruction::Instruction;
#[doc(inline)]
pub use version::Version;
