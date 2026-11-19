use moxy_token::{Delim, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A method declaration or default implementation inside a trait definition.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemFn {
    pub attrs: Attributes,
    /// A visibility, which rustc's parser accepts and the compiler rejects
    /// later, as in `pub fn f();`.
    pub vis: Visibility,
    pub defaultness: Option<Token![default]>,
    pub sig: Signature,
    pub body: Option<StmtBlock>,
    pub semi: Option<Token![;]>,
}

impl Attributed for TraitItemFn {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for TraitItemFn {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .and_then(|cursor| Visibility::skip(cursor))
            .map(|cursor| Option::<Token![default]>::skip(cursor).unwrap_or(cursor))
            .map(|cursor| Signature::peek(cursor))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let defaultness = <_ as Parse>::parse(parser)?;
        let sig = <_ as Parse>::parse(parser)?;
        let (body, semi) = if parser.is_delimited(Delim::Brace) {
            (Some(<_ as Parse>::parse(parser)?), None)
        } else {
            (None, Some(<_ as Parse>::parse(parser)?))
        };

        Ok(Self {
            attrs,
            vis,
            defaultness,
            sig,
            body,
            semi,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Option::<Token![default]>::skip(cursor)?;
        cursor = Signature::skip(cursor)?;

        if StmtBlock::peek(cursor) {
            StmtBlock::skip(cursor)
        } else {
            <Token![;]>::skip(cursor)
        }
    }
}

impl Spanner for TraitItemFn {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for TraitItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
