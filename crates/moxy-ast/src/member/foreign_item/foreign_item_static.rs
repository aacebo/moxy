use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A foreign static declaration inside an `extern` block (`static NAME: Type;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemStatic {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub static_keyword: Token![static],
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub colon: Token![:],
    pub ty: Type,
    pub semi: Option<Token![;]>,
}

impl Parse for ForeignItemStatic {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        cursor.peek::<Token![static]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            vis: parser.parse()?,
            static_keyword: parser.parse()?,
            mutability: parser.parse()?,
            ident: parser.parse()?,
            colon: parser.parse()?,
            ty: parser.parse()?,
            semi: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = cursor.skip::<Token![static]>()?;
        cursor = cursor.skip::<Option<Token![mut]>>()?;
        cursor = cursor.skip::<Ident>()?;
        cursor = cursor.skip::<Token![:]>()?;
        cursor = cursor.skip::<Type>()?;
        cursor.skip::<Option<Token![;]>>()
    }
}

impl Spanner for ForeignItemStatic {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.ty.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for ForeignItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.static_keyword.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
