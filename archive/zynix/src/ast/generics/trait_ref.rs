use zynix_macros::{Parse, ToTokens};

use crate::Span;
use crate::ast::{BoundPolarity, Path};

#[doc = "A trait reference (`Trait`, `!Trait`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitRef {
    #[parse(skip)]
    pub span: Span,
    pub polarity: BoundPolarity,
    pub path: Path,
}
