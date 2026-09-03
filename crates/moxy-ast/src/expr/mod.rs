use crate::{Parse, ParseError, Parser, Peek};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

pub mod binary;
pub mod block;
pub mod jump;
pub mod postfix;
pub mod primary;
pub mod unary;

pub use binary::*;
pub use block::*;
pub use jump::*;
pub use postfix::*;
pub use primary::*;
pub use unary::*;

/// A Rust expression. The primary recursive node covering all expression forms.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Postfix(PostfixExpr),
    Block(BlockExpr),
    Jump(JumpExpr),
    Primary(PrimaryExpr),
    Infer,
    Verbatim(TokenStream),
}

impl Peek for Expr {
    fn peek(parser: &Parser) -> bool {
        parser.parse::<Self>().is_ok()
    }
}

impl Expr {
    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Unary(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    pub fn is_postfix(&self) -> bool {
        matches!(self, Self::Postfix(_))
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_jump(&self) -> bool {
        matches!(self, Self::Jump(_))
    }

    pub fn is_primary(&self) -> bool {
        matches!(self, Self::Primary(_))
    }

    pub fn is_infer(&self) -> bool {
        matches!(self, Self::Infer)
    }

    pub fn is_verbatim(&self) -> bool {
        matches!(self, Self::Verbatim(_))
    }

    pub fn as_unary(&self) -> Option<&UnaryExpr> {
        if let Self::Unary(v) = self { Some(v) } else { None }
    }

    pub fn as_binary(&self) -> Option<&BinaryExpr> {
        if let Self::Binary(v) = self { Some(v) } else { None }
    }

    pub fn as_postfix(&self) -> Option<&PostfixExpr> {
        if let Self::Postfix(v) = self { Some(v) } else { None }
    }

    pub fn as_block(&self) -> Option<&BlockExpr> {
        if let Self::Block(v) = self { Some(v) } else { None }
    }

    pub fn as_jump(&self) -> Option<&JumpExpr> {
        if let Self::Jump(v) = self { Some(v) } else { None }
    }

    pub fn as_primary(&self) -> Option<&PrimaryExpr> {
        if let Self::Primary(v) = self { Some(v) } else { None }
    }

    pub fn attrs(&self) -> Option<&crate::Attributes> {
        match self {
            Self::Unary(v) => Some(v.attrs()),
            Self::Binary(v) => Some(v.attrs()),
            Self::Postfix(v) => Some(v.attrs()),
            Self::Block(v) => Some(v.attrs()),
            Self::Jump(v) => Some(v.attrs()),
            Self::Primary(v) => Some(v.attrs()),
            Self::Infer | Self::Verbatim(_) => None,
        }
    }

    pub fn attrs_mut(&mut self) -> Option<&mut crate::Attributes> {
        match self {
            Self::Unary(v) => Some(v.attrs_mut()),
            Self::Binary(v) => Some(v.attrs_mut()),
            Self::Postfix(v) => Some(v.attrs_mut()),
            Self::Block(v) => Some(v.attrs_mut()),
            Self::Jump(v) => Some(v.attrs_mut()),
            Self::Primary(v) => Some(v.attrs_mut()),
            Self::Infer | Self::Verbatim(_) => None,
        }
    }
}

impl Spanner for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Unary(v) => v.span(),
            Self::Binary(v) => v.span(),
            Self::Postfix(v) => v.span(),
            Self::Block(v) => v.span(),
            Self::Jump(v) => v.span(),
            Self::Primary(v) => v.span(),
            Self::Infer => Span::call_site(),
            Self::Verbatim(_) => Span::call_site(),
        }
    }
}

impl From<UnaryExpr> for Expr {
    fn from(v: UnaryExpr) -> Self {
        Self::Unary(v)
    }
}

impl From<BinaryExpr> for Expr {
    fn from(v: BinaryExpr) -> Self {
        Self::Binary(v)
    }
}

impl From<PostfixExpr> for Expr {
    fn from(v: PostfixExpr) -> Self {
        Self::Postfix(v)
    }
}

impl From<BlockExpr> for Expr {
    fn from(v: BlockExpr) -> Self {
        Self::Block(v)
    }
}

impl From<JumpExpr> for Expr {
    fn from(v: JumpExpr) -> Self {
        Self::Jump(v)
    }
}

impl From<PrimaryExpr> for Expr {
    fn from(v: PrimaryExpr) -> Self {
        Self::Primary(v)
    }
}

impl From<ExprReference> for Expr {
    fn from(value: ExprReference) -> Self {
        Self::Unary(UnaryExpr::from(value))
    }
}

impl From<ExprUnary> for Expr {
    fn from(value: ExprUnary) -> Self {
        Self::Unary(UnaryExpr::from(value))
    }
}

impl From<ExprCast> for Expr {
    fn from(value: ExprCast) -> Self {
        Self::Unary(UnaryExpr::from(value))
    }
}

impl From<ExprTry> for Expr {
    fn from(value: ExprTry) -> Self {
        Self::Unary(UnaryExpr::from(value))
    }
}

impl From<ExprBinary> for Expr {
    fn from(value: ExprBinary) -> Self {
        Self::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprAssign> for Expr {
    fn from(value: ExprAssign) -> Self {
        Self::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprAssignOp> for Expr {
    fn from(value: ExprAssignOp) -> Self {
        Self::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprRange> for Expr {
    fn from(value: ExprRange) -> Self {
        Self::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprType> for Expr {
    fn from(value: ExprType) -> Self {
        Self::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprCall> for Expr {
    fn from(value: ExprCall) -> Self {
        Self::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprMethodCall> for Expr {
    fn from(value: ExprMethodCall) -> Self {
        Self::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprField> for Expr {
    fn from(value: ExprField) -> Self {
        Self::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprIndex> for Expr {
    fn from(value: ExprIndex) -> Self {
        Self::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprAwait> for Expr {
    fn from(value: ExprAwait) -> Self {
        Self::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprBrace> for Expr {
    fn from(value: ExprBrace) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprIf> for Expr {
    fn from(value: ExprIf) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprWhile> for Expr {
    fn from(value: ExprWhile) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprForLoop> for Expr {
    fn from(value: ExprForLoop) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprLoop> for Expr {
    fn from(value: ExprLoop) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprMatch> for Expr {
    fn from(value: ExprMatch) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprAsync> for Expr {
    fn from(value: ExprAsync) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprUnsafe> for Expr {
    fn from(value: ExprUnsafe) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprConst> for Expr {
    fn from(value: ExprConst) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprTryBlock> for Expr {
    fn from(value: ExprTryBlock) -> Self {
        Self::Block(BlockExpr::from(value))
    }
}

impl From<ExprReturn> for Expr {
    fn from(value: ExprReturn) -> Self {
        Self::Jump(JumpExpr::from(value))
    }
}

impl From<ExprBreak> for Expr {
    fn from(value: ExprBreak) -> Self {
        Self::Jump(JumpExpr::from(value))
    }
}

impl From<ExprContinue> for Expr {
    fn from(value: ExprContinue) -> Self {
        Self::Jump(JumpExpr::from(value))
    }
}

impl From<ExprYield> for Expr {
    fn from(value: ExprYield) -> Self {
        Self::Jump(JumpExpr::from(value))
    }
}

impl From<ExprLit> for Expr {
    fn from(value: ExprLit) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprPath> for Expr {
    fn from(value: ExprPath) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprStruct> for Expr {
    fn from(value: ExprStruct) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprClosure> for Expr {
    fn from(value: ExprClosure) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprTuple> for Expr {
    fn from(value: ExprTuple) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprArray> for Expr {
    fn from(value: ExprArray) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprRepeat> for Expr {
    fn from(value: ExprRepeat) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprLet> for Expr {
    fn from(value: ExprLet) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprParen> for Expr {
    fn from(value: ExprParen) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprGroup> for Expr {
    fn from(value: ExprGroup) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprMacro> for Expr {
    fn from(value: ExprMacro) -> Self {
        Self::Primary(PrimaryExpr::from(value))
    }
}

impl Parse for Expr {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        parse_expr(parser, true)
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Unary(v) => v.to_tokens(t),
            Self::Binary(v) => v.to_tokens(t),
            Self::Postfix(v) => v.to_tokens(t),
            Self::Block(v) => v.to_tokens(t),
            Self::Jump(v) => v.to_tokens(t),
            Self::Primary(v) => v.to_tokens(t),
            Self::Infer => {}
            Self::Verbatim(v) => v.to_tokens(t),
        }
    }
}

// Parser

pub fn parse_expr(parser: &Parser, allow_struct: bool) -> Result<Expr, ParseError> {
    use crate::precedence::Precedence;
    let lhs = unary::UnaryExpr::parse_from(parser, allow_struct)?;
    binary::BinaryExpr::parse_from(parser, lhs, Precedence::Min, allow_struct)
}
