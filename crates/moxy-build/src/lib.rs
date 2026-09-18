//! # Moxy build
//!
//! Build-script helpers for Cargo directives and rustc version detection.
//!
//! ## Quick start
//!
//! Add `moxy` as a build dependency with the `build` feature, then use
//! [`rustc::Config`] to inspect rustc and emit typed Cargo instructions:
//!
//! ```ignore
//! let mut rustc = moxy::build::rustc::Config::read()?;
//! rustc.min_version("1.85.0").rerun_if_changed("build.rs").emit();
//! ```
//!
//! The [`rustc`] module also exposes compiler channels and individual directive
//! types for callers that need finer control.

/// rustc configuration, version detection, and Cargo instruction types.
pub mod rustc;
