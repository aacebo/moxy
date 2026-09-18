use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::{Abi, FnParams, Variadic};
use crate::*;

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
    fn peek(cursor: Cursor<'_>) -> bool {
        let mut cursor = cursor;

        if cursor.peek::<Token![const]>() {
            cursor = cursor.offset(1);
        }

        if cursor.peek::<Token![async]>() {
            cursor = cursor.offset(1);
        }

        if cursor.peek::<Token![unsafe]>() {
            cursor = cursor.offset(1);
        }

        if cursor.peek::<Token![extern]>() {
            cursor = cursor.offset(1);

            if matches!(cursor.curr(), Some(moxy_token::TokenTree::Literal(lit)) if lit.repr().starts_with('"')) {
                cursor = cursor.offset(1);
            }
        }

        cursor.peek::<Token![fn]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let constness = parser.parse()?;
        let asyncness = parser.parse()?;
        let unsafety = parser.parse()?;
        let abi = parser.parse()?;
        let fn_keyword = parser.parse()?;
        let ident = parser.parse()?;
        let mut generics: Generics = parser.parse()?;
        let params = Delimited::parse_paren_with(parser, |parser| {
            let mut inputs = Punctuated::new();
            let mut variadic = None;

            while !parser.is_empty() {
                if parser.peek::<Variadic>() {
                    variadic = Some(parser.parse()?);
                    break;
                }

                inputs.push_value(parser.parse()?);

                if parser.peek::<Token![,]>() {
                    inputs.push_punct(parser.parse()?);
                } else {
                    break;
                }
            }

            Ok(FnParams { inputs, variadic })
        })?;

        let output = parser.parse()?;
        generics.where_clause = parser.parse()?;

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

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Constness::skip(cursor)?;
        cursor = Asyncness::skip(cursor)?;
        cursor = Unsafety::skip(cursor)?;
        cursor = cursor.skip::<Option<Abi>>()?;
        cursor = cursor.skip::<Token![fn]>()?;
        cursor = cursor.skip::<Ident>()?;
        cursor = Generics::skip(cursor)?;
        let mut inner = cursor.descend(moxy_token::Delim::Paren)?;

        while !inner.is_empty() {
            if inner.peek::<Variadic>() {
                inner = inner.skip::<Variadic>()?;
                break;
            }

            inner = inner.skip::<super::FnParam>()?;

            if inner.is_empty() {
                break;
            }

            inner = inner.skip::<Token![,]>()?;
        }

        if !inner.is_empty() {
            return None;
        }

        cursor = cursor.offset(1);
        cursor = ReturnType::skip(cursor)?;
        cursor.skip::<Option<WhereClause>>()
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

impl ToTokens for Signature {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.constness.to_tokens(t);
        self.asyncness.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.abi.to_tokens(t);
        self.fn_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.lt.to_tokens(t);
        self.generics.params.to_tokens(t);
        self.generics.gt.to_tokens(t);
        self.params.to_tokens(t);
        self.output.to_tokens(t);
        self.generics.where_clause.to_tokens(t);
    }
}
