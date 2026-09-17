use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A pattern that binds a name, optionally with `ref`/`mut` and a subpattern (`@ pat`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatIdent {
    pub attrs: Attributes,
    pub by_ref: Option<Token![ref]>,
    pub mutability: Mutability,
    pub ident: Ident,
    pub subpat: Option<(Token![@], Box<Pattern>)>,
}

impl Spanner for PatIdent {
    fn span(&self) -> Span {
        let end = if let Some((_, sub)) = &self.subpat {
            sub.span()
        } else {
            self.ident.span()
        };

        self.attrs.span().join(end)
    }
}

impl Parse for PatIdent {
    fn peek(mut cursor: Cursor<'_>) -> bool {
        cursor = Attributes::skip(cursor).unwrap_or(cursor);

        if cursor.peek::<Token![ref]>() {
            cursor = cursor.offset(1);
        }

        if cursor.peek::<Token![mut]>() {
            cursor = cursor.offset(1);
        }

        cursor.peek::<Ident>()
            && !cursor.offset(1).peek::<Token![::]>()
            && !cursor.offset(1).peek::<Token![!]>()
            && !cursor.offset(1).is_delimited(moxy_token::Delim::Paren)
            && !cursor.offset(1).is_delimited(moxy_token::Delim::Brace)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            by_ref: parser.parse()?,
            mutability: parser.parse()?,
            ident: parser.parse()?,
            subpat: if parser.peek::<Token![@]>() {
                Some((parser.parse()?, parser.parse()?))
            } else {
                None
            },
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = cursor.skip::<Option<Token![ref]>>()?;
        cursor = cursor.skip::<Mutability>()?;
        cursor = cursor.skip::<Ident>()?;

        if cursor.peek::<Token![@]>() {
            cursor = cursor.skip::<Token![@]>()?;
            cursor = cursor.skip::<Pattern>()?;
        }

        Some(cursor)
    }
}

impl ToTokens for PatIdent {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.by_ref.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);

        if let Some((at, sub)) = &self.subpat {
            at.to_tokens(t);
            sub.to_tokens(t);
        }
    }
}
