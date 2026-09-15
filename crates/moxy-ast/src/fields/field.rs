use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct/enum field definition (`pub name: Type` or `pub Type`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Field {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Option<Ident>,
    pub colon: Option<Token![:]>,
    pub ty: Type,
}

impl Parse for Field {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![pub]>() || cursor.peek::<Token![mut]>() || cursor.peek::<Ident>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            vis: parser.parse()?,
            mutability: parser.parse()?,
            ident: parser.parse()?,
            colon: parser.parse()?,
            ty: parser.parse()?,
        })
    }
}

impl Spanner for Field {
    fn span(&self) -> Span {
        self.attrs.span().join(self.ty.span())
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon.to_tokens(t);
        self.ty.to_tokens(t);
    }
}
