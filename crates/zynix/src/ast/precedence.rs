use crate::ast::BinOp;

#[doc = "Operator precedence level used when parsing and printing expressions without parentheses."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Precedence {
    Min = 0,
    Range,   // .. ..=
    Or,      // ||
    And,     // &&
    Compare, // == != < > <= >=
    BitOr,   // |
    BitXor,  // ^
    BitAnd,  // &
    Shift,   // << >>
    Add,     // + -
    Mul,     // * / %
    Cast,    // as
}

impl Precedence {
    pub fn next(self) -> Self {
        match self {
            Self::Min => Self::Range,
            Self::Range => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Compare,
            Self::Compare => Self::BitOr,
            Self::BitOr => Self::BitXor,
            Self::BitXor => Self::BitAnd,
            Self::BitAnd => Self::Shift,
            Self::Shift => Self::Add,
            Self::Add => Self::Mul,
            Self::Mul | Self::Cast => Self::Cast,
        }
    }

    pub fn of(op: &BinOp) -> Self {
        match op {
            BinOp::Add | BinOp::Sub => Self::Add,
            BinOp::Mul | BinOp::Div | BinOp::Rem => Self::Mul,
            BinOp::And => Self::And,
            BinOp::Or => Self::Or,
            BinOp::BitXor => Self::BitXor,
            BinOp::BitAnd => Self::BitAnd,
            BinOp::BitOr => Self::BitOr,
            BinOp::Shl | BinOp::Shr => Self::Shift,
            BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => Self::Compare,
        }
    }
}
