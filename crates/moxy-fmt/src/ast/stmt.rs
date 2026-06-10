use moxy_ast::stmt::{StmtLocal, StmtMacro};
use moxy_ast::{Stmt, StmtBlock};

use crate::{FmtError, Format, Formatter};

impl Format for StmtBlock {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("{")?;
        f.indent(|f| {
            for stmt in &self.stmts.inner {
                f.hard_break()?;
                stmt.format(f)?;
            }

            Ok(())
        })?;

        if !self.stmts.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Format for Stmt {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Local(v) => v.format(f),
            Self::Block(v) => v.format(f),
            Self::Item(v) => v.format(f),
            Self::Expr(expr, semi) => {
                expr.format(f)?;

                if semi.is_some() {
                    f.text(";")?;
                }

                Ok(())
            }
            Self::Macro(v) => v.format(f),
        }
    }
}

impl Format for StmtLocal {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("let ")?;
        self.pat.format(f)?;

        if let Some((_, ty)) = &self.ty {
            f.text(": ")?;
            ty.format(f)?;
        }

        if let Some(init) = &self.init {
            f.text(" = ")?;
            init.expr.format(f)?;

            if let Some((_, diverge)) = &init.diverge {
                f.text(" else ")?;
                diverge.format(f)?;
            }
        }

        f.text(";")
    }
}

impl Format for StmtMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.format(f)?;

        if self.semi.is_some() {
            f.text(";")?;
        }

        Ok(())
    }
}
