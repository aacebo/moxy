#[doc = "Whether an attribute is outer (`#[...]`) or inner (`#![...]`)."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum AttrStyle {
    Outer,
    Inner,
}
