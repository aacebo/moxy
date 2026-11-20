use moxy_token::*;

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Punct {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_punct()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(v)) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.expected("punctuation")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for And {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::And(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::And(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", And::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Or {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Or(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Or(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Or::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Not {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Not(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Not(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Not::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Tilde {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Tilde(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Tilde(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Tilde::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Plus {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Plus(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Plus(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Plus::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Minus {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Minus(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Minus(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Minus::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Star {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Star(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Star(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Star::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Slash {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Slash(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Slash(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Slash::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Percent {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Percent(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Percent(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Percent::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Caret {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Caret(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Caret(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Caret::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Eq {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Eq(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Eq(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Eq::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Lt {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Lt(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Lt(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Lt::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Gt {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Gt(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Gt(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Gt::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for At {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::At(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::At(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", At::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Dot {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Dot(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Dot(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Dot::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Comma {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Comma(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Comma(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Comma::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Semi {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Semi(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Semi(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Semi::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Colon {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Colon(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Colon(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Colon::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Pound {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Pound(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Pound(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Pound::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Dollar {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Dollar(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Dollar(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Dollar::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Question {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Question(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Question(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Question::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Quote {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Quote(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Punct(Punct::Quote(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Quote::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

// The lexer and the compiler give `_` as an identifier, which this token
// also accepts
impl Parse for Underscore {
    fn peek(cursor: Cursor<'_>) -> bool {
        match cursor.curr() {
            Some(TokenTree::Punct(Punct::Underscore(_))) => true,
            Some(TokenTree::Ident(ident)) => ident.text() == Underscore::TEXT,
            _ => false,
        }
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Ident(ident)) if ident.text() == Underscore::TEXT => {
                parser.advance();
                Ok(Underscore::new(ident.span()))
            }
            Some(TokenTree::Punct(Punct::Underscore(value))) => {
                parser.advance();
                Ok(*value)
            }
            _ => Err(parser.expected(format_args!("`{}`", Underscore::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}
