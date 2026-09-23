use moxy_ast::expr::*;
use moxy_ast::fields::FieldValue;
use moxy_ast::*;

use crate::{FmtError, Format, Formatter};

impl Format for Expr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Unary(v) => v.format(f),
            Self::Binary(v) => v.format(f),
            Self::Block(v) => v.format(f),
            Self::Infer(v) => v.format(f),
            Self::Return(v) => v.format(f),
            Self::Break(v) => v.format(f),
            Self::Continue(v) => v.format(f),
            Self::Yield(v) => v.format(f),
            Self::Call(v) => v.format(f),
            Self::MethodCall(v) => v.format(f),
            Self::Field(v) => v.format(f),
            Self::Index(v) => v.format(f),
            Self::Await(v) => v.format(f),
            Self::If(v) => v.format(f),
            Self::While(v) => v.format(f),
            Self::ForLoop(v) => v.format(f),
            Self::Loop(v) => v.format(f),
            Self::Match(v) => v.format(f),
            Self::Async(v) => v.format(f),
            Self::Unsafe(v) => v.format(f),
            Self::Const(v) => v.format(f),
            Self::TryBlock(v) => v.format(f),
            Self::Assign(v) => v.format(f),
            Self::Range(v) => v.format(f),
            Self::Reference(v) => v.format(f),
            Self::Cast(v) => v.format(f),
            Self::Try(v) => v.format(f),
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
            Self::RawAddr(v) => v.format(f),
            Self::Verbatim(v) => f.text(v),
        }
    }
}

impl Format for ExprInfer {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.underscore.format(f)
    }
}

impl Format for ExprRawAddr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("&raw")?;
        f.space()?;
        self.mutability.format(f)?;
        f.space()?;
        self.expr.format(f)
    }
}

impl Format for ExprLit {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.lit.format(f)
    }
}

impl Format for ExprPath {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;

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
        self.attrs.format(f)?;
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
        self.attrs.format(f)?;

        if self.is_shorthand() {
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
        self.attrs.format(f)?;

        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.format(f)?;
            f.text(" ")?;
        }

        self.constness.format(f)?;

        if self.constness.is_some() {
            f.text(" ")?;
        }

        self.movability.format(f)?;

        if self.movability.is_some() {
            f.text(" ")?;
        }

        self.asyncness.format(f)?;

        if self.asyncness.is_some() {
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
        self.attrs.format(f)?;
        f.text("(")?;
        self.elems.inner.format(f)?;
        f.text(")")
    }
}

impl Format for ExprArray {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("[")?;
        self.elems.inner.format(f)?;
        f.text("]")
    }
}

impl Format for ExprRepeat {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("[")?;
        self.content.inner.elem.format(f)?;
        f.text("; ")?;
        self.content.inner.len.format(f)?;
        f.text("]")
    }
}

impl Format for ExprLet {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("let ")?;
        self.pat.format(f)?;
        f.text(" = ")?;
        self.expr.format(f)
    }
}

impl Format for ExprParen {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("(")?;
        self.content.inner.format(f)?;
        f.text(")")
    }
}

impl Format for ExprGroup {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.expr.format(f)
    }
}

impl Format for ExprMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.mac.format(f)
    }
}

impl Format for ExprReference {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("&")?;
        self.mutability.format(f)?;

        if self.mutability.is_some() {
            f.text(" ")?;
        }

        self.expr.format(f)
    }
}

impl Format for ExprUnary {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.op.format(f)?;
        self.expr.format(f)
    }
}

impl Format for ExprCast {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.expr.format(f)?;
        f.text(" as ")?;
        self.ty.format(f)
    }
}

impl Format for ExprTry {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.expr.format(f)?;
        f.text("?")
    }
}

impl Format for ExprBinary {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.left.format(f)?;
        f.text(" ")?;
        self.op.format(f)?;
        f.text(" ")?;
        self.right.format(f)
    }
}

impl Format for ExprAssign {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.left.format(f)?;
        f.text(" = ")?;
        self.right.format(f)
    }
}

impl Format for ExprRange {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;

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

impl Format for ExprCall {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.func.format(f)?;
        f.text("(")?;
        self.args.inner.format(f)?;
        f.text(")")
    }
}

impl Format for ExprMethodCall {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.receiver.format(f)?;
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
        self.attrs.format(f)?;
        self.base.format(f)?;
        f.text(".")?;
        self.member.format(f)
    }
}

impl Format for ExprIndex {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.base.format(f)?;
        f.text("[")?;
        self.index.inner.format(f)?;
        f.text("]")
    }
}

impl Format for ExprAwait {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.base.format(f)?;
        f.text(".await")
    }
}

impl Format for ExprBlock {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;

        if let Some(label) = &self.label {
            label.format(f)?;
            f.text(" ")?;
        }

        self.block.format(f)
    }
}

impl Format for ExprIf {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
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
        self.attrs.format(f)?;

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
        self.attrs.format(f)?;

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
        self.attrs.format(f)?;

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
        self.attrs.format(f)?;
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
        self.attrs.format(f)?;
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
        self.attrs.format(f)?;
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
        self.attrs.format(f)?;
        f.text("unsafe ")?;
        self.block.format(f)
    }
}

impl Format for ExprConst {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("const ")?;
        self.block.format(f)
    }
}

impl Format for ExprTryBlock {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("try ")?;
        self.block.format(f)
    }
}

impl Format for ExprReturn {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
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
        self.attrs.format(f)?;
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
        self.attrs.format(f)?;
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
        self.attrs.format(f)?;
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
