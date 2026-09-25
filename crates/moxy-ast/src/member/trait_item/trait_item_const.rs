use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A constant item inside a trait definition (`const NAME: Type;` or `const NAME: Type = expr;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemConst {
    pub attrs: Attributes,
    pub const_keyword: Token![const],
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Token![:],
    pub ty: Type,
    pub default: Option<(Token![=], Expr)>,
    pub semi: Token![;],
}

impl Parse for TraitItemConst {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| <Token![const]>::peek(cursor) && Ident::peek(cursor.offset(1)))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let const_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let generics = <_ as Parse>::parse(parser)?;
        let colon = <_ as Parse>::parse(parser)?;
        let ty = <_ as Parse>::parse(parser)?;
        let default = if <Token![=]>::peek(parser.cursor()) {
            let eq = <_ as Parse>::parse(parser)?;
            Some((eq, <_ as Parse>::parse(parser)?))
        } else {
            None
        };

        let semi = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            const_keyword,
            ident,
            generics,
            colon,
            ty,
            default,
            semi,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = <Token![const]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;

        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

        <Token![;]>::skip(cursor)
    }
}

impl Spanner for TraitItemConst {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi.span())
    }
}

impl ToTokens for TraitItemConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);

        if let Some((eq, expr)) = &self.default {
            eq.to_tokens(t);
            expr.to_tokens(t);
        }

        self.semi.to_tokens(t);
    }
}
