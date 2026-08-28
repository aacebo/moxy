use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Lifetime, Pattern, Punctuated, Type};

/// A closure parameter, either type-annotated (`pat: ty`) or inferred (`pat`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let pat = Box::new(Pattern::parse_single(stream)?);

        if stream.peek::<Token![:]>() {
            let colon = stream.parse::<Token![:]>()?;
            let ty = Box::new(stream.parse::<Type>()?);
            Ok(Self::Typed { pat, colon, ty })
        } else {
            Ok(Self::Inferred { pat })
        }
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Token![->]>() {
            let arrow = stream.parse::<Token![->]>()?;
            Ok(Self::Type(arrow, Box::new(stream.parse::<crate::Type>()?)))
        } else {
            Ok(Self::Default)
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BoundLifetimes {
    pub for_keyword: Token![for],
    pub lt: Token![<],
    pub params: Punctuated<Lifetime, Token![,]>,
    pub gt: Token![>],
}

impl Parse for BoundLifetimes {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let for_keyword = stream.parse::<Token![for]>()?;
        let lt = stream.parse::<Token![<]>()?;
        let mut params = Punctuated::new();

        while !stream.peek::<Token![>]>() && !stream.is_empty() {
            params.push_value(stream.parse::<Lifetime>()?);
            if stream.peek::<Token![,]>() {
                params.push_punct(stream.parse::<Token![,]>()?);
            } else {
                break;
            }
        }

        let gt = stream.parse::<Token![>]>()?;

        Ok(Self {
            for_keyword,
            lt,
            params,
            gt,
        })
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
