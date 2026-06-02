use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;

use crate::{BoundLifetimes, BoundPolarity, Path, TraitBoundModifier};

#[doc = "A trait bound (`Trait`, `?Sized`, `for<'a> Trait`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitBound {
    #[parse(skip)]
    pub span: Span,
    pub polarity: BoundPolarity,
    pub lifetimes: Option<BoundLifetimes>,
    pub modifier: TraitBoundModifier,
    pub path: Path,
}
