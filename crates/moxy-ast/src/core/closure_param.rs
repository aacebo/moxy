use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A closure parameter, either type-annotated (`pat: ty`) or inferred (`pat`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ClosureParam {
    Typed {
        pat: Box<Pattern>,
        colon: Token![:],
        ty: Box<Type>,
    },
    Inferred {
        pat: Box<Pattern>,
    },
}

impl ClosureParam {
    pub fn is_typed(&self) -> bool {
        matches!(self, Self::Typed { .. })
    }

    pub fn is_inferred(&self) -> bool {
        matches!(self, Self::Inferred { .. })
    }
}

impl Parse for ClosureParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        pat::skip::single(cursor).is_some()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let pat = Box::new(pat::parse::single(parser)?);

        if parser.peek::<Token![:]>() {
            Ok(Self::Typed {
                pat,
                colon: parser.parse()?,
                ty: parser.parse()?,
            })
        } else {
            Ok(Self::Inferred { pat })
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = pat::skip::single(cursor)?;

        if cursor.peek::<Token![:]>() {
            cursor = cursor.skip::<Token![:]>()?;
            cursor = cursor.skip::<Type>()?;
        }

        Some(cursor)
    }
}

impl Spanner for ClosureParam {
    fn span(&self) -> Span {
        match self {
            Self::Typed { pat, ty, .. } => pat.span().join(ty.span()),
            Self::Inferred { pat } => pat.span(),
        }
    }
}

impl ToTokens for ClosureParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Typed { pat, colon, ty } => {
                pat.to_tokens(t);
                colon.to_tokens(t);
                ty.to_tokens(t);
            }
            Self::Inferred { pat } => pat.to_tokens(t),
        }
    }
}
