use moxy_token::{Keyword, TokenTree, keyword};

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

impl Parse for keyword::As {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::As(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::As(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::As::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Async {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Async(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Async(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Async::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Auto {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Auto(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Auto(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Auto::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Await {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Await(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Await(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Await::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Become {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Become(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Become(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Become::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Box {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Box(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Box(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Box::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Break {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Break(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Break(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Break::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Const {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Const(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Const(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Const::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Continue {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Continue(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Continue(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Continue::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Crate {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Crate(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Crate(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Crate::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Default {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Default(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Default(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Default::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Do {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Do(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Do(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Do::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Dyn {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Dyn(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Dyn(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Dyn::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Else {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Else(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Else(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Else::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Enum {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Enum(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Enum(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Enum::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Extern {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Extern(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Extern(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Extern::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Final {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Final(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Final(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Final::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Fn {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Fn(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Fn(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Fn::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::For {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::For(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::For(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::For::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::If {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::If(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::If(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::If::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Impl {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Impl(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Impl(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Impl::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::In {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::In(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::In(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::In::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Let {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Let(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Let(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Let::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Loop {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Loop(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Loop(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Loop::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Macro {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Macro(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Macro(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Macro::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::MacroRules {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::MacroRules(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::MacroRules(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::MacroRules::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Match {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Match(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Match(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Match::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Mod {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Mod(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Mod(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Mod::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Move {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Move(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Move(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Move::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Mut {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Mut(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Mut(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Mut::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Override {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Override(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Override(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Override::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Priv {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Priv(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Priv(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Priv::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Pub {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Pub(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Pub(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Pub::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Raw {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Raw(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Raw(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Raw::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Ref {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Ref(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Ref(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Ref::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Return {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Return(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Return(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Return::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::SelfType {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::SelfType(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::SelfType(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::SelfType::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::SelfValue {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::SelfValue(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::SelfValue(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::SelfValue::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Static {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Static(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Static(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Static::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Struct {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Struct(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Struct(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Struct::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Super {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Super(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Super(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Super::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Trait {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Trait(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Trait(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Trait::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Try {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Try(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Try(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Try::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Type {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Type(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Type(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Type::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Typeof {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Typeof(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Typeof(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Typeof::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Union {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Union(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Union(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Union::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Unsafe {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Unsafe(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Unsafe(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Unsafe::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Unsized {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Unsized(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Unsized(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Unsized::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Use {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Use(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Use(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Use::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Virtual {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Virtual(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Virtual(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Virtual::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Where {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Where(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Where(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Where::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::While {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::While(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::While(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::While::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl Parse for keyword::Yield {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(next) = cursor.curr() else {
            return false;
        };

        matches!(next, TokenTree::Keyword(Keyword::Yield(_)))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.parse()? {
            Keyword::Yield(v) => Ok(v),
            _ => Err(parser.error(format!("expected `{}` keyword", keyword::Yield::TEXT))),
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}
