use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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
        cursor.peek::<Member>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let member = parser.parse()?;
        let (colon, pat) = if parser.peek::<Token![:]>() {
            (Some(parser.parse()?), parser.parse()?)
        } else {
            let Member::Named(ident) = &member else {
                return parser.error("tuple index needs a pattern").into();
            };

            (
                None,
                Pattern::Ident(PatIdent {
                    attrs: Attributes::default(),
                    by_ref: None,
                    mutability: Mutability::Immutable,
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
        let shorthand = cursor.peek::<Ident>();
        cursor = cursor.skip::<Member>()?;

        if cursor.peek::<Token![:]>() {
            cursor = cursor.skip::<Token![:]>()?;
            cursor = cursor.skip::<Pattern>()?;
        } else if !shorthand {
            return None;
        }

        Some(cursor)
    }
}

impl ToTokens for PatField {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.member.to_tokens(t);
        self.colon.to_tokens(t);
        self.pat.to_tokens(t);
    }
}
