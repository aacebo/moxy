use crate::{Parse, ParseError, Parser};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{Abi, FnParam, FnParams, Variadic};
use crate::{Asyncness, Constness, Delimited, Generics, Ident, Punctuated, ReturnType, Unsafety};

/// A function signature.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Signature {
    pub constness: Constness,
    pub asyncness: Asyncness,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub fn_keyword: Token![fn],
    pub ident: Ident,
    pub generics: Generics,
    pub params: Delimited<FnParams>,
    pub output: ReturnType,
}

impl Parse for Signature {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let constness = parser.parse::<Constness>()?;
        let asyncness = parser.parse::<Asyncness>()?;
        let unsafety = parser.parse::<Unsafety>()?;
        let abi = if parser.peek::<Token![extern]>() {
            Some(parser.parse::<Abi>()?)
        } else {
            None
        };

        let fn_keyword = parser.parse::<Token![fn]>()?;
        let ident = parser.parse::<Ident>()?;
        let mut generics = parser.parse::<Generics>()?;
        let params = Delimited::parse_paren_with(parser, |inner| {
            let mut inputs = Punctuated::new();
            let mut variadic = None;

            while !inner.is_empty() {
                if let Some(v) = inner.parse_if::<Variadic>() {
                    variadic = Some(v);
                    break;
                }

                inputs.push_value(inner.parse::<FnParam>()?);

                if inner.peek::<Token![,]>() {
                    inputs.push_punct(inner.parse::<Token![,]>()?);
                } else {
                    break;
                }
            }

            Ok(FnParams { inputs, variadic })
        })?;

        let output = parser.parse::<ReturnType>()?;
        generics.where_clause = parser.parse_if();

        Ok(Self {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_keyword,
            ident,
            generics,
            params,
            output,
        })
    }
}

impl Spanner for Signature {
    fn span(&self) -> Span {
        let start = if !matches!(self.constness, Constness::NoConst) {
            self.constness.span()
        } else if !matches!(self.asyncness, Asyncness::Sync) {
            self.asyncness.span()
        } else if !matches!(self.unsafety, Unsafety::Safe) {
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

impl Signature {
    pub fn emit_angle_params(generics: &Generics, t: &mut TokenStream) {
        if !generics.params.is_empty() {
            <Token![<]>::default().to_tokens(t);
            generics.params.to_tokens(t);
            <Token![>]>::default().to_tokens(t);
        }
    }

    pub fn is_start(parser: &Parser) -> bool {
        let fork = parser.lookahead();

        if fork.peek::<Token![const]>() {
            fork.advance();
        }
        if fork.peek::<Token![async]>() {
            fork.advance();
        }
        if fork.peek::<Token![unsafe]>() {
            fork.advance();
        }

        if fork.peek::<Token![extern]>() {
            fork.advance();
            if matches!(fork.curr(), Some(moxy_token::TokenTree::Literal(lit)) if lit.repr().starts_with('"')) {
                fork.advance();
            }
        }

        fork.peek::<Token![fn]>()
    }
}

impl ToTokens for Signature {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.constness.to_tokens(t);
        self.asyncness.to_tokens(t);
        self.unsafety.to_tokens(t);

        if let Some(abi) = &self.abi {
            abi.to_tokens(t);
        }

        self.fn_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        Self::emit_angle_params(&self.generics, t);
        self.params.to_tokens(t);
        self.output.to_tokens(t);

        if let Some(w) = &self.generics.where_clause {
            w.to_tokens(t);
        }
    }
}
