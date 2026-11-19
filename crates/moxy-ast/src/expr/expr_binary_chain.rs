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
    /// Two or more operands with an operator between each two; parsing and
    /// [`ExprBinaryChain::new`] build no shorter chain.
    #[cfg_attr(feature = "serde", serde(serialize_with = "crate::list::serialize_with_separators"))]
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

    /// The operands and operator of a chain of one operation.
    ///
    /// ```
    /// use moxy_ast::{BinOp, Expr};
    ///
    /// let expr: Expr = moxy_ast::parse!("a - b").unwrap();
    /// let (left, op, right) = expr.as_binary_chain().unwrap().as_pair().unwrap();
    /// assert!(left.is_path() && matches!(op, BinOp::Sub(_)) && right.is_path());
    ///
    /// let expr: Expr = moxy_ast::parse!("a - b - c").unwrap();
    /// assert!(expr.as_binary_chain().unwrap().as_pair().is_none());
    /// ```
    pub fn as_pair(&self) -> Option<(&Expr, &BinOp, &Expr)> {
        let mut pairs = self.operands.pairs();
        let (Some(Pair::List(left, op)), Some(Pair::End(right)), None) = (pairs.next(), pairs.next(), pairs.next()) else {
            return None;
        };

        Some((left, op, right))
    }

    /// Splits off the last operation, as `(a - b) - c` reads `a - b - c`: the
    /// chain's attributes, the operations before the last as one expression,
    /// the last operator, and its right operand. The attributes, which apply to
    /// the whole chain, come back apart so that no operand takes them; parsing
    /// gives an outer attribute to the first operand, so a parsed chain has
    /// none. `None` for a chain of fewer than two operands, which parsing never
    /// builds.
    ///
    /// ```
    /// use moxy_ast::{BinOp, Expr};
    /// use moxy_token::ToTokenStream;
    ///
    /// let expr: Expr = moxy_ast::parse!("a - b - c").unwrap();
    /// let Expr::BinaryChain(chain) = expr else { unreachable!() };
    /// let (attrs, left, op, right) = chain.split_last().unwrap();
    /// assert!(attrs.is_empty());
    /// assert_eq!(left.to_token_stream().to_string(), "a - b");
    /// assert!(matches!(op, BinOp::Sub(_)));
    /// assert_eq!(right.to_token_stream().to_string(), "c");
    /// ```
    pub fn split_last(mut self) -> Option<(Attributes, Expr, BinOp, Expr)> {
        if self.operands.len() < 2 {
            return None;
        }

        let Some(Pair::End(right)) = self.operands.pop() else {
            return None;
        };
        let op = self.operands.pop_punct()?;
        let attrs = std::mem::take(&mut self.attrs);

        let left = if self.operands.len() == 1 {
            match self.operands.pop() {
                Some(Pair::End(left)) => left,
                _ => return None,
            }
        } else {
            Expr::BinaryChain(self)
        };

        Some((attrs, left, op, right))
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
