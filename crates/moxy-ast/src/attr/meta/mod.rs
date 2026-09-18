mod meta_argument;
mod meta_layout;
mod meta_value;

pub use meta_argument::*;
pub use meta_layout::*;
pub use meta_value::*;

use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::{Cursor, Delimited, Lit, Parse, ParseError, Parser, Path, Punctuated};

/// A structured attribute meta item (`name`, `name(...)`, `name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Meta {
    pub path: Path,
    pub content: MetaLayout,
}

impl std::ops::Deref for Meta {
    type Target = MetaLayout;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl std::ops::DerefMut for Meta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl Parse for Meta {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Path>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            path: parser.parse()?,
            content: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Path>()?;
        MetaLayout::skip(cursor)
    }
}

impl Spanner for Meta {
    fn span(&self) -> Span {
        match self.content.span() {
            None => self.path.span(),
            Some(span) => self.path.span().join(span),
        }
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.content.to_tokens(t);
    }
}
