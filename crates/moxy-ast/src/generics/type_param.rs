use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A type parameter (`T: Bound = Default`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeParam {
    pub attrs: Attributes,
    pub ident: Ident,
    pub colon_punct: Option<Token![:]>,
    pub bounds: List<TypeBound, Token![+]>,
    pub eq_punct: Option<Token![=]>,
    pub default: Option<Type>,
}

impl Attributed for TypeParam {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for TypeParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor).map(|cursor| Ident::peek(cursor)).unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let (colon_punct, bounds) = if <Token![:]>::peek(parser.cursor()) {
            let colon_punct = <_ as Parse>::parse(parser)?;
            // Bounds may be empty, as in `T:`
            let bounds = if TypeBound::peek(parser.cursor()) {
                TypeBound::parse_bounds(parser)?
            } else {
                List::new()
            };
            (Some(colon_punct), bounds)
        } else {
            (None, List::new())
        };

        let (eq_punct, default) = if <Token![=]>::peek(parser.cursor()) {
            let eq_punct = <_ as Parse>::parse(parser)?;
            let default = <_ as Parse>::parse(parser)?;
            (Some(eq_punct), Some(default))
        } else {
            (None, None)
        };

        Ok(Self {
            attrs,
            ident,
            colon_punct,
            bounds,
            eq_punct,
            default,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Ident::skip(cursor)?;

        if <Token![:]>::peek(cursor) {
            cursor = <Token![:]>::skip(cursor)?;
            if TypeBound::peek(cursor) {
                cursor = TypeBound::skip(cursor)?;

                while <Token![+]>::peek(cursor) {
                    cursor = <Token![+]>::skip(cursor)?;
                    cursor = TypeBound::skip(cursor)?;
                }
            }
        }

        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
        }

        Some(cursor)
    }
}

impl Spanner for TypeParam {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for TypeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.default.to_tokens(t);
    }
}

impl TypeParam {
    pub fn into_generic_param(self) -> super::GenericParam {
        super::GenericParam::from(self)
    }
}
