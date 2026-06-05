use moxy_token::keyword::For;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Comma, Gt, Lt, RArrow};
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Attribute, Lifetime, Pattern, Punctuated, Type};

#[doc = "A closure parameter, either type-annotated (`pat: ty`) or inferred (`pat`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ClosureParam {
    Typed { pat: Box<Pattern>, colon: Colon, ty: Box<Type> },
    Inferred { pat: Box<Pattern> },
}

impl Parse for ClosureParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let pat = Box::new(Pattern::parse_single(stream)?);
        if stream.peek::<Colon>().is_some() {
            let colon = stream.parse::<Colon>()?;
            let ty = Box::new(stream.parse::<Type>()?);
            Ok(ClosureParam::Typed { pat, colon, ty })
        } else {
            Ok(ClosureParam::Inferred { pat })
        }
    }
}

impl ToTokens for ClosureParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            ClosureParam::Typed { pat, colon, ty } => {
                pat.to_tokens(t);
                colon.to_tokens(t);
                ty.to_tokens(t);
            }
            ClosureParam::Inferred { pat } => pat.to_tokens(t),
        }
    }
}

#[doc = "The optional return type of a function (`-> Type` or nothing)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ReturnType {
    Default,
    Type(RArrow, Box<Type>),
}

impl Parse for ReturnType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<RArrow>().is_some() {
            let arrow = stream.parse::<RArrow>()?;
            Ok(ReturnType::Type(arrow, Box::new(stream.parse::<crate::Type>()?)))
        } else {
            Ok(ReturnType::Default)
        }
    }
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let ReturnType::Type(arrow, ty) = self {
            arrow.to_tokens(t);
            ty.to_tokens(t);
        }
    }
}

#[doc = "A `for<'a, 'b>` higher-ranked lifetime binder."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BoundLifetimes {
    pub span: Span,
    pub for_keyword: For,
    pub lt: Lt,
    pub params: Punctuated<Lifetime, Comma>,
    pub gt: Gt,
}

impl Parse for BoundLifetimes {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let for_keyword = stream.parse::<For>()?;
        let lt = stream.parse::<Lt>()?;
        let mut params = Punctuated::new();

        while stream.peek::<Gt>().is_none() && !stream.is_empty() {
            params.push_value(stream.parse::<Lifetime>()?);
            if stream.peek::<Comma>().is_some() {
                params.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        let gt = stream.parse::<Gt>()?;
        Ok(Self {
            span: Span::default(),
            for_keyword,
            lt,
            params,
            gt,
        })
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
