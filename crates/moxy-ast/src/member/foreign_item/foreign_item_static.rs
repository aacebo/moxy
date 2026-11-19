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
    /// A value, which rustc's parser accepts and the compiler rejects later.
    pub eq: Option<Token![=]>,
    pub expr: Option<Expr>,
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
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let safety = <_ as Parse>::parse(parser)?;
        let static_keyword = <_ as Parse>::parse(parser)?;
        let mutability = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let colon = <_ as Parse>::parse(parser)?;
        let ty = <_ as Parse>::parse(parser)?;
        let eq: Option<Token![=]> = <_ as Parse>::parse(parser)?;
        let expr = if eq.is_some() {
            Some(<_ as Parse>::parse(parser)?)
        } else {
            None
        };

        Ok(Self {
            attrs,
            vis,
            safety,
            static_keyword,
            mutability,
            ident,
            colon,
            ty,
            eq,
            expr,
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

        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

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
        self.eq.to_tokens(t);
        self.expr.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
