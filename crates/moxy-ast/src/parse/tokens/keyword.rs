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
        match <_ as Parse>::parse(parser)? {
            Keyword::As(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Async(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Auto(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Await(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Become(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Box(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Break(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Const(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Continue(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Crate(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Default(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Do(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Dyn(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Else(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Enum(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Extern(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Final(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Fn(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::For(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::If(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Impl(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::In(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Let(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Loop(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Macro(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::MacroRules(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Match(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Mod(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Move(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Mut(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Override(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Priv(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Pub(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Raw(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Ref(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Return(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::SelfType(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::SelfValue(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Static(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Struct(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Super(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Trait(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Try(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Type(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Typeof(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Union(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Unsafe(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Safe(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Unsized(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Use(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Virtual(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Where(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::While(v) => Ok(v),
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
        match <_ as Parse>::parse(parser)? {
            Keyword::Yield(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", Yield::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}
