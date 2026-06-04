use moxy_ast::stmt::{StmtLocal, StmtMacro};
use moxy_ast::{Stmt, StmtBlock};

use crate::{Fmt, FmtError, Formatter};

impl Fmt for StmtBlock {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("{")?;
        f.indent(|f| {
            for stmt in &self.stmts {
                f.hard_break()?;
                stmt.fmt(f)?;
            }

            Ok(())
        })?;

        if !self.stmts.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Fmt for Stmt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Local(v) => v.fmt(f),
            Self::Block(v) => v.fmt(f),
            Self::Item(v) => v.fmt(f),
            Self::Expr(expr, semi) => {
                expr.fmt(f)?;

                if semi.is_some() {
                    f.text(";")?;
                }

                Ok(())
            }
            Self::Macro(v) => v.fmt(f),
        }
    }
}

impl Fmt for StmtLocal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("let ")?;
        self.pat.fmt(f)?;

        if let Some((_, ty)) = &self.ty {
            f.text(": ")?;
            ty.fmt(f)?;
        }

        if let Some(init) = &self.init {
            f.text(" = ")?;
            init.expr.fmt(f)?;

            if let Some((_, diverge)) = &init.diverge {
                f.text(" else ")?;
                diverge.fmt(f)?;
            }
        }

        f.text(";")
    }
}

impl Fmt for StmtMacro {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.fmt(f)?;

        if self.semi.is_some() {
            f.text(";")?;
        }

        Ok(())
    }
}
