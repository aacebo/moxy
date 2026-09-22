use moxy_token::{Group, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A structured attribute meta item (`name`, `name(...)`, `name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Meta {
    pub path: Path,
    pub layout: MetaLayout,
}

impl Spanner for Meta {
    fn span(&self) -> Span {
        self.path.span().join(self.layout.span())
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.layout.to_tokens(tokens);
    }
}

impl Parse for Meta {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = cursor.skip::<Path>() else {
            return false;
        };

        cursor.peek::<MetaLayout>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            path: parser.parse()?,
            layout: parser.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<Path>()?;
        cursor.skip::<MetaLayout>()
    }
}

/// The shape of a meta item after its path (`name`, `name = v`, `name(..)`, `name { .. }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum MetaLayout {
    /// `#[debug]`
    Unit,
    /// `#[debug(true, env = "test")]`
    List(Group),
    /// `#[debug = true]`
    Expr { eq: Token![=], expr: Expr },
}

impl Spanner for MetaLayout {
    fn span(&self) -> Span {
        match self {
            Self::Unit => Default::default(),
            Self::List(v) => v.span(),
            Self::Expr { eq, expr } => eq.span().join(expr.span()),
        }
    }
}

impl ToTokens for MetaLayout {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Unit => {}
            Self::List(v) => v.to_tokens(tokens),
            Self::Expr { eq, expr } => {
                eq.to_tokens(tokens);
                expr.to_tokens(tokens);
            }
        }
    }
}

impl Parse for MetaLayout {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.is_empty() || cursor.peek::<Token![=]>() || cursor.peek::<Group>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.is_empty() {
            Ok(Self::Unit)
        } else if parser.peek::<Token![=]>() {
            Ok(Self::Expr {
                eq: parser.parse()?,
                expr: parser.parse()?,
            })
        } else {
            Ok(Self::List(parser.parse()?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.is_empty() {
            Some(cursor)
        } else if cursor.peek::<Token![=]>() {
            cursor.skip::<Token![=]>()?;
            cursor.skip::<Expr>()
        } else if cursor.peek::<Group>() {
            cursor.skip::<Group>()
        } else {
            None
        }
    }
}
