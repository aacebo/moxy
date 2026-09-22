use moxy_token::{Group, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A structured attribute meta item (`name`, `name(...)`, `name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Meta {
    pub path: Path,
    pub content: MetaContent,
}

impl Meta {
    pub fn for_each<P>(&self, mut parse: P) -> Result<(), ParseError>
    where
        P: FnMut(&Self) -> Result<(), ParseError>,
    {
        let MetaContent::List(group) = &self.content else {
            return ParseError::new(self.span(), "meta content must be a list to descend").into();
        };

        let parser = Parser::from_tokens(&group.tokens);

        while parser.peek::<Path>() {
            let meta = parser.parse()?;
            parse(&meta)?;
        }

        Ok(())
    }

    pub fn parse<T>(&self) -> Result<T, ParseError>
    where
        T: Parse,
    {
        match &self.content {
            MetaContent::Unit => ParseError::new(self.span(), "unit meta content cannot be parsed").into(),
            MetaContent::List(v) => Parser::from_tokens(&v.tokens).parse(),
            MetaContent::Expr { eq: _, expr } => Parser::from_tokens(&expr).parse(),
        }
    }
}

impl Spanner for Meta {
    fn span(&self) -> Span {
        self.path.span().join(self.content.span())
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.content.to_tokens(tokens);
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

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor.skip::<Path>()?;
        cursor.skip::<MetaContent>()
    }
}

/// The shape of a meta item after its path (`name`, `name = v`, `name(..)`, `name { .. }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum MetaContent {
    /// `#[debug]`
    Unit,
    /// `#[debug(true, env = "test")]`
    List(Group),
    /// `#[debug = true]`
    Expr { eq: Token![=], expr: TokenStream },
}

impl Spanner for MetaContent {
    fn span(&self) -> Span {
        match self {
            Self::Unit => Default::default(),
            Self::List(v) => v.span(),
            Self::Expr { eq, expr } => eq.span().join(expr.span()),
        }
    }
}

impl ToTokens for MetaContent {
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

impl Parse for MetaContent {
    fn peek(cursor: Cursor<'_>) -> bool {
        if cursor.is_empty() {
            return true;
        }

        if cursor.peek::<Token![=]>() && !cursor.peek::<Token![==]>() && !cursor.peek::<Token![=>]>() && cursor.offset(1).peek::<Expr>() {
            return true;
        }

        cursor.peek::<Group>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.is_empty() {
            Ok(Self::Unit)
        } else if parser.peek::<Token![=]>() && !parser.peek::<Token![==]>() && !parser.peek::<Token![=>]>() {
            let eq = parser.parse()?;
            let start = parser.cursor();
            let end = parser.skip::<Expr>().cursor();

            Ok(Self::Expr {
                eq,
                expr: end.range(start).into(),
            })
        } else {
            Ok(Self::List(parser.parse()?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.is_empty() {
            Some(cursor)
        } else if cursor.peek::<Token![=]>() && !cursor.peek::<Token![==]>() && !cursor.peek::<Token![=>]>() && cursor.offset(1).peek::<Expr>() {
            cursor.skip::<Token![=]>()?;
            cursor.skip::<Expr>()
        } else if cursor.peek::<Group>() {
            cursor.skip::<Group>()
        } else {
            None
        }
    }
}
