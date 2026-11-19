use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A method receiver parameter (`self`, `&self`, `&mut self`, `self: Box<Self>`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Receiver {
    pub attrs: Attributes,
    pub reference: Option<Token![&]>,
    pub lifetime: Option<Lifetime>,
    pub mutability: Option<Token![mut]>,
    pub self_keyword: Token![self],
    /// The explicit type of a `self` or `mut self` receiver, such as `self: Box<Self>`.
    pub ty: Option<(Token![:], Type)>,
}

impl Attributed for Receiver {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for Receiver {
    fn peek(cursor: Cursor<'_>) -> bool {
        Self::skip(cursor).is_some()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let reference: Option<Token![&]> = <_ as Parse>::parse(parser)?;
        let lifetime = if reference.is_some() {
            <_ as Parse>::parse(parser)?
        } else {
            None
        };

        let mutability = <_ as Parse>::parse(parser)?;
        let self_keyword = <_ as Parse>::parse(parser)?;
        let ty = if reference.is_none() && peek_lone_colon(parser.cursor()) {
            Some((<_ as Parse>::parse(parser)?, <_ as Parse>::parse(parser)?))
        } else {
            None
        };

        Ok(Self {
            attrs,
            reference,
            lifetime,
            mutability,
            self_keyword,
            ty,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        let reference = <Token![&]>::peek(cursor);
        cursor = Option::<Token![&]>::skip(cursor)?;

        if reference {
            cursor = Option::<Lifetime>::skip(cursor)?;
        }

        cursor = Option::<Token![mut]>::skip(cursor)?;
        cursor = <Token![self]>::skip(cursor)?;

        if !reference && peek_lone_colon(cursor) {
            cursor = <Token![:]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
        }

        Some(cursor)
    }
}

impl Spanner for Receiver {
    fn span(&self) -> Span {
        let end = match &self.ty {
            Some((_, ty)) => ty.span(),
            None => self.self_keyword.span(),
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for Receiver {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.reference.to_tokens(t);
        self.lifetime.to_tokens(t);
        self.mutability.to_tokens(t);
        self.self_keyword.to_tokens(t);

        if let Some((colon, ty)) = &self.ty {
            colon.to_tokens(t);
            ty.to_tokens(t);
        }
    }
}
