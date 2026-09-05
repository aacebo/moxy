use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Abi, BareFnArg, BoundLifetimes, Delimited, Punctuated, ReturnType, Unsafety, Variadic};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BareFnParams {
    pub inputs: Punctuated<BareFnArg, Token![,]>,
    pub variadic: Option<Variadic>,
}

impl ToTokens for BareFnParams {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.inputs.to_tokens(t);

        if let Some(v) = &self.variadic {
            if !self.inputs.is_empty() && !self.inputs.is_trailing() {
                <Token![,]>::default().to_tokens(t);
            }

            v.to_tokens(t);
        }
    }
}

/// A bare function pointer type (e.g. `fn(u8) -> u8`, `extern "C" fn()`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeBareFn {
    pub lifetimes: Option<BoundLifetimes>,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub fn_keyword: Token![fn],
    pub params: Delimited<BareFnParams>,
    pub output: ReturnType,
}

impl Parse for TypeBareFn {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let lifetimes = parser.parse_if::<BoundLifetimes>();
        let unsafety = parser.parse::<Unsafety>()?;
        let abi = if parser.peek::<Token![extern]>() {
            Some(parser.parse::<Abi>()?)
        } else {
            None
        };

        let fn_keyword = parser.parse::<Token![fn]>()?;
        let params = Delimited::parse_paren_with(parser, |inner| {
            let inputs = Punctuated::parse_terminated(inner)?;
            Ok(BareFnParams { inputs, variadic: None })
        })?;

        let output = parser.parse::<ReturnType>()?;

        Ok(Self {
            lifetimes,
            unsafety,
            abi,
            fn_keyword,
            params,
            output,
        })
    }
}

impl Spanner for TypeBareFn {
    fn span(&self) -> Span {
        let start = if let Some(l) = &self.lifetimes {
            l.span()
        } else if !matches!(self.unsafety, crate::Unsafety::Safe) {
            self.unsafety.span()
        } else if let Some(abi) = &self.abi {
            abi.span()
        } else {
            self.fn_keyword.span()
        };

        let end = match &self.output {
            ReturnType::Type(_, ty) => ty.span(),
            ReturnType::Default => self.params.span(),
        };

        start.join(end)
    }
}

impl ToTokens for TypeBareFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let Some(l) = &self.lifetimes {
            l.to_tokens(t);
        }

        self.unsafety.to_tokens(t);

        if let Some(abi) = &self.abi {
            abi.to_tokens(t);
        }

        self.fn_keyword.to_tokens(t);
        self.params.to_tokens(t);
        self.output.to_tokens(t);
    }
}
