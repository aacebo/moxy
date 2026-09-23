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

/// The optional return type of a function (`-> Type` or nothing).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ReturnType {
    Default,
    Type(Token![->], Box<Type>),
}

impl ReturnType {
    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(..))
    }

    pub fn as_type(&self) -> Option<&Type> {
        if let Self::Type(_, v) = self { Some(v.as_ref()) } else { None }
    }
}

impl Parse for ReturnType {
    fn peek(_cursor: Cursor<'_>) -> bool {
        true
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Token![->]>() {
            let arrow = parser.parse()?;
            Ok(Self::Type(arrow, parser.parse()?))
        } else {
            Ok(Self::Default)
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Token![->]>() {
            cursor.skip::<Token![->]>()?.skip::<Type>()
        } else {
            Some(cursor)
        }
    }
}

impl Spanner for ReturnType {
    fn span(&self) -> Span {
        match self {
            Self::Default => Span::call_site(),
            Self::Type(arrow, ty) => arrow.span().join(ty.span()),
        }
    }
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let Self::Type(arrow, ty) = self {
            arrow.to_tokens(t);
            ty.to_tokens(t);
        }
    }
}

/// A `for<'a, 'b>` higher-ranked lifetime binder.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BoundLifetimes {
    pub for_keyword: Token![for],
    pub lt: Token![<],
    pub params: Punctuated<Lifetime, Token![,]>,
    pub gt: Token![>],
}

impl Parse for BoundLifetimes {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![for]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let for_keyword = parser.parse()?;
        let lt = parser.parse()?;
        let params = Punctuated::parse_separated_nonempty(parser)?;
        let gt = parser.parse()?;

        Ok(Self {
            for_keyword,
            lt,
            params,
            gt,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = cursor.skip::<Token![for]>()?;
        cursor = cursor.skip::<Token![<]>()?;
        cursor = cursor.skip::<Lifetime>()?;

        while cursor.peek::<Token![,]>() {
            cursor = cursor.skip::<Token![,]>()?;
            cursor = cursor.skip::<Lifetime>()?;
        }

        cursor.skip::<Token![>]>()
    }
}

impl Spanner for BoundLifetimes {
    fn span(&self) -> Span {
        self.for_keyword.span().join(self.gt.span())
    }
}

impl ToTokens for BoundLifetimes {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.for_keyword.to_tokens(t);
        self.lt.to_tokens(t);
        self.params.to_tokens(t);
        self.gt.to_tokens(t);
    }
}
