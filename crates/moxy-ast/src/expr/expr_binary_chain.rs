use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A chain of binary operations of one precedence: `a + b - c`, `x && y`.
///
/// The operators all bind equally and associate to the left, so `a - b + c`
/// is `(a - b) + c`. An operand that binds more or less tightly is an
/// expression of its own: `a + b * c` holds the operands `a` and `b * c`.
/// Comparisons and compound assignments hold one operator; a compound
/// assignment associates to the right, so `a += b += c` holds `a` and
/// `b += c`. Holding a chain in one node keeps every traversal of a long
/// chain such as `a + b + .. + z` from recursing once per operator.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBinaryChain {
    pub attrs: Attributes,
    /// Two or more operands with an operator between each two.
    pub operands: List<Expr, BinOp>,
}

impl ExprBinaryChain {
    /// A node of one operation, `left op right`.
    pub fn new(left: Expr, op: BinOp, right: Expr) -> Self {
        let mut operands = List::new();
        operands.push_value(left);
        operands.push_punct(op);
        operands.push_value(right);
        Self {
            attrs: Attributes::default(),
            operands,
        }
    }
}

impl Attributed for ExprBinaryChain {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl From<ExprBinaryChain> for Expr {
    fn from(value: ExprBinaryChain) -> Self {
        Self::BinaryChain(value)
    }
}

impl Spanner for ExprBinaryChain {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ExprBinaryChain {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.operands.to_tokens(t);
    }
}
