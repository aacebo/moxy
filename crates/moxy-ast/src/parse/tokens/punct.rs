use moxy_token::{Punct, TokenTree, punct};

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Punct {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_punct()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Punct(v)) => Ok(*v),
            _ => Err(parser.error("expected punctuation")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::And {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::And(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::And(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::And::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Or {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Or(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Or(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Or::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Not {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Not(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Not(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Not::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Tilde {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Tilde(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Tilde(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Tilde::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Plus {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Plus(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Plus(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Plus::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Minus {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Minus(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Minus(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Minus::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Star {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Star(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Star(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Star::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Slash {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Slash(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Slash(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Slash::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Percent {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Percent(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Percent(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Percent::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Caret {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Caret(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Caret(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Caret::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Eq {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Eq(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Eq(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Eq::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Lt {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Lt(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Lt(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Lt::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Gt {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Gt(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Gt(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Gt::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::At {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::At(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::At(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::At::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Dot {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Dot(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Dot(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Dot::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Comma {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Comma(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Comma(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Comma::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Semi {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Semi(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Semi(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Semi::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Colon {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Colon(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Colon(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Colon::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Pound {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Pound(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Pound(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Pound::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Dollar {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Dollar(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Dollar(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Dollar::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Question {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Question(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Question(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Question::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Quote {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Quote(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Quote(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Quote::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for punct::Underscore {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Punct(Punct::Underscore(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match <_ as Parse>::parse(parser)? {
            Punct::Underscore(value) => Ok(value),
            _ => Err(parser.error(format!("expected `{}` punctuation", punct::Underscore::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}
