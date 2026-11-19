mod stmt_block;
mod stmt_local;
mod stmt_macro;

pub use stmt_block::*;
pub use stmt_local::*;
pub use stmt_macro::*;

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A statement in a block.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum Stmt {
    Empty(Token![;]),
    Local(Box<StmtLocal>),
    Block(StmtBlock),
    Item(Box<Item>),
    Expr(Box<Expr>, Option<Token![;]>),
    Macro(StmtMacro),
}

impl Stmt {
    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local(_))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(_))
    }

    pub fn is_expr(&self) -> bool {
        matches!(self, Self::Expr(..))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn as_local(&self) -> Option<&StmtLocal> {
        if let Self::Local(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_block(&self) -> Option<&StmtBlock> {
        if let Self::Block(v) = self { Some(v) } else { None }
    }

    pub fn as_item(&self) -> Option<&Item> {
        if let Self::Item(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_macro(&self) -> Option<&StmtMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }
}

impl Spanner for Stmt {
    fn span(&self) -> Span {
        match self {
            Self::Empty(v) => v.span(),
            Self::Local(v) => v.span(),
            Self::Block(v) => v.span(),
            Self::Item(v) => v.span(),
            Self::Expr(v, semi) => {
                let end = semi.as_ref().map(|s| s.span()).unwrap_or_else(|| v.span());
                v.span().join(end)
            }
            Self::Macro(v) => v.span(),
        }
    }
}

impl Parse for Stmt {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![;]>::peek(cursor)
            || StmtLocal::peek(cursor)
            || StmtMacro::peek(cursor)
            || block(cursor)
            || Item::peek(cursor)
            || Expr::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        // One call site for every form keeps one return slot in this frame
        let parse: fn(&Parser) -> Result<Self, ParseError> = if <Token![;]>::peek(parser.cursor()) {
            |parser| Ok(Self::Empty(<_ as Parse>::parse(parser)?))
        } else if StmtLocal::peek(parser.cursor()) {
            |parser| Ok(Self::Local(Box::new(<_ as Parse>::parse(parser)?)))
        } else if StmtMacro::peek(parser.cursor()) {
            |parser| Ok(Self::Macro(<_ as Parse>::parse(parser)?))
        } else if block(parser.cursor()) {
            |parser| Ok(Self::Block(<_ as Parse>::parse(parser)?))
        } else if Item::peek(parser.cursor()) && !macro_expr(parser.cursor()) {
            // An item inside a block takes another level of stack, as in
            // `fn f() { fn f() { .. } }`
            |parser| {
                let _weight = moxy_token::Depth::enter_by(1).ok_or_else(|| ParseError::too_deep(parser.span()))?;
                Ok(Self::Item(Box::new(<_ as Parse>::parse(parser)?)))
            }
        } else {
            |parser| {
                let block_like = crate::expr::peek::block_like(Attributes::skip(parser.cursor()).unwrap_or(parser.cursor()));
                let expr = crate::expr::parse::early(parser)?;
                let semi: Option<Token![;]> = <_ as Parse>::parse(parser)?;

                // As in rustc, only a block-like expression that ends the
                // statement alone, or the last one in a block, needs no `;`
                if semi.is_none() && !parser.is_empty() && !(block_like && ends_block(&expr)) {
                    return parser.error("expected `;`").into();
                }

                Ok(Self::Expr(expr, semi))
            }
        };

        parse(parser)
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![;]>::peek(cursor) {
            <Token![;]>::skip(cursor)
        } else if StmtLocal::peek(cursor) {
            StmtLocal::skip(cursor)
        } else if StmtMacro::peek(cursor) {
            StmtMacro::skip(cursor)
        } else if block(cursor) {
            StmtBlock::skip(cursor)
        } else if Item::peek(cursor) && !macro_expr(cursor) {
            Item::skip(cursor)
        } else {
            let (cursor, block_like) = crate::expr::skip::early_block_like(cursor)?;

            if <Token![;]>::peek(cursor) {
                <Token![;]>::skip(cursor)
            } else {
                (block_like || cursor.is_empty()).then_some(cursor)
            }
        }
    }
}

// Whether an expression is a block-like one, which a statement that starts
// block-like and is not continued parses to
pub(crate) fn ends_block(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Block(_)
            | Expr::IfChain(_)
            | Expr::Match(_)
            | Expr::Loop(_)
            | Expr::While(_)
            | Expr::ForLoop(_)
            | Expr::Unsafe(_)
            | Expr::Const(_)
            | Expr::TryBlock(_)
    )
}

// Whether a block statement starts here. A method call, field access, or `?`
// after the block continues it as an expression: `{ x }.len();`
fn block(cursor: Cursor<'_>) -> bool {
    StmtBlock::peek(cursor) && !crate::expr::peek::continues_block_like(cursor.offset(1))
}

// Whether a macro call that an operator follows starts an expression
// statement, which `StmtMacro::peek` leaves to the expression parser
fn macro_expr(cursor: Cursor<'_>) -> bool {
    Attributes::skip(cursor).is_some_and(|start| MacroCall::peek(start)) && !StmtMacro::peek(cursor)
}

impl ToTokens for Stmt {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Empty(v) => v.to_tokens(t),
            Self::Local(v) => v.to_tokens(t),
            Self::Block(v) => v.to_tokens(t),
            Self::Item(v) => v.to_tokens(t),
            Self::Expr(v, semi) => {
                v.to_tokens(t);
                semi.to_tokens(t);
            }
            Self::Macro(v) => v.to_tokens(t),
        }
    }
}
