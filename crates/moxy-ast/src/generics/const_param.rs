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
            .map(|cursor| cursor.peek::<Token![const]>())
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let const_keyword = parser.parse()?;
        let ident = parser.parse()?;
        let colon_punct = parser.parse()?;
        let ty = parser.parse()?;
        let (default_eq_punct, default) = if parser.peek::<Token![=]>() {
            let eq_punct = parser.parse()?;
            let expr = parser.parse()?;
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
        cursor = cursor.skip::<Token![const]>()?;
        cursor = cursor.skip::<Ident>()?;
        cursor = cursor.skip::<Token![:]>()?;
        cursor = cursor.skip::<Type>()?;

        if cursor.peek::<Token![=]>() {
            cursor = cursor.skip::<Token![=]>()?;
            cursor = cursor.skip::<Expr>()?;
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
