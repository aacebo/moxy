use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A binary operator (`+`, `==`, `&&`, ...).
#[derive(Copy, Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum BinOp {
    ShlAssign(Token![<<=]),
    ShrAssign(Token![>>=]),
    AddAssign(Token![+=]),
    SubAssign(Token![-=]),
    MulAssign(Token![*=]),
    DivAssign(Token![/=]),
    RemAssign(Token![%=]),
    BitXorAssign(Token![^=]),
    BitAndAssign(Token![&=]),
    BitOrAssign(Token![|=]),
    And(Token![&&]),
    Or(Token![||]),
    Shl(Token![<<]),
    Shr(Token![>>]),
    Eq(Token![==]),
    Ne(Token![!=]),
    Le(Token![<=]),
    Ge(Token![>=]),
    Add(Token![+]),
    Sub(Token![-]),
    Mul(Token![*]),
    Div(Token![/]),
    Rem(Token![%]),
    BitXor(Token![^]),
    BitAnd(Token![&]),
    BitOr(Token![|]),
    Lt(Token![<]),
    Gt(Token![>]),
}

impl Spanner for BinOp {
    fn span(&self) -> Span {
        match self {
            Self::ShlAssign(v) => v.span(),
            Self::ShrAssign(v) => v.span(),
            Self::AddAssign(v) => v.span(),
            Self::SubAssign(v) => v.span(),
            Self::MulAssign(v) => v.span(),
            Self::DivAssign(v) => v.span(),
            Self::RemAssign(v) => v.span(),
            Self::BitXorAssign(v) => v.span(),
            Self::BitAndAssign(v) => v.span(),
            Self::BitOrAssign(v) => v.span(),
            Self::And(v) => v.span(),
            Self::Or(v) => v.span(),
            Self::Shl(v) => v.span(),
            Self::Shr(v) => v.span(),
            Self::Eq(v) => v.span(),
            Self::Ne(v) => v.span(),
            Self::Le(v) => v.span(),
            Self::Ge(v) => v.span(),
            Self::Add(v) => v.span(),
            Self::Sub(v) => v.span(),
            Self::Mul(v) => v.span(),
            Self::Div(v) => v.span(),
            Self::Rem(v) => v.span(),
            Self::BitXor(v) => v.span(),
            Self::BitAnd(v) => v.span(),
            Self::BitOr(v) => v.span(),
            Self::Lt(v) => v.span(),
            Self::Gt(v) => v.span(),
        }
    }
}

impl ToTokens for BinOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::ShlAssign(v) => v.to_tokens(tokens),
            Self::ShrAssign(v) => v.to_tokens(tokens),
            Self::AddAssign(v) => v.to_tokens(tokens),
            Self::SubAssign(v) => v.to_tokens(tokens),
            Self::MulAssign(v) => v.to_tokens(tokens),
            Self::DivAssign(v) => v.to_tokens(tokens),
            Self::RemAssign(v) => v.to_tokens(tokens),
            Self::BitXorAssign(v) => v.to_tokens(tokens),
            Self::BitAndAssign(v) => v.to_tokens(tokens),
            Self::BitOrAssign(v) => v.to_tokens(tokens),
            Self::And(v) => v.to_tokens(tokens),
            Self::Or(v) => v.to_tokens(tokens),
            Self::Shl(v) => v.to_tokens(tokens),
            Self::Shr(v) => v.to_tokens(tokens),
            Self::Eq(v) => v.to_tokens(tokens),
            Self::Ne(v) => v.to_tokens(tokens),
            Self::Le(v) => v.to_tokens(tokens),
            Self::Ge(v) => v.to_tokens(tokens),
            Self::Add(v) => v.to_tokens(tokens),
            Self::Sub(v) => v.to_tokens(tokens),
            Self::Mul(v) => v.to_tokens(tokens),
            Self::Div(v) => v.to_tokens(tokens),
            Self::Rem(v) => v.to_tokens(tokens),
            Self::BitXor(v) => v.to_tokens(tokens),
            Self::BitAnd(v) => v.to_tokens(tokens),
            Self::BitOr(v) => v.to_tokens(tokens),
            Self::Lt(v) => v.to_tokens(tokens),
            Self::Gt(v) => v.to_tokens(tokens),
        }
    }
}

impl Parse for BinOp {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![<<=]>()
            || cursor.peek::<Token![>>=]>()
            || cursor.peek::<Token![+=]>()
            || cursor.peek::<Token![-=]>()
            || cursor.peek::<Token![*=]>()
            || cursor.peek::<Token![/=]>()
            || cursor.peek::<Token![%=]>()
            || cursor.peek::<Token![^=]>()
            || cursor.peek::<Token![&=]>()
            || cursor.peek::<Token![|=]>()
            || cursor.peek::<Token![&&]>()
            || cursor.peek::<Token![||]>()
            || cursor.peek::<Token![<<]>()
            || cursor.peek::<Token![>>]>()
            || cursor.peek::<Token![==]>()
            || cursor.peek::<Token![!=]>()
            || cursor.peek::<Token![<=]>()
            || cursor.peek::<Token![>=]>()
            || cursor.peek::<Token![+]>()
            || cursor.peek::<Token![-]>()
            || cursor.peek::<Token![*]>()
            || cursor.peek::<Token![/]>()
            || cursor.peek::<Token![%]>()
            || cursor.peek::<Token![^]>()
            || cursor.peek::<Token![&]>()
            || cursor.peek::<Token![|]>()
            || cursor.peek::<Token![<]>()
            || cursor.peek::<Token![>]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Token![<<=]>() {
            return Ok(Self::ShlAssign(parser.parse()?));
        }

        if parser.peek::<Token![>>=]>() {
            return Ok(Self::ShrAssign(parser.parse()?));
        }

        if parser.peek::<Token![+=]>() {
            return Ok(Self::AddAssign(parser.parse()?));
        }

        if parser.peek::<Token![-=]>() {
            return Ok(Self::SubAssign(parser.parse()?));
        }

        if parser.peek::<Token![*=]>() {
            return Ok(Self::MulAssign(parser.parse()?));
        }

        if parser.peek::<Token![/=]>() {
            return Ok(Self::DivAssign(parser.parse()?));
        }

        if parser.peek::<Token![%=]>() {
            return Ok(Self::RemAssign(parser.parse()?));
        }

        if parser.peek::<Token![^=]>() {
            return Ok(Self::BitXorAssign(parser.parse()?));
        }

        if parser.peek::<Token![&=]>() {
            return Ok(Self::BitAndAssign(parser.parse()?));
        }

        if parser.peek::<Token![|=]>() {
            return Ok(Self::BitOrAssign(parser.parse()?));
        }

        if parser.peek::<Token![&&]>() {
            return Ok(Self::And(parser.parse()?));
        }

        if parser.peek::<Token![||]>() {
            return Ok(Self::Or(parser.parse()?));
        }

        if parser.peek::<Token![<<]>() {
            return Ok(Self::Shl(parser.parse()?));
        }

        if parser.peek::<Token![>>]>() {
            return Ok(Self::Shr(parser.parse()?));
        }

        if parser.peek::<Token![==]>() {
            return Ok(Self::Eq(parser.parse()?));
        }

        if parser.peek::<Token![!=]>() {
            return Ok(Self::Ne(parser.parse()?));
        }

        if parser.peek::<Token![<=]>() {
            return Ok(Self::Le(parser.parse()?));
        }

        if parser.peek::<Token![>=]>() {
            return Ok(Self::Ge(parser.parse()?));
        }

        if parser.peek::<Token![+]>() {
            return Ok(Self::Add(parser.parse()?));
        }

        if parser.peek::<Token![-]>() {
            return Ok(Self::Sub(parser.parse()?));
        }

        if parser.peek::<Token![*]>() {
            return Ok(Self::Mul(parser.parse()?));
        }

        if parser.peek::<Token![/]>() {
            return Ok(Self::Div(parser.parse()?));
        }

        if parser.peek::<Token![%]>() {
            return Ok(Self::Rem(parser.parse()?));
        }

        if parser.peek::<Token![^]>() {
            return Ok(Self::BitXor(parser.parse()?));
        }

        if parser.peek::<Token![&]>() {
            return Ok(Self::BitAnd(parser.parse()?));
        }

        if parser.peek::<Token![|]>() {
            return Ok(Self::BitOr(parser.parse()?));
        }

        if parser.peek::<Token![<]>() {
            return Ok(Self::Lt(parser.parse()?));
        }

        if parser.peek::<Token![>]>() {
            return Ok(Self::Gt(parser.parse()?));
        }

        Err(parser.error("expected binary operation"))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Token![<<=]>() {
            return cursor.skip::<Token![<<=]>();
        }

        if cursor.peek::<Token![>>=]>() {
            return cursor.skip::<Token![>>=]>();
        }

        if cursor.peek::<Token![+=]>() {
            return cursor.skip::<Token![+=]>();
        }

        if cursor.peek::<Token![-=]>() {
            return cursor.skip::<Token![-=]>();
        }

        if cursor.peek::<Token![*=]>() {
            return cursor.skip::<Token![*=]>();
        }

        if cursor.peek::<Token![/=]>() {
            return cursor.skip::<Token![/=]>();
        }

        if cursor.peek::<Token![%=]>() {
            return cursor.skip::<Token![%=]>();
        }

        if cursor.peek::<Token![^=]>() {
            return cursor.skip::<Token![^=]>();
        }

        if cursor.peek::<Token![&=]>() {
            return cursor.skip::<Token![&=]>();
        }

        if cursor.peek::<Token![|=]>() {
            return cursor.skip::<Token![|=]>();
        }

        if cursor.peek::<Token![&&]>() {
            return cursor.skip::<Token![&&]>();
        }

        if cursor.peek::<Token![||]>() {
            return cursor.skip::<Token![||]>();
        }

        if cursor.peek::<Token![<<]>() {
            return cursor.skip::<Token![<<]>();
        }

        if cursor.peek::<Token![>>]>() {
            return cursor.skip::<Token![>>]>();
        }

        if cursor.peek::<Token![==]>() {
            return cursor.skip::<Token![==]>();
        }

        if cursor.peek::<Token![!=]>() {
            return cursor.skip::<Token![!=]>();
        }

        if cursor.peek::<Token![<=]>() {
            return cursor.skip::<Token![<=]>();
        }

        if cursor.peek::<Token![>=]>() {
            return cursor.skip::<Token![>=]>();
        }

        if cursor.peek::<Token![+]>() {
            return cursor.skip::<Token![+]>();
        }

        if cursor.peek::<Token![-]>() {
            return cursor.skip::<Token![-]>();
        }

        if cursor.peek::<Token![*]>() {
            return cursor.skip::<Token![*]>();
        }

        if cursor.peek::<Token![/]>() {
            return cursor.skip::<Token![/]>();
        }

        if cursor.peek::<Token![%]>() {
            return cursor.skip::<Token![%]>();
        }

        if cursor.peek::<Token![^]>() {
            return cursor.skip::<Token![^]>();
        }

        if cursor.peek::<Token![&]>() {
            return cursor.skip::<Token![&]>();
        }

        if cursor.peek::<Token![|]>() {
            return cursor.skip::<Token![|]>();
        }

        if cursor.peek::<Token![<]>() {
            return cursor.skip::<Token![<]>();
        }

        if cursor.peek::<Token![>]>() {
            return cursor.skip::<Token![>]>();
        }

        None
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShlAssign(v) => write!(f, "{v}"),
            Self::ShrAssign(v) => write!(f, "{v}"),
            Self::AddAssign(v) => write!(f, "{v}"),
            Self::SubAssign(v) => write!(f, "{v}"),
            Self::MulAssign(v) => write!(f, "{v}"),
            Self::DivAssign(v) => write!(f, "{v}"),
            Self::RemAssign(v) => write!(f, "{v}"),
            Self::BitXorAssign(v) => write!(f, "{v}"),
            Self::BitAndAssign(v) => write!(f, "{v}"),
            Self::BitOrAssign(v) => write!(f, "{v}"),
            Self::And(v) => write!(f, "{v}"),
            Self::Or(v) => write!(f, "{v}"),
            Self::Shl(v) => write!(f, "{v}"),
            Self::Shr(v) => write!(f, "{v}"),
            Self::Eq(v) => write!(f, "{v}"),
            Self::Ne(v) => write!(f, "{v}"),
            Self::Le(v) => write!(f, "{v}"),
            Self::Ge(v) => write!(f, "{v}"),
            Self::Add(v) => write!(f, "{v}"),
            Self::Sub(v) => write!(f, "{v}"),
            Self::Mul(v) => write!(f, "{v}"),
            Self::Div(v) => write!(f, "{v}"),
            Self::Rem(v) => write!(f, "{v}"),
            Self::BitXor(v) => write!(f, "{v}"),
            Self::BitAnd(v) => write!(f, "{v}"),
            Self::BitOr(v) => write!(f, "{v}"),
            Self::Lt(v) => write!(f, "{v}"),
            Self::Gt(v) => write!(f, "{v}"),
        }
    }
}

/// A unary operator (`*`, `!`, `-`).
#[derive(Copy, Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UnOp {
    Deref(Token![*]),
    Not(Token![!]),
    Neg(Token![-]),
}

impl Spanner for UnOp {
    fn span(&self) -> Span {
        match self {
            Self::Deref(v) => v.span(),
            Self::Not(v) => v.span(),
            Self::Neg(v) => v.span(),
        }
    }
}

impl ToTokens for UnOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Deref(v) => v.to_tokens(tokens),
            Self::Not(v) => v.to_tokens(tokens),
            Self::Neg(v) => v.to_tokens(tokens),
        }
    }
}

impl Parse for UnOp {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![*]>() || cursor.peek::<Token![!]>() || cursor.peek::<Token![-]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Token![*]>() {
            return Ok(Self::Deref(parser.parse()?));
        }

        if parser.peek::<Token![!]>() {
            return Ok(Self::Not(parser.parse()?));
        }

        if parser.peek::<Token![-]>() {
            return Ok(Self::Neg(parser.parse()?));
        }

        Err(parser.error("expected `UnOp`"))
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.peek::<Token![*]>() {
            return cursor.skip::<Token![*]>();
        }

        if cursor.peek::<Token![!]>() {
            return cursor.skip::<Token![!]>();
        }

        if cursor.peek::<Token![-]>() {
            return cursor.skip::<Token![-]>();
        }

        None
    }
}

impl std::fmt::Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deref(v) => write!(f, "{v}"),
            Self::Not(v) => write!(f, "{v}"),
            Self::Neg(v) => write!(f, "{v}"),
        }
    }
}
