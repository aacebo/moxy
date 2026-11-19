use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A constant item inside an `impl` block (`const NAME: Type = expr;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemConst {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub defaultness: Option<Token![default]>,
    pub const_keyword: Token![const],
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Token![:],
    pub ty: Type,
    /// The `=` and value, or none before `;`, which rustc's parser accepts and
    /// the compiler rejects later.
    pub eq: Option<Token![=]>,
    pub expr: Option<Expr>,
    pub semi: Option<Token![;]>,
}

impl Attributed for ImplItemConst {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ImplItemConst {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![default]>::skip(cursor).unwrap_or(cursor);
        <Token![const]>::peek(cursor) && Ident::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let defaultness = <_ as Parse>::parse(parser)?;
        let const_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let generics = <_ as Parse>::parse(parser)?;
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
            defaultness,
            const_keyword,
            ident,
            generics,
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
        cursor = Option::<Token![default]>::skip(cursor)?;
        cursor = <Token![const]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;
        if <Token![=]>::peek(cursor) {
            cursor = <Token![=]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for ImplItemConst {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ImplItemConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
        self.eq.to_tokens(t);
        self.expr.to_tokens(t);
        self.semi.to_tokens(t);
    }
}
