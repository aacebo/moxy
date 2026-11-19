use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{Cursor, GenericArgument, List, Parse, ParseError, Parser, Token};

/// A `<...>` argument list.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AngleArguments {
    pub colon2: Option<Token![::]>,
    pub lt_punct: Token![<],
    pub args: List<GenericArgument, Token![,]>,
    pub gt_punct: Token![>],
}

impl Parse for AngleArguments {
    fn peek(cursor: Cursor<'_>) -> bool {
        // `<=` is a comparison, as in `x as u64 <= max`, never the start of arguments
        let cursor = <Token![::]>::skip(cursor).unwrap_or(cursor);
        <Token![<]>::peek(cursor) && !<Token![<=]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            colon2: <_ as Parse>::parse(parser)?,
            lt_punct: <_ as Parse>::parse(parser)?,
            // Arguments may be empty, as in `Vec::<>::new()`
            args: if <Token![>]>::peek(parser.cursor()) {
                List::new()
            } else {
                List::parse_nonempty_until::<Token![>]>(parser)?
            },
            gt_punct: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Option::<Token![::]>::skip(cursor)?;
        cursor = <Token![<]>::skip(cursor)?;

        if <Token![>]>::peek(cursor) {
            return <Token![>]>::skip(cursor);
        }

        cursor = GenericArgument::skip(cursor)?;

        while <Token![,]>::peek(cursor) {
            cursor = <Token![,]>::skip(cursor)?;

            if <Token![>]>::peek(cursor) {
                break;
            }

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
