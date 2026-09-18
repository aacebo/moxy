/// Conversions for the compiler's `proc_macro` token types.
pub mod proc_macro;

/// Conversions for `proc_macro2` token types; requires the `proc-macro2` feature.
#[cfg(feature = "proc-macro2")]
pub mod proc_macro2;
