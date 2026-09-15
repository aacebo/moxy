use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A `use<'a, T>` bound (precise capturing).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UseBound {
    pub use_keyword: Token![use],
    pub lt_punct: Token![<],
    pub lifetimes: Punctuated<Lifetime, Token![,]>,
    pub gt_punct: Token![>],
}

impl Parse for UseBound {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![use]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            use_keyword: parser.parse()?,
            lt_punct: parser.parse()?,
            lifetimes: Punctuated::parse_separated_nonempty(parser)?,
            gt_punct: parser.parse()?,
        })
    }
}

impl Spanner for UseBound {
    fn span(&self) -> Span {
        self.use_keyword.span().join(self.gt_punct.span())
    }
}

impl ToTokens for UseBound {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.use_keyword.to_tokens(t);
        self.lt_punct.to_tokens(t);
        self.lifetimes.to_tokens(t);
        self.gt_punct.to_tokens(t);
    }
}
