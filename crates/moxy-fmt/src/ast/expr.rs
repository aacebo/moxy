use moxy_ast::expr::binary::{ExprAssign, ExprAssignOp, ExprBinary, ExprRange, ExprType};
use moxy_ast::expr::block::{
    ExprAsync, ExprBrace, ExprConst, ExprForLoop, ExprIf, ExprLoop, ExprMatch, ExprTryBlock, ExprUnsafe, ExprWhile,
};
use moxy_ast::expr::jump::{ExprBreak, ExprContinue, ExprReturn, ExprYield};
use moxy_ast::expr::postfix::{ExprAwait, ExprCall, ExprField, ExprIndex, ExprMethodCall};
use moxy_ast::expr::primary::{
    ExprArray, ExprClosure, ExprGroup, ExprLet, ExprLit, ExprMacro, ExprParen, ExprPath, ExprRepeat, ExprStruct, ExprTuple,
};
use moxy_ast::expr::unary::{ExprCast, ExprReference, ExprTry, ExprUnary};
use moxy_ast::fields::FieldValue;
use moxy_ast::{BinaryExpr, BlockExpr, ClosureParam, Expr, JumpExpr, MatchArm, Member, PostfixExpr, PrimaryExpr, UnaryExpr};

use crate::{Fmt, FmtError, Formatter};

impl Fmt for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Unary(v) => v.fmt(f),
            Self::Binary(v) => v.fmt(f),
            Self::Postfix(v) => v.fmt(f),
            Self::Block(v) => v.fmt(f),
            Self::Jump(v) => v.fmt(f),
            Self::Primary(v) => v.fmt(f),
            Self::Infer => f.text("_"),
            Self::Verbatim(v) => f.text(v),
        }
    }
}

// ── Primary ──────────────────────────────────────────────────────────────────

impl Fmt for PrimaryExpr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Lit(v) => v.fmt(f),
            Self::Path(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Closure(v) => v.fmt(f),
            Self::Tuple(v) => v.fmt(f),
            Self::Array(v) => v.fmt(f),
            Self::Repeat(v) => v.fmt(f),
            Self::Let(v) => v.fmt(f),
            Self::Paren(v) => v.fmt(f),
            Self::Group(v) => v.fmt(f),
            Self::Macro(v) => v.fmt(f),
        }
    }
}

impl Fmt for ExprLit {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.lit.fmt(f)
    }
}

impl Fmt for ExprPath {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(qself) = &self.qself {
            // reuse TypePath QSelf logic via inline emit
            f.text("<")?;
            qself.ty.fmt(f)?;

            if qself.position > 0 {
                f.text(" as ")?;

                for (i, pair) in self.path.segments.pairs().enumerate() {
                    if i >= qself.position {
                        break;
                    }

                    match pair {
                        moxy_ast::Pair::Punctuated(seg, _) => {
                            seg.fmt(f)?;
                            f.text("::")?;
                        }
                        moxy_ast::Pair::End(seg) => {
                            seg.fmt(f)?;
                        }
                    }
                }
            }

            f.text(">")?;
            f.text("::")?;

            for (i, pair) in self.path.segments.pairs().enumerate() {
                if i < qself.position {
                    continue;
                }

                match pair {
                    moxy_ast::Pair::Punctuated(seg, _) => {
                        seg.fmt(f)?;
                        f.text("::")?;
                    }
                    moxy_ast::Pair::End(seg) => {
                        seg.fmt(f)?;
                    }
                }
            }
        } else {
            self.path.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for ExprStruct {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.path.fmt(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.body.inner.fields.pairs() {
                f.hard_break()?;

                match pair {
                    moxy_ast::Pair::Punctuated(fv, _) => {
                        fv.fmt(f)?;
                        f.text(",")?;
                    }
                    moxy_ast::Pair::End(fv) => {
                        fv.fmt(f)?;
                        f.text(",")?;
                    }
                }
            }

            if let Some((_, rest)) = &self.body.inner.rest {
                f.hard_break()?;
                f.text("..")?;
                rest.fmt(f)?;
            }

            Ok(())
        })?;

        if !self.body.inner.fields.is_empty() || self.body.inner.rest.is_some() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Fmt for FieldValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.shorthand {
            self.member.fmt(f)
        } else {
            self.member.fmt(f)?;
            f.text(": ")?;
            self.expr.fmt(f)
        }
    }
}

impl Fmt for ExprClosure {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.fmt(f)?;
            f.text(" ")?;
        }

        self.constness.fmt(f)?;

        if matches!(self.constness, moxy_ast::Constness::Const(_)) {
            f.text(" ")?;
        }

        self.movability.fmt(f)?;

        if matches!(self.movability, moxy_ast::Movability::Static(_)) {
            f.text(" ")?;
        }

        self.asyncness.fmt(f)?;

        if matches!(self.asyncness, moxy_ast::Asyncness::Async(_)) {
            f.text(" ")?;
        }

        if self.capture.is_some() {
            f.text("move ")?;
        }

        f.text("|")?;
        self.inputs.fmt(f)?;
        f.text("|")?;
        self.output.fmt(f)?;
        f.text(" ")?;
        self.body.fmt(f)
    }
}

impl Fmt for ClosureParam {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Typed { pat, ty, .. } => {
                pat.fmt(f)?;
                f.text(": ")?;
                ty.fmt(f)
            }
            Self::Inferred { pat } => pat.fmt(f),
        }
    }
}

impl Fmt for ExprTuple {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.elems.inner.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for ExprArray {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.elems.inner.fmt(f)?;
        f.text("]")
    }
}

impl Fmt for ExprRepeat {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.content.inner.elem.fmt(f)?;
        f.text("; ")?;
        self.content.inner.len.fmt(f)?;
        f.text("]")
    }
}

impl Fmt for ExprLet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("let ")?;
        self.pat.fmt(f)?;
        f.text(" = ")?;
        self.expr.fmt(f)
    }
}

impl Fmt for ExprParen {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.content.inner.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for ExprGroup {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.fmt(f)
    }
}

impl Fmt for ExprMacro {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.fmt(f)
    }
}

// ── Unary ─────────────────────────────────────────────────────────────────────

impl Fmt for UnaryExpr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Reference(v) => v.fmt(f),
            Self::Unary(v) => v.fmt(f),
            Self::Cast(v) => v.fmt(f),
            Self::Try(v) => v.fmt(f),
        }
    }
}

impl Fmt for ExprReference {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("&")?;
        self.mutability.fmt(f)?;

        if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
            f.text(" ")?;
        }

        self.expr.fmt(f)
    }
}

impl Fmt for ExprUnary {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.op.fmt(f)?;
        self.expr.fmt(f)
    }
}

impl Fmt for ExprCast {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.fmt(f)?;
        f.text(" as ")?;
        self.ty.fmt(f)
    }
}

impl Fmt for ExprTry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.fmt(f)?;
        f.text("?")
    }
}

// ── Binary ────────────────────────────────────────────────────────────────────

impl Fmt for BinaryExpr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Binary(v) => v.fmt(f),
            Self::Assign(v) => v.fmt(f),
            Self::AssignOp(v) => v.fmt(f),
            Self::Range(v) => v.fmt(f),
            Self::Type(v) => v.fmt(f),
        }
    }
}

impl Fmt for ExprBinary {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.left.fmt(f)?;
        f.text(" ")?;
        self.op.fmt(f)?;
        f.text(" ")?;
        self.right.fmt(f)
    }
}

impl Fmt for ExprAssign {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.left.fmt(f)?;
        f.text(" = ")?;
        self.right.fmt(f)
    }
}

impl Fmt for ExprAssignOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.left.fmt(f)?;
        f.text(" ")?;
        self.op.fmt(f)?;
        f.text(" ")?;
        self.right.fmt(f)
    }
}

impl Fmt for ExprRange {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(start) = &self.start {
            start.fmt(f)?;
        }

        self.limits.fmt(f)?;

        if let Some(end) = &self.end {
            end.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for ExprType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)
    }
}

// ── Postfix ───────────────────────────────────────────────────────────────────

impl Fmt for PostfixExpr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Call(v) => v.fmt(f),
            Self::MethodCall(v) => v.fmt(f),
            Self::Field(v) => v.fmt(f),
            Self::Index(v) => v.fmt(f),
            Self::Await(v) => v.fmt(f),
        }
    }
}

impl Fmt for ExprCall {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.func.fmt(f)?;
        f.text("(")?;
        self.args.inner.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for ExprMethodCall {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.receiver.fmt(f)?;
        f.soft_break()?;
        f.text(".")?;
        self.method.fmt(f)?;

        if let Some(turbofish) = &self.turbofish {
            f.text("::")?;
            turbofish.fmt(f)?;
        }

        f.text("(")?;
        self.args.inner.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for ExprField {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.base.fmt(f)?;
        f.text(".")?;
        self.member.fmt(f)
    }
}

impl Fmt for ExprIndex {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.base.fmt(f)?;
        f.text("[")?;
        self.index.inner.fmt(f)?;
        f.text("]")
    }
}

impl Fmt for ExprAwait {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.base.fmt(f)?;
        f.text(".await")
    }
}

// ── Block ─────────────────────────────────────────────────────────────────────

impl Fmt for BlockExpr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Brace(v) => v.fmt(f),
            Self::If(v) => v.fmt(f),
            Self::While(v) => v.fmt(f),
            Self::ForLoop(v) => v.fmt(f),
            Self::Loop(v) => v.fmt(f),
            Self::Match(v) => v.fmt(f),
            Self::Async(v) => v.fmt(f),
            Self::Unsafe(v) => v.fmt(f),
            Self::Const(v) => v.fmt(f),
            Self::TryBlock(v) => v.fmt(f),
        }
    }
}

impl Fmt for ExprBrace {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(label) = &self.label {
            label.fmt(f)?;
            f.text(" ")?;
        }

        self.block.fmt(f)
    }
}

impl Fmt for ExprIf {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("if ")?;
        self.cond.fmt(f)?;
        f.text(" ")?;
        self.then_branch.fmt(f)?;

        if let Some(else_branch) = &self.else_branch {
            f.text(" else ")?;
            else_branch.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for ExprWhile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(label) = &self.label {
            label.fmt(f)?;
            f.text(" ")?;
        }

        f.text("while ")?;
        self.cond.fmt(f)?;
        f.text(" ")?;
        self.body.fmt(f)
    }
}

impl Fmt for ExprForLoop {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(label) = &self.label {
            label.fmt(f)?;
            f.text(" ")?;
        }

        f.text("for ")?;
        self.pat.fmt(f)?;
        f.text(" in ")?;
        self.expr.fmt(f)?;
        f.text(" ")?;
        self.body.fmt(f)
    }
}

impl Fmt for ExprLoop {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(label) = &self.label {
            label.fmt(f)?;
            f.text(" ")?;
        }

        f.text("loop ")?;
        self.body.fmt(f)
    }
}

impl Fmt for ExprMatch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("match ")?;
        self.expr.fmt(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for arm in &self.arms.inner {
                f.hard_break()?;
                arm.fmt(f)?;
            }

            Ok(())
        })?;
        f.hard_break()?;
        f.text("}")
    }
}

impl Fmt for MatchArm {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.pat.fmt(f)?;

        if let Some(guard) = &self.guard {
            f.text(" if ")?;
            guard.fmt(f)?;
        }

        f.text(" => ")?;
        self.body.fmt(f)?;
        f.text(",")
    }
}

impl Fmt for ExprAsync {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("async")?;

        if self.move_keyword.is_some() {
            f.text(" move")?;
        }

        f.text(" ")?;
        self.block.fmt(f)
    }
}

impl Fmt for ExprUnsafe {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("unsafe ")?;
        self.block.fmt(f)
    }
}

impl Fmt for ExprConst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("const ")?;
        self.block.fmt(f)
    }
}

impl Fmt for ExprTryBlock {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("try ")?;
        self.block.fmt(f)
    }
}

// ── Jump ──────────────────────────────────────────────────────────────────────

impl Fmt for JumpExpr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Return(v) => v.fmt(f),
            Self::Break(v) => v.fmt(f),
            Self::Continue(v) => v.fmt(f),
            Self::Yield(v) => v.fmt(f),
        }
    }
}

impl Fmt for ExprReturn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("return")?;

        if let Some(expr) = &self.expr {
            f.text(" ")?;
            expr.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for ExprBreak {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("break")?;

        if let Some(label) = &self.label {
            f.text(" ")?;
            label.fmt(f)?;
        }

        if let Some(expr) = &self.expr {
            f.text(" ")?;
            expr.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for ExprContinue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("continue")?;

        if let Some(label) = &self.label {
            f.text(" ")?;
            label.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for ExprYield {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("yield")?;

        if let Some(expr) = &self.expr {
            f.text(" ")?;
            expr.fmt(f)?;
        }

        Ok(())
    }
}

// ── Member ────────────────────────────────────────────────────────────────────

impl Fmt for Member {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Named(ident) => ident.fmt(f),
            Self::Unnamed(index) => f.text(index),
        }
    }
}
