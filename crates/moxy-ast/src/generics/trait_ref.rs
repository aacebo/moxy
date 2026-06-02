use moxy_macros::{Parse, ToTokens};
use moxy_token::Span;

use crate::{BoundPolarity, Path};

#[doc = "A trait reference (`Trait`, `!Trait`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitRef {
    #[parse(skip)]
    pub span: Span,
    pub polarity: BoundPolarity,
    pub path: Path,
}
