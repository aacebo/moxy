use moxy_token::*;

use crate::{Cursor, Parse, ParseError, Parser};

impl Parse for Keyword {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        next.is_keyword()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Keyword(v)) => Ok(*v),
            _ => Err(parser.error("expected keyword")),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for As {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::As(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::As(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", As::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Async {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Async(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Async(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Async::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Auto {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Auto(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Auto(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Auto::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Await {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Await(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Await(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Await::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Become {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Become(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Become(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Become::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Box {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Box(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Box(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Box::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Break {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Break(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Break(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Break::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Const {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Const(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Const(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Const::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Continue {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Continue(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Continue(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Continue::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Crate {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Crate(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Crate(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Crate::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Default {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Default(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Default(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Default::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Do {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Do(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Do(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Do::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Dyn {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Dyn(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Dyn(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Dyn::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Else {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Else(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Else(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Else::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Enum {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Enum(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Enum(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Enum::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Extern {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Extern(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Extern(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Extern::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Final {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Final(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Final(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Final::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Fn {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Fn(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Fn(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Fn::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for For {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::For(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::For(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", For::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for If {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::If(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::If(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", If::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Impl {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Impl(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Impl(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Impl::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for In {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::In(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::In(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", In::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Let {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Let(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Let(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Let::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Loop {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Loop(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Loop(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Loop::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Macro {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Macro(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Macro(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Macro::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for MacroRules {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::MacroRules(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::MacroRules(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", MacroRules::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Match {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Match(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Match(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Match::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Mod {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Mod(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Mod(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Mod::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Move {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Move(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Move(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Move::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Mut {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Mut(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Mut(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Mut::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Override {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Override(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Override(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Override::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Priv {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Priv(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Priv(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Priv::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Pub {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Pub(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Pub(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Pub::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Raw {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Raw(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Raw(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Raw::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Ref {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Ref(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Ref(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Ref::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Return {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Return(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Return(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Return::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for SelfType {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::SelfType(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::SelfType(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", SelfType::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for SelfValue {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::SelfValue(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::SelfValue(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", SelfValue::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Static {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Static(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Static(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Static::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Struct {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Struct(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Struct(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Struct::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Super {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Super(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Super(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Super::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Trait {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Trait(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Trait(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Trait::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Try {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Try(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Try(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Try::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Type {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Type(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Type(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Type::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Typeof {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Typeof(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Typeof(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Typeof::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Union {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Union(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Union(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Union::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Unsafe {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Unsafe(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Unsafe(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Unsafe::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Safe {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Keyword(Keyword::Safe(_))))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Safe(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Safe::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Unsized {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Unsized(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Unsized(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Unsized::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Use {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Use(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Use(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Use::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Virtual {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Virtual(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Virtual(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Virtual::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Where {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Where(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Where(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Where::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for While {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::While(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::While(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", While::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for Yield {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Yield(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.curr() {
            Some(TokenTree::Keyword(Keyword::Yield(v))) => {
                parser.advance();
                Ok(*v)
            }
            _ => Err(parser.error(format!("expected `{}` keyword", Yield::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}
