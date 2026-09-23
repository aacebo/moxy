use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// An associated type definition inside an `impl` block (`type Name = Type;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemType {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub eq: Token![=],
    pub ty: Type,
    pub semi: Option<Token![;]>,
}

impl Parse for ImplItemType {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Defaultness::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![type]>() && cursor.offset(1).peek::<Ident>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            vis: parser.parse()?,
            defaultness: parser.parse()?,
            type_keyword: parser.parse()?,
            ident: parser.parse()?,
            generics: parser.parse()?,
            eq: parser.parse()?,
            ty: parser.parse()?,
            semi: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Defaultness::skip(cursor)?;
        cursor = cursor.skip::<Token![type]>()?;
        cursor = cursor.skip::<Ident>()?;
        cursor = Generics::skip(cursor)?;
        cursor = cursor.skip::<Token![=]>()?;
        cursor = cursor.skip::<Type>()?;
        cursor.skip::<Option<Token![;]>>()
    }
}

impl Spanner for ImplItemType {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.ty.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ImplItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.eq.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
