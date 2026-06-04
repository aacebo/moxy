use crate::BinOp;

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
            BinOp::Add(_) | BinOp::Sub(_) => Self::Add,
            BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_) => Self::Mul,
            BinOp::And(_) => Self::And,
            BinOp::Or(_) => Self::Or,
            BinOp::BitXor(_) => Self::BitXor,
            BinOp::BitAnd(_) => Self::BitAnd,
            BinOp::BitOr(_) => Self::BitOr,
            BinOp::Shl(_) | BinOp::Shr(_) => Self::Shift,
            BinOp::Eq(_) | BinOp::Lt(_) | BinOp::Le(_) | BinOp::Ne(_) | BinOp::Ge(_) | BinOp::Gt(_) => Self::Compare,
        }
    }
}
