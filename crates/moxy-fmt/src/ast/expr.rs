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

use crate::{FmtError, Format, Formatter};

impl Format for Expr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Unary(v) => v.format(f),
            Self::Binary(v) => v.format(f),
            Self::Postfix(v) => v.format(f),
            Self::Block(v) => v.format(f),
            Self::Jump(v) => v.format(f),
            Self::Primary(v) => v.format(f),
            Self::Infer => f.text("_"),
            Self::Verbatim(v) => f.text(v),
        }
    }
}

// ── Primary ──────────────────────────────────────────────────────────────────

impl Format for PrimaryExpr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Lit(v) => v.format(f),
            Self::Path(v) => v.format(f),
            Self::Struct(v) => v.format(f),
            Self::Closure(v) => v.format(f),
            Self::Tuple(v) => v.format(f),
            Self::Array(v) => v.format(f),
            Self::Repeat(v) => v.format(f),
            Self::Let(v) => v.format(f),
            Self::Paren(v) => v.format(f),
            Self::Group(v) => v.format(f),
            Self::Macro(v) => v.format(f),
        }
    }
}

impl Format for ExprLit {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.lit.format(f)
    }
}

impl Format for ExprPath {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(qself) = &self.qself {
            // reuse TypePath QSelf logic via inline emit
            f.text("<")?;
            qself.ty.format(f)?;

            if qself.position > 0 {
                f.text(" as ")?;

                for (i, pair) in self.path.pairs().enumerate() {
                    if i >= qself.position {
                        break;
                    }

                    match pair {
                        moxy_ast::Pair::Punctuated(seg, _) => {
                            seg.format(f)?;
                        }
                        moxy_ast::Pair::End(seg) => {
                            seg.format(f)?;
                        }
                    }
                }
            }

            f.text(">")?;

            for (i, pair) in self.path.pairs().enumerate() {
                if i < qself.position {
                    continue;
                }

                match pair {
                    moxy_ast::Pair::Punctuated(seg, _) => {
                        seg.format(f)?;
                    }
                    moxy_ast::Pair::End(seg) => {
                        seg.format(f)?;
                    }
                }
            }
        } else {
            self.path.format(f)?;
        }

        Ok(())
    }
}

impl Format for ExprStruct {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.path.format(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.body.inner.fields.pairs() {
                f.hard_break()?;

                match pair {
                    moxy_ast::Pair::Punctuated(fv, _) => {
                        fv.format(f)?;
                        f.text(",")?;
                    }
                    moxy_ast::Pair::End(fv) => {
                        fv.format(f)?;
                        f.text(",")?;
                    }
                }
            }

            if let Some((_, rest)) = &self.body.inner.rest {
                f.hard_break()?;
                f.text("..")?;
                rest.format(f)?;
            }

            Ok(())
        })?;

        if !self.body.inner.fields.is_empty() || self.body.inner.rest.is_some() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Format for FieldValue {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.shorthand {
            self.member.format(f)
        } else {
            self.member.format(f)?;
            f.text(": ")?;
            self.expr.format(f)
        }
    }
}

impl Format for ExprClosure {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.format(f)?;
            f.text(" ")?;
        }

        self.constness.format(f)?;

        if matches!(self.constness, moxy_ast::Constness::Const(_)) {
            f.text(" ")?;
        }

        self.movability.format(f)?;

        if matches!(self.movability, moxy_ast::Movability::Static(_)) {
            f.text(" ")?;
        }

        self.asyncness.format(f)?;

        if matches!(self.asyncness, moxy_ast::Asyncness::Async(_)) {
            f.text(" ")?;
        }

        if self.capture.is_some() {
            f.text("move ")?;
        }

        f.text("|")?;
        self.inputs.format(f)?;
        f.text("|")?;
        self.output.format(f)?;
        f.text(" ")?;
        self.body.format(f)
    }
}

impl Format for ClosureParam {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Typed { pat, ty, .. } => {
                pat.format(f)?;
                f.text(": ")?;
                ty.format(f)
            }
            Self::Inferred { pat } => pat.format(f),
        }
    }
}

impl Format for ExprTuple {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.elems.inner.format(f)?;
        f.text(")")
    }
}

impl Format for ExprArray {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.elems.inner.format(f)?;
        f.text("]")
    }
}

impl Format for ExprRepeat {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.content.inner.elem.format(f)?;
        f.text("; ")?;
        self.content.inner.len.format(f)?;
        f.text("]")
    }
}

impl Format for ExprLet {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("let ")?;
        self.pat.format(f)?;
        f.text(" = ")?;
        self.expr.format(f)
    }
}

impl Format for ExprParen {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.content.inner.format(f)?;
        f.text(")")
    }
}

impl Format for ExprGroup {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.format(f)
    }
}

impl Format for ExprMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.format(f)
    }
}

// ── Unary ─────────────────────────────────────────────────────────────────────

impl Format for UnaryExpr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Reference(v) => v.format(f),
            Self::Unary(v) => v.format(f),
            Self::Cast(v) => v.format(f),
            Self::Try(v) => v.format(f),
        }
    }
}

impl Format for ExprReference {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("&")?;
        self.mutability.format(f)?;

        if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
            f.text(" ")?;
        }

        self.expr.format(f)
    }
}

impl Format for ExprUnary {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.op.format(f)?;
        self.expr.format(f)
    }
}

impl Format for ExprCast {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.format(f)?;
        f.text(" as ")?;
        self.ty.format(f)
    }
}

impl Format for ExprTry {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.format(f)?;
        f.text("?")
    }
}

// ── Binary ────────────────────────────────────────────────────────────────────

impl Format for BinaryExpr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Binary(v) => v.format(f),
            Self::Assign(v) => v.format(f),
            Self::AssignOp(v) => v.format(f),
            Self::Range(v) => v.format(f),
            Self::Type(v) => v.format(f),
        }
    }
}

impl Format for ExprBinary {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.left.format(f)?;
        f.text(" ")?;
        self.op.format(f)?;
        f.text(" ")?;
        self.right.format(f)
    }
}

impl Format for ExprAssign {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.left.format(f)?;
        f.text(" = ")?;
        self.right.format(f)
    }
}

impl Format for ExprAssignOp {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.left.format(f)?;
        f.text(" ")?;
        self.op.format(f)?;
        f.text(" ")?;
        self.right.format(f)
    }
}

impl Format for ExprRange {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(start) = &self.start {
            start.format(f)?;
        }

        self.limits.format(f)?;

        if let Some(end) = &self.end {
            end.format(f)?;
        }

        Ok(())
    }
}

impl Format for ExprType {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.format(f)?;
        f.text(": ")?;
        self.ty.format(f)
    }
}

// ── Postfix ───────────────────────────────────────────────────────────────────

impl Format for PostfixExpr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Call(v) => v.format(f),
            Self::MethodCall(v) => v.format(f),
            Self::Field(v) => v.format(f),
            Self::Index(v) => v.format(f),
            Self::Await(v) => v.format(f),
        }
    }
}

impl Format for ExprCall {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.func.format(f)?;
        f.text("(")?;
        self.args.inner.format(f)?;
        f.text(")")
    }
}

impl Format for ExprMethodCall {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.receiver.format(f)?;
        // f.soft_break()?;
        f.text(".")?;
        self.method.format(f)?;

        if let Some(turbofish) = &self.turbofish {
            turbofish.format(f)?;
        }

        f.text("(")?;
        self.args.inner.format(f)?;
        f.text(")")
    }
}

impl Format for ExprField {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.base.format(f)?;
        f.text(".")?;
        self.member.format(f)
    }
}

impl Format for ExprIndex {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.base.format(f)?;
        f.text("[")?;
        self.index.inner.format(f)?;
        f.text("]")
    }
}

impl Format for ExprAwait {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.base.format(f)?;
        f.text(".await")
    }
}

// ── Block ─────────────────────────────────────────────────────────────────────

impl Format for BlockExpr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Brace(v) => v.format(f),
            Self::If(v) => v.format(f),
            Self::While(v) => v.format(f),
            Self::ForLoop(v) => v.format(f),
            Self::Loop(v) => v.format(f),
            Self::Match(v) => v.format(f),
            Self::Async(v) => v.format(f),
            Self::Unsafe(v) => v.format(f),
            Self::Const(v) => v.format(f),
            Self::TryBlock(v) => v.format(f),
        }
    }
}

impl Format for ExprBrace {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(label) = &self.label {
            label.format(f)?;
            f.text(" ")?;
        }

        self.block.format(f)
    }
}

impl Format for ExprIf {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("if ")?;
        self.cond.format(f)?;
        f.text(" ")?;
        self.then_branch.format(f)?;

        if let Some(else_branch) = &self.else_branch {
            f.text(" else ")?;
            else_branch.format(f)?;
        }

        Ok(())
    }
}

impl Format for ExprWhile {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(label) = &self.label {
            label.format(f)?;
            f.text(" ")?;
        }

        f.text("while ")?;
        self.cond.format(f)?;
        f.text(" ")?;
        self.body.format(f)
    }
}

impl Format for ExprForLoop {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(label) = &self.label {
            label.format(f)?;
            f.text(" ")?;
        }

        f.text("for ")?;
        self.pat.format(f)?;
        f.text(" in ")?;
        self.expr.format(f)?;
        f.text(" ")?;
        self.body.format(f)
    }
}

impl Format for ExprLoop {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(label) = &self.label {
            label.format(f)?;
            f.text(" ")?;
        }

        f.text("loop ")?;
        self.body.format(f)
    }
}

impl Format for ExprMatch {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("match ")?;
        self.expr.format(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for arm in &self.arms.inner {
                f.hard_break()?;
                arm.format(f)?;
            }

            Ok(())
        })?;
        f.hard_break()?;
        f.text("}")
    }
}

impl Format for MatchArm {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.pat.format(f)?;

        if let Some(guard) = &self.guard {
            f.text(" if ")?;
            guard.format(f)?;
        }

        f.text(" => ")?;
        self.body.format(f)?;
        f.text(",")
    }
}

impl Format for ExprAsync {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("async")?;

        if self.move_keyword.is_some() {
            f.text(" move")?;
        }

        f.text(" ")?;
        self.block.format(f)
    }
}

impl Format for ExprUnsafe {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("unsafe ")?;
        self.block.format(f)
    }
}

impl Format for ExprConst {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("const ")?;
        self.block.format(f)
    }
}

impl Format for ExprTryBlock {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("try ")?;
        self.block.format(f)
    }
}

// ── Jump ──────────────────────────────────────────────────────────────────────

impl Format for JumpExpr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Return(v) => v.format(f),
            Self::Break(v) => v.format(f),
            Self::Continue(v) => v.format(f),
            Self::Yield(v) => v.format(f),
        }
    }
}

impl Format for ExprReturn {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("return")?;

        if let Some(expr) = &self.expr {
            f.text(" ")?;
            expr.format(f)?;
        }

        Ok(())
    }
}

impl Format for ExprBreak {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("break")?;

        if let Some(label) = &self.label {
            f.text(" ")?;
            label.format(f)?;
        }

        if let Some(expr) = &self.expr {
            f.text(" ")?;
            expr.format(f)?;
        }

        Ok(())
    }
}

impl Format for ExprContinue {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("continue")?;

        if let Some(label) = &self.label {
            f.text(" ")?;
            label.format(f)?;
        }

        Ok(())
    }
}

impl Format for ExprYield {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("yield")?;

        if let Some(expr) = &self.expr {
            f.text(" ")?;
            expr.format(f)?;
        }

        Ok(())
    }
}

// ── Member ────────────────────────────────────────────────────────────────────

impl Format for Member {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Named(ident) => ident.format(f),
            Self::Unnamed(index) => f.text(index.repr()),
        }
    }
}
