use moxy_ast::{
    Abi, AssignOp, Asyncness, BinOp, BoundPolarity, Constness, Defaultness, Ident, Label, Movability, Mutability, RangeLimits,
    TraitBoundModifier, UnOp, Unsafety, Visibility,
};

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

impl Format for AssignOp {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Format for UnOp {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Format for Asyncness {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Async(_) => f.text("async"),
            Self::Sync => Ok(()),
        }
    }
}

impl Format for Constness {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Const(_) => f.text("const"),
            Self::NoConst => Ok(()),
        }
    }
}

impl Format for Unsafety {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Unsafe(_) => f.text("unsafe"),
            Self::Safe => Ok(()),
        }
    }
}

impl Format for Defaultness {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Default(_) => f.text("default"),
            Self::Final => Ok(()),
        }
    }
}

impl Format for Mutability {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Mutable(_) => f.text("mut"),
            Self::Immutable => Ok(()),
        }
    }
}

impl Format for Movability {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Static(_) => f.text("static"),
            Self::Movable => Ok(()),
        }
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

impl Format for TraitBoundModifier {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Maybe(_) => f.text("?"),
            Self::None => Ok(()),
        }
    }
}

impl Format for BoundPolarity {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Negative(_) => f.text("!"),
            Self::Positive => Ok(()),
        }
    }
}
