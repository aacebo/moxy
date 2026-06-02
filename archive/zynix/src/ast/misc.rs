use crate::ast::{Attribute, Lifetime, Pattern, Punctuated, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::For;
use crate::token::punct::{Colon, Comma, Gt, Lt, RArrow};
use crate::{Parse, Span, TokenStream};

#[doc = "A closure parameter, either type-annotated (`pat: ty`) or inferred (`pat`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ClosureParam {
    Typed { pat: Box<Pattern>, ty: Box<Type> },
    Inferred { pat: Box<Pattern> },
}

impl Parse for ClosureParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let pat = Box::new(Pattern::parse_single(stream)?);
        if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            let ty = Box::new(stream.parse::<Type>()?);
            Ok(ClosureParam::Typed { pat, ty })
        } else {
            Ok(ClosureParam::Inferred { pat })
        }
    }
}

impl ToTokens for ClosureParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            ClosureParam::Typed { pat, ty } => {
                pat.to_tokens(t);
                Colon::default().to_tokens(t);
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
    Type(Box<Type>),
}

impl Parse for ReturnType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<RArrow>().is_some() {
            let _ = stream.parse::<RArrow>()?;
            Ok(ReturnType::Type(Box::new(stream.parse::<crate::ast::Type>()?)))
        } else {
            Ok(ReturnType::Default)
        }
    }
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let ReturnType::Type(ty) = self {
            RArrow::default().to_tokens(t);
            ty.to_tokens(t);
        }
    }
}

#[doc = "A `for<'a, 'b>` higher-ranked lifetime binder."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BoundLifetimes {
    pub span: Span,
    pub params: Punctuated<Lifetime, Comma>,
}

impl Parse for BoundLifetimes {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<For>()?;
        let _ = stream.parse::<Lt>()?;
        let mut params = Punctuated::new();

        while stream.peek::<Gt>().is_none() && !stream.is_empty() {
            params.push_value(stream.parse::<Lifetime>()?);
            if stream.peek::<Comma>().is_some() {
                params.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        let _ = stream.parse::<Gt>()?;
        Ok(Self {
            span: Span::default(),
            params,
        })
    }
}

impl ToTokens for BoundLifetimes {
    fn to_tokens(&self, t: &mut TokenStream) {
        For::default().to_tokens(t);
        Lt::default().to_tokens(t);
        self.params.to_tokens(t);
        Gt::default().to_tokens(t);
    }
}
