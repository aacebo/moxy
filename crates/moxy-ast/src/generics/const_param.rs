use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A const generic parameter (`const N: usize = 0`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstParam {
    pub attrs: Attributes,
    pub const_keyword: Token![const],
    pub ident: Ident,
    pub colon_punct: Token![:],
    pub ty: Type,
    pub default_eq_punct: Option<Token![=]>,
    pub default: Option<Expr>,
}

impl Parse for ConstParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| <Token![const]>::peek(cursor))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let const_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let colon_punct = <_ as Parse>::parse(parser)?;
        let ty = <_ as Parse>::parse(parser)?;
        let (default_eq_punct, default) = if <Token![=]>::peek(parser.cursor()) {
            let eq_punct = <_ as Parse>::parse(parser)?;
            let expr = expr::parse::const_generic_default(parser)?;
            (Some(eq_punct), Some(expr))
        } else {
            (None, None)
        };

        Ok(Self {
            attrs,
            const_keyword,
            ident,
            colon_punct,
            ty,
            default_eq_punct,
            default,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = <Token![const]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;

        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = expr::skip::const_generic_default(cursor)?;
        }

        Some(cursor)
    }
}

impl Spanner for ConstParam {
    fn span(&self) -> Span {
        let end = if let Some(d) = &self.default {
            d.span()
        } else {
            self.ty.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for ConstParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.ty.to_tokens(t);
        self.default_eq_punct.to_tokens(t);
        self.default.to_tokens(t);
    }
}
