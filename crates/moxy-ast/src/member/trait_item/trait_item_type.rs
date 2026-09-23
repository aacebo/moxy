use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An associated type inside a trait definition (`type Name: Bound = Default;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemType {
    pub attrs: Attributes,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Option<Token![:]>,
    pub bounds: Punctuated<TypeBound, Token![+]>,
    pub default: Option<(Token![=], Type)>,
    pub semi: Token![;],
}

impl Parse for TraitItemType {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| cursor.peek::<Token![type]>() && cursor.offset(1).peek::<Ident>())
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let type_keyword = parser.parse()?;
        let ident = parser.parse()?;
        let generics = parser.parse()?;
        let (colon, bounds) = if parser.peek::<Token![:]>() {
            (Some(parser.parse()?), Punctuated::parse_separated_nonempty(parser)?)
        } else {
            (None, Punctuated::new())
        };

        let default = if parser.peek::<Token![=]>() {
            let eq = parser.parse()?;
            Some((eq, parser.parse()?))
        } else {
            None
        };

        let semi = parser.parse()?;

        Ok(Self {
            attrs,
            type_keyword,
            ident,
            generics,
            colon,
            bounds,
            default,
            semi,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = cursor.skip::<Token![type]>()?;
        cursor = cursor.skip::<Ident>()?;
        cursor = Generics::skip(cursor)?;

        if cursor.peek::<Token![:]>() {
            cursor = cursor.skip::<Token![:]>()?;
            cursor = cursor.skip::<TypeBound>()?;

            while cursor.peek::<Token![+]>() {
                cursor = cursor.skip::<Token![+]>()?;
                cursor = cursor.skip::<TypeBound>()?;
            }
        }

        if cursor.peek::<Token![=]>() {
            cursor = cursor.skip::<Token![=]>()?;
            cursor = cursor.skip::<Type>()?;
        }

        cursor.skip::<Token![;]>()
    }
}

impl Spanner for TraitItemType {
    fn span(&self) -> Span {
        self.attrs.span().join(self.semi.span())
    }
}

impl ToTokens for TraitItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon.to_tokens(t);
        self.bounds.to_tokens(t);

        if let Some((eq, d)) = &self.default {
            eq.to_tokens(t);
            d.to_tokens(t);
        }

        self.semi.to_tokens(t);
    }
}
