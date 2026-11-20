use moxy_ast::*;

use super::path::format_qualified_path;
use crate::{FmtError, Format, Formatter};

impl Format for Expr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.nest(|f| match self {
            Self::Unary(v) => v.format(f),
            Self::BinaryChain(v) => v.format(f),
            Self::Block(v) => v.format(f),
            Self::Infer(v) => v.format(f),
            Self::Return(v) => v.format(f),
            Self::Break(v) => v.format(f),
            Self::Continue(v) => v.format(f),
            Self::Yield(v) => v.format(f),
            Self::IfChain(v) => v.format(f),
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
            Self::CastChain(v) => v.format(f),
            Self::Lit(v) => v.format(f),
            Self::Path(v) => v.format(f),
            Self::Postfix(v) => v.format(f),
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
            Self::Verbatim(v) => {
                f.check_tokens(v)?;
                f.text(v)
            }
            _ => Err(FmtError::unsupported("unsupported expression variant")),
        })
    }
}

impl Format for ExprInfer {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        self.underscore.format(f)
    }
}

impl Format for ExprRawAddr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("&raw")?;
        f.space()?;
        self.mutability.format(f)?;
        f.space()?;
        self.expr.format(f)
    }
}

impl Format for ExprLit {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        self.lit.format(f)
    }
}

impl Format for ExprPath {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        format_qualified_path(self.qself.as_ref(), &self.path, f)
    }
}

impl Format for ExprStruct {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        format_qualified_path(self.qself.as_ref(), &self.path, f)?;
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.body.inner.fields.pairs() {
                f.hard_break()?;

                match pair {
                    Pair::List(fv, _) => {
                        fv.format(f)?;
                        f.text(",")?;
                    }
                    Pair::End(fv) => {
                        fv.format(f)?;
                        f.text(",")?;
                    }
                }
            }

            if let Some((_, rest)) = &self.body.inner.rest {
                f.hard_break()?;
                f.text("..")?;

                if let Some(rest) = rest {
                    rest.format(f)?;
                }
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
        super::attr::inline_attrs(&self.attrs, f)?;

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
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("(")?;
        self.elems.inner.format(f)?;

        // A tuple of one element keeps its comma, as in `(a,)`
        if self.elems.inner.len() == 1 {
            f.text(",")?;
        }

        f.text(")")
    }
}

impl Format for ExprArray {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("[")?;
        self.elems.inner.format(f)?;
        f.text("]")
    }
}

impl Format for ExprRepeat {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("[")?;
        self.content.inner.elem.format(f)?;
        f.text("; ")?;
        self.content.inner.len.format(f)?;
        f.text("]")
    }
}

impl Format for ExprLet {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("let ")?;
        self.pat.format(f)?;
        f.text(" = ")?;
        self.expr.format(f)
    }
}

impl Format for ExprParen {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("(")?;
        self.content.inner.format(f)?;
        f.text(")")
    }
}

impl Format for ExprGroup {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        self.expr.format(f)
    }
}

impl Format for ExprMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        self.mac.format(f)
    }
}

impl Format for ExprReference {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
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
        super::attr::inline_attrs(&self.attrs, f)?;
        self.op.format(f)?;
        self.expr.format(f)
    }
}

impl Format for ExprCastChain {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        self.expr.format(f)?;

        for (_, ty) in &self.casts {
            f.text(" as ")?;
            ty.format(f)?;
        }

        Ok(())
    }
}

impl Format for PostfixTry {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("?")
    }
}

impl Format for ExprBinaryChain {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;

        for pair in self.operands.pairs() {
            pair.value().format(f)?;

            if let Some(op) = pair.punct() {
                f.text(" ")?;
                op.format(f)?;
                f.text(" ")?;
            }
        }

        Ok(())
    }
}

impl Format for ExprAssign {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        self.left.format(f)?;
        f.text(" = ")?;
        self.right.format(f)
    }
}

impl Format for ExprRange {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;

        if let Some(start) = &self.start {
            start.format(f)?;
        }

        self.limits.format(f)?;

        if let Some(end) = &self.end {
            // `a.. ..b` written together would lex as `...`
            if matches!(&**end, Expr::Range(range) if range.start.is_none()) {
                f.text(" ")?;
            }

            end.format(f)?;
        }

        Ok(())
    }
}

impl Format for ExprPostfix {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        self.base.format(f)?;

        for op in &self.ops {
            op.format(f)?;
        }

        Ok(())
    }
}

impl Format for Postfix {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Call(v) => v.format(f),
            Self::MethodCall(v) => v.format(f),
            Self::Field(v) => v.format(f),
            Self::Index(v) => v.format(f),
            Self::Try(v) => v.format(f),
            Self::Await(v) => v.format(f),
        }
    }
}

impl Format for PostfixCall {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.args.inner.format(f)?;
        f.text(")")
    }
}

impl Format for PostfixMethodCall {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
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

impl Format for PostfixField {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(".")?;
        self.member.format(f)
    }
}

impl Format for PostfixIndex {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.index.inner.format(f)?;
        f.text("]")
    }
}

impl Format for PostfixAwait {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(".await")
    }
}

impl Format for ExprBlock {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;

        if let Some(label) = &self.label {
            label.format(f)?;
            f.text(" ")?;
        }

        self.block.format(f)
    }
}

impl Format for ExprIfChain {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("if ")?;
        self.cond.format(f)?;
        f.text(" ")?;
        self.then_branch.format(f)?;

        for branch in &self.else_ifs {
            f.text(" else if ")?;
            branch.cond.format(f)?;
            f.text(" ")?;
            branch.then_branch.format(f)?;
        }

        if let Some(else_branch) = &self.else_branch {
            f.text(" else ")?;
            else_branch.format(f)?;
        }

        Ok(())
    }
}

impl Format for ExprWhile {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;

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
        super::attr::inline_attrs(&self.attrs, f)?;

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
        super::attr::inline_attrs(&self.attrs, f)?;

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
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("match ")?;
        self.expr.format(f)?;
        f.text(" {")?;
        f.indent(|f| {
            super::attr::inner_attrs(&self.inner_attrs, f)?;

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
        super::attr::inline_attrs(&self.attrs, f)?;
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
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("unsafe ")?;
        self.block.format(f)
    }
}

impl Format for ExprConst {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("const ")?;
        self.block.format(f)
    }
}

impl Format for ExprTryBlock {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
        f.text("try ")?;
        self.block.format(f)
    }
}

impl Format for ExprReturn {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        super::attr::inline_attrs(&self.attrs, f)?;
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
        super::attr::inline_attrs(&self.attrs, f)?;
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
        super::attr::inline_attrs(&self.attrs, f)?;
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
        super::attr::inline_attrs(&self.attrs, f)?;
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
