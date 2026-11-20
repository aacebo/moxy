use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use super::{Abi, FnParams};
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

            if matches!(cursor.curr(), Some(moxy_token::TokenTree::Literal(moxy_token::Lit::Str(_)))) {
                cursor = cursor.offset(1);
            }
        }

        <Token![fn]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Self::parse_with(parser, true)
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::skip_with(cursor, true)
    }
}

impl Signature {
    // Parses a signature whose parameters may be types alone, as `u8` in
    // `fn f(u8);`, only with `anonymous`: rustc accepts them only in traits
    pub(crate) fn parse_with(parser: &Parser, anonymous: bool) -> Result<Self, ParseError> {
        let constness = <_ as Parse>::parse(parser)?;
        let asyncness = <_ as Parse>::parse(parser)?;
        let safety = <_ as Parse>::parse(parser)?;
        let abi = <_ as Parse>::parse(parser)?;
        let fn_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let mut generics: Generics = <_ as Parse>::parse(parser)?;
        let params = Delimited::parse_paren_with(parser, |parser| {
            let mut inputs = List::new();

            while !parser.is_empty() {
                let param: super::FnParam = <_ as Parse>::parse(parser)?;

                // A type that ends the parameter is one without a name;
                // otherwise the tokens after it report the error
                let ends = parser.is_empty() || <Token![,]>::peek(parser.cursor());

                if !anonymous && ends && matches!(param, super::FnParam::Type(_)) {
                    return Err(ParseError::new(param.span(), "expected a parameter name"));
                }

                inputs.push_value(param);

                if <Token![,]>::peek(parser.cursor()) {
                    inputs.push_punct(<_ as Parse>::parse(parser)?);
                } else {
                    break;
                }
            }

            Ok(FnParams { inputs })
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

    pub(crate) fn skip_with(mut cursor: Cursor<'_>, anonymous: bool) -> Option<Cursor<'_>> {
        cursor = Option::<Token![const]>::skip(cursor)?;
        cursor = Option::<Token![async]>::skip(cursor)?;
        cursor = Option::<Safety>::skip(cursor)?;
        cursor = Option::<Abi>::skip(cursor)?;
        cursor = <Token![fn]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        let mut inner = cursor.descend(moxy_token::Delim::Paren)?;

        while !inner.is_empty() {
            inner = if anonymous {
                super::FnParam::skip(inner)?
            } else {
                super::FnParam::skip_named(inner)?
            };

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
        self.to_token_stream().span()
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
