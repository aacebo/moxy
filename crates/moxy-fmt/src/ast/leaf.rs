use moxy_ast::*;
use moxy_token::Ident;

use crate::{FmtError, Format, Formatter};

impl Format for Ident {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Format for Label {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.name.format(f)?;
        f.text(":")
    }
}

impl Format for Abi {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("extern")?;

        if let Some(name) = &self.name {
            f.text(" ")?;
            f.text(format!("\"{}\"", name))?;
        }

        Ok(())
    }
}

impl Format for Visibility {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Inherited => Ok(()),
            Self::Public { .. } => f.text("pub"),
            Self::Crate { .. } => f.text("pub(crate)"),
            Self::SelfValue { .. } => f.text("pub(self)"),
            Self::Super { .. } => f.text("pub(super)"),
            Self::Restricted { path, .. } => {
                f.text("pub(in ")?;
                path.inner.1.format(f)?;
                f.text(")")
            }
        }
    }
}

impl Format for BinOp {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Format for UnOp {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Format for RangeLimits {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Closed(_) => f.text("..="),
            Self::HalfOpen(_) => f.text(".."),
        }
    }
}

impl Format for moxy_ast::Safety {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Safe(_) => f.text("safe"),
            Self::Unsafe(_) => f.text("unsafe"),
        }
    }
}

impl Format for moxy_ast::PatRangeLimits {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Closed(_) => f.text("..="),
            Self::HalfOpen(_) => f.text(".."),
            Self::Obsolete(_) => f.text("..."),
        }
    }
}

impl Format for PointerMutability {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Const(v) => v.format(f),
            Self::Mut(v) => v.format(f),
        }
    }
}
