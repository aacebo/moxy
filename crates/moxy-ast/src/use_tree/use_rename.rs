use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A renamed use leaf (`foo as bar`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseRename {
    pub ident: Ident,
    pub as_keyword: Token![as],
    pub rename: Ident,
}

impl Parse for UseRename {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Ident>() && cursor.offset(1).peek::<Token![as]>() && cursor.offset(2).peek::<Ident>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            ident: parser.parse()?,
            as_keyword: parser.parse()?,
            rename: parser.parse()?,
        })
    }
}

impl Spanner for UseRename {
    fn span(&self) -> Span {
        self.ident.span().join(self.rename.span())
    }
}

impl ToTokens for UseRename {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        self.as_keyword.to_tokens(t);
        self.rename.to_tokens(t);
    }
}
