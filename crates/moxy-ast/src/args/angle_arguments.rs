use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Cursor, GenericArgument, Parse, ParseError, Parser, Punctuated, Token};

/// A `<...>` argument list.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AngleArguments {
    pub colon2: Option<Token![::]>,
    pub lt_punct: Token![<],
    pub args: Punctuated<GenericArgument, Token![,]>,
    pub gt_punct: Token![>],
}

impl Parse for AngleArguments {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![<]>::peek(cursor) || <Token![::]>::skip(cursor).is_some_and(|cursor| <Token![<]>::peek(cursor))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            colon2: parser.parse()?,
            lt_punct: parser.parse()?,
            args: Punctuated::parse_separated_nonempty(parser)?,
            gt_punct: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![::]>::skip(cursor)?;
        cursor = <Token![<]>::skip(cursor)?;
        cursor = GenericArgument::skip(cursor)?;

        while <Token![,]>::peek(cursor) {
            cursor = <Token![,]>::skip(cursor)?;
            cursor = GenericArgument::skip(cursor)?;
        }

        <Token![>]>::skip(cursor)
    }
}

impl Spanner for AngleArguments {
    fn span(&self) -> Span {
        self.colon2
            .map(|c| c.span())
            .unwrap_or(self.lt_punct.span())
            .join(self.gt_punct.span())
    }
}

impl ToTokens for AngleArguments {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.colon2.to_tokens(t);
        self.lt_punct.to_tokens(t);
        self.args.to_tokens(t);
        self.gt_punct.to_tokens(t);
    }
}
