use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use super::{Abi, FnParams, Variadic};
use crate::*;

/// A function signature.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Signature {
    pub constness: Option<Token![const]>,
    pub asyncness: Option<Token![async]>,
    pub safety: Option<Safety>,
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

        if <Token![const]>::peek(cursor) {
            cursor = cursor.offset(1);
        }

        if <Token![async]>::peek(cursor) {
            cursor = cursor.offset(1);
        }

        if Safety::peek(cursor) {
            cursor = cursor.offset(1);
        }

        if <Token![extern]>::peek(cursor) {
            cursor = cursor.offset(1);

            if matches!(cursor.curr(), Some(moxy_token::TokenTree::Literal(lit)) if lit.repr().starts_with('"')) {
                cursor = cursor.offset(1);
            }
        }

        <Token![fn]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let constness = <_ as Parse>::parse(parser)?;
        let asyncness = <_ as Parse>::parse(parser)?;
        let safety = <_ as Parse>::parse(parser)?;
        let abi = <_ as Parse>::parse(parser)?;
        let fn_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let mut generics: Generics = <_ as Parse>::parse(parser)?;
        let params = Delimited::parse_paren_with(parser, |parser| {
            let mut inputs = Punctuated::new();
            let mut variadic = None;

            while !parser.is_empty() {
                if Variadic::peek(parser.cursor()) {
                    variadic = Some(<_ as Parse>::parse(parser)?);
                    break;
                }

                inputs.push_value(<_ as Parse>::parse(parser)?);

                if <Token![,]>::peek(parser.cursor()) {
                    inputs.push_punct(<_ as Parse>::parse(parser)?);
                } else {
                    break;
                }
            }

            Ok(FnParams { inputs, variadic })
        })?;

        let output = <_ as Parse>::parse(parser)?;
        generics.where_clause = <_ as Parse>::parse(parser)?;

        Ok(Self {
            constness,
            asyncness,
            safety,
            abi,
            fn_keyword,
            ident,
            generics,
            params,
            output,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![const]>::skip(cursor)?;
        cursor = Option::<Token![async]>::skip(cursor)?;
        cursor = Option::<Safety>::skip(cursor)?;
        cursor = Option::<Abi>::skip(cursor)?;
        cursor = <Token![fn]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        let mut inner = cursor.descend(moxy_token::Delim::Paren)?;

        while !inner.is_empty() {
            if Variadic::peek(inner) {
                inner = Variadic::skip(inner)?;
                break;
            }

            inner = super::FnParam::skip(inner)?;

            if inner.is_empty() {
                break;
            }

            inner = <Token![,]>::skip(inner)?;
        }

        if !inner.is_empty() {
            return None;
        }

        cursor = cursor.offset(1);
        cursor = ReturnType::skip(cursor)?;
        Option::<WhereClause>::skip(cursor)
    }
}

impl Spanner for Signature {
    fn span(&self) -> Span {
        let start = if let Some(v) = &self.constness {
            v.span()
        } else if let Some(v) = &self.asyncness {
            v.span()
        } else if let Some(v) = &self.safety {
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

impl ToTokens for Signature {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.constness.to_tokens(t);
        self.asyncness.to_tokens(t);
        self.safety.to_tokens(t);
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
