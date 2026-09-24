use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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
        cursor.peek::<Shebang>() || cursor.peek::<Item>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            shebang: parser.parse()?,
            attrs: parser.parse()?,
            items: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Shebang>() {
            cursor = cursor.skip::<Shebang>()?;
        }

        while let Some(next) = cursor.skip::<Item>() {
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
        cursor.peek::<Token![#]>() && cursor.offset(1).peek::<Token![!]>() && cursor.offset(2).peek::<FilePath>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            pound: parser.parse()?,
            bang: parser.parse()?,
            path: parser.parse()?,
            ident: parser.parse()?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = cursor.skip::<Token![#]>()?;
        let cursor = cursor.skip::<Token![!]>()?;
        let cursor = cursor.skip::<FilePath>()?;
        cursor.skip::<Option<Ident>>()
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
        cursor.peek::<Token![/]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let mut segments = vec![];

        while parser.peek::<Token![/]>() {
            segments.push((parser.parse()?, parser.parse()?));
        }

        Ok(Self(segments))
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while let Some(next) = cursor.skip::<Token![/]>() {
            cursor = next.skip::<Ident>()?;
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
