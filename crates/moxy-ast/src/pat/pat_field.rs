use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::pat::PatIdent;
use crate::*;

/// A single field binding inside a struct pattern, e.g. `x` (shorthand) or `x: pat`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatField {
    pub attrs: Attributes,
    pub member: Member,
    pub colon: Option<Token![:]>,
    pub pat: Pattern,
}

impl Attributed for PatField {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl PatField {
    pub fn is_shorthand(&self) -> bool {
        self.colon.is_none()
    }
}

impl Spanner for PatField {
    fn span(&self) -> Span {
        self.attrs.span().join(self.pat.span())
    }
}

impl Parse for PatField {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        Member::peek(cursor) || PatIdent::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let by_ref = <Option<Token![ref]> as Parse>::parse(parser)?;
        let mutability = <Option<Token![mut]> as Parse>::parse(parser)?;
        let modified = by_ref.is_some() || mutability.is_some();
        let member = if modified {
            Member::Named(<_ as Parse>::parse(parser)?)
        } else {
            <_ as Parse>::parse(parser)?
        };

        if modified && <Token![:]>::peek(parser.cursor()) {
            return parser.error("shorthand binding cannot have a colon").into();
        }

        let (colon, pat) = if !modified && <Token![:]>::peek(parser.cursor()) {
            (Some(<_ as Parse>::parse(parser)?), <_ as Parse>::parse(parser)?)
        } else {
            let Member::Named(ident) = &member else {
                return parser.error("tuple index needs a pattern").into();
            };

            (
                None,
                Pattern::Ident(PatIdent {
                    attrs: Attributes::default(),
                    by_ref,
                    mutability,
                    ident: ident.clone(),
                    subpat: None,
                }),
            )
        };

        Ok(Self {
            attrs,
            member,
            colon,
            pat,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        let modified = <Token![ref]>::peek(cursor) || <Token![mut]>::peek(cursor);
        cursor = Option::<Token![ref]>::skip(cursor)?;
        cursor = Option::<Token![mut]>::skip(cursor)?;
        let shorthand = Ident::peek(cursor);

        if modified {
            cursor = Ident::skip(cursor)?;
            return (!<Token![:]>::peek(cursor)).then_some(cursor);
        }

        cursor = Member::skip(cursor)?;

        if <Token![:]>::peek(cursor) {
            cursor = <Token![:]>::skip(cursor)?;
            cursor = Pattern::skip(cursor)?;
        } else if !shorthand {
            return None;
        }

        Some(cursor)
    }
}

impl ToTokens for PatField {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if !self.is_shorthand() {
            self.member.to_tokens(t);
            self.colon.to_tokens(t);
        }

        self.pat.to_tokens(t);
    }
}
