use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::*;

#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct File {
    pub shebang: Option<Shebang>,
    pub attrs: Attributes,
    pub items: Vec<Item>,
}

impl Spanner for File {
    fn span(&self) -> Span {
        let first = self.shebang.as_ref().map(|v| v.span()).unwrap_or(self.attrs.span());
        let last = self.items.last().map(|v| v.span()).unwrap_or(self.attrs.span());
        first.join(last)
    }
}

impl ToTokens for File {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.shebang.to_tokens(tokens);
        self.attrs.to_tokens(tokens);

        for item in &self.items {
            item.to_tokens(tokens);
        }
    }
}

impl Parse for File {
    fn peek(cursor: Cursor<'_>) -> bool {
        Shebang::peek(cursor) || Item::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            shebang: <_ as Parse>::parse(parser)?,
            attrs: <_ as Parse>::parse(parser)?,
            items: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if Shebang::peek(cursor) {
            cursor = Shebang::skip(cursor)?;
        }

        while let Some(next) = Item::skip(cursor) {
            cursor = next;
        }

        Some(cursor)
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Shebang {
    pub pound: Token![#],
    pub bang: Token![!],
    pub path: FilePath,
    pub ident: Option<Ident>,
}

impl Spanner for Shebang {
    fn span(&self) -> Span {
        let last = self.ident.as_ref().map(|v| v.span()).unwrap_or(self.path.span());
        self.pound.span().join(last)
    }
}

impl ToTokens for Shebang {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound.to_tokens(tokens);
        self.bang.to_tokens(tokens);
        self.path.to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

impl Parse for Shebang {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![#]>::peek(cursor) && <Token![!]>::peek(cursor.offset(1)) && FilePath::peek(cursor.offset(2))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            pound: <_ as Parse>::parse(parser)?,
            bang: <_ as Parse>::parse(parser)?,
            path: <_ as Parse>::parse(parser)?,
            ident: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = <Token![#]>::skip(cursor)?;
        let cursor = <Token![!]>::skip(cursor)?;
        let cursor = FilePath::skip(cursor)?;
        Option::<Ident>::skip(cursor)
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FilePath(Vec<(Token![/], Ident)>);

impl Spanner for FilePath {
    fn span(&self) -> Span {
        let first = self.0.first().map(|(slash, _)| slash.span()).unwrap_or_default();
        let last = self.0.last().map(|(_, ident)| ident.span()).unwrap_or_default();
        first.join(last)
    }
}

impl ToTokens for FilePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (slash, ident) in &self.0 {
            slash.to_tokens(tokens);
            ident.to_tokens(tokens);
        }
    }
}

impl Parse for FilePath {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![/]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let mut segments = vec![];

        while <Token![/]>::peek(parser.cursor()) {
            segments.push((<_ as Parse>::parse(parser)?, <_ as Parse>::parse(parser)?));
        }

        Ok(Self(segments))
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while let Some(next) = <Token![/]>::skip(cursor) {
            cursor = Ident::skip(next)?;
        }

        Some(cursor)
    }
}

impl std::ops::Deref for FilePath {
    type Target = [(Token![/], Ident)];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
