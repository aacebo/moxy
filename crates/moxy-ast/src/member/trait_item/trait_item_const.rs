use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Generics, Ident, Type};

/// A constant item inside a trait definition (`const NAME: Type;` or `const NAME: Type = expr;`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let const_keyword = parser.parse()?;
        let ident = parser.parse()?;
        let generics = parser.parse()?;
        let colon = parser.parse()?;
        let ty = parser.parse()?;
        let default = if parser.peek::<Token![=]>() {
            let eq = parser.parse()?;
            Some((eq, parser.parse()?))
        } else {
            None
        };

        let semi = parser.parse()?;

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

impl TraitItemConst {
    pub fn into_trait_item(self) -> super::TraitItem {
        super::TraitItem::from(self)
    }
}
