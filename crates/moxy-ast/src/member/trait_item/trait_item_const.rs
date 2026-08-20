use moxy_token::keyword::Const;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq, Semi};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Generics, Ident, Type};

/// A constant item inside a trait definition (`const NAME: Type;` or `const NAME: Type = expr;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemConst {
    pub attrs: Attributes,
    pub const_keyword: Const,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Colon,
    pub ty: Type,
    pub default: Option<(Eq, Expr)>,
    pub semi: Option<Semi>,
}

impl Parse for TraitItemConst {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse::<Attributes>()?;

        if stream.curr().and_then(|t| t.text()) != Some("const") {
            return Err(LexError::new(at).message("expected trait const").into());
        }

        let const_keyword = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let generics = stream.parse::<Generics>()?;
        let colon = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;

        let default = if stream.peek::<Eq>() {
            let eq = stream.parse::<Eq>()?;
            Some((eq, stream.parse::<Expr>()?))
        } else {
            None
        };

        let semi = stream.parse_if::<Semi>();
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
        let end = self
            .semi
            .as_ref()
            .map(|s| s.span())
            .or_else(|| self.default.as_ref().map(|(_, e)| e.span()))
            .unwrap_or_else(|| self.ty.span());
        self.attrs.span().join(end)
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

        if let Some((eq, d)) = &self.default {
            eq.to_tokens(t);
            d.to_tokens(t);
        }

        self.semi.to_tokens(t);
    }
}

impl TraitItemConst {
    pub fn into_trait_item(self) -> super::TraitItem {
        super::TraitItem::from(self)
    }
}
