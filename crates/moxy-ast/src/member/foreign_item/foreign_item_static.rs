use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A foreign static declaration inside an `extern` block (`static NAME: Type;`,
/// or `safe static NAME: Type;` in an `unsafe extern` block).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemStatic {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub safety: Option<Safety>,
    pub static_keyword: Token![static],
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub colon: Token![:],
    pub ty: Type,
    pub semi: Option<Token![;]>,
}

impl Attributed for ForeignItemStatic {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ForeignItemStatic {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Safety>::skip(cursor).unwrap_or(cursor);
        <Token![static]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: <_ as Parse>::parse(parser)?,
            vis: <_ as Parse>::parse(parser)?,
            safety: <_ as Parse>::parse(parser)?,
            static_keyword: <_ as Parse>::parse(parser)?,
            mutability: <_ as Parse>::parse(parser)?,
            ident: <_ as Parse>::parse(parser)?,
            colon: <_ as Parse>::parse(parser)?,
            ty: <_ as Parse>::parse(parser)?,
            semi: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = Option::<Safety>::skip(cursor)?;
        cursor = <Token![static]>::skip(cursor)?;
        cursor = Option::<Token![mut]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;
        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for ForeignItemStatic {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ForeignItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.safety.to_tokens(t);
        self.static_keyword.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
