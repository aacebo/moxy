use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A bare function pointer type (e.g. `fn(u8) -> u8`, `extern "C" fn()`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeBareFn {
    pub lifetimes: Option<BoundLifetimes>,
    pub unsafety: Option<Token![unsafe]>,
    pub abi: Option<Abi>,
    pub fn_keyword: Token![fn],
    pub params: Delimited<BareFnParams>,
    pub output: ReturnType,
}

impl Parse for TypeBareFn {
    fn peek(mut cursor: Cursor<'_>) -> bool {
        cursor = BoundLifetimes::skip(cursor).unwrap_or(cursor);

        if <Token![unsafe]>::peek(cursor) {
            cursor = cursor.offset(1);
        }

        if <Token![extern]>::peek(cursor) {
            cursor = Abi::skip(cursor).unwrap_or(cursor);
        }

        <Token![fn]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let lifetimes = parser.parse()?;
        let unsafety = parser.parse()?;
        let abi = parser.parse()?;
        let fn_keyword = parser.parse()?;
        let params = Delimited::parse_paren_with(parser, |inner| {
            let mut inputs = Punctuated::new();
            let mut variadic = None;

            while !inner.is_empty() {
                if Variadic::peek(inner.cursor()) {
                    variadic = Some(inner.parse()?);
                    break;
                }

                inputs.push_value(inner.parse()?);

                if <Token![,]>::peek(inner.cursor()) {
                    inputs.push_punct(inner.parse()?);
                } else {
                    break;
                }
            }

            Ok(BareFnParams { inputs, variadic })
        })?;

        let output = parser.parse()?;

        Ok(Self {
            lifetimes,
            unsafety,
            abi,
            fn_keyword,
            params,
            output,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = BoundLifetimes::skip(cursor).unwrap_or(cursor);
        cursor = Option::<Token![unsafe]>::skip(cursor)?;
        cursor = Option::<Abi>::skip(cursor)?;
        cursor = <Token![fn]>::skip(cursor)?;
        let mut inner = cursor.descend(moxy_token::Delim::Paren)?;

        while !inner.is_empty() {
            if Variadic::peek(inner) {
                inner = Variadic::skip(inner)?;
                break;
            }

            inner = BareFnArg::skip(inner)?;

            if <Token![,]>::peek(inner) {
                inner = <Token![,]>::skip(inner)?;
            } else {
                break;
            }
        }

        if !inner.is_empty() {
            return None;
        }

        ReturnType::skip(cursor.offset(1))
    }
}

impl Spanner for TypeBareFn {
    fn span(&self) -> Span {
        let start = if let Some(l) = &self.lifetimes {
            l.span()
        } else if let Some(v) = &self.unsafety {
            v.span()
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
        self.lifetimes.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.abi.to_tokens(t);
        self.fn_keyword.to_tokens(t);
        self.params.to_tokens(t);
        self.output.to_tokens(t);
    }
}

/// An AST representation of Rust bare fn params syntax.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BareFnParams {
    pub inputs: Punctuated<BareFnArg, Token![,]>,
    pub variadic: Option<Variadic>,
}

impl ToTokens for BareFnParams {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.inputs.to_tokens(t);
        self.variadic.to_tokens(t);
    }
}
