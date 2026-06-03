use moxy_ast::{
    Abi, AssignOp, Asyncness, BinOp, BoundPolarity, Constness, Defaultness, Ident, Label, Movability, Mutability, RangeLimits,
    TraitBoundModifier, UnOp, Unsafety, Visibility,
};

use crate::{Fmt, FmtError, Formatter};

impl Fmt for Ident {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Fmt for Label {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.name.fmt(f)?;
        f.text(":")
    }
}

impl Fmt for Abi {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("extern")?;

        if let Some(name) = &self.name {
            f.space()?;
            f.text(format!("\"{}\"", name))?;
        }

        Ok(())
    }
}

impl Fmt for Visibility {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Inherited => Ok(()),
            Self::Public { .. } => f.text("pub"),
            Self::Crate { .. } => f.text("pub(crate)"),
            Self::SelfValue { .. } => f.text("pub(self)"),
            Self::Super { .. } => f.text("pub(super)"),
            Self::Restricted { path, .. } => {
                f.text("pub(in ")?;
                path.fmt(f)?;
                f.text(")")
            }
        }
    }
}

impl Fmt for BinOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Fmt for AssignOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Fmt for UnOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self)
    }
}

impl Fmt for Asyncness {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Async => f.text("async"),
            Self::Sync => Ok(()),
        }
    }
}

impl Fmt for Constness {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Const => f.text("const"),
            Self::NoConst => Ok(()),
        }
    }
}

impl Fmt for Unsafety {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Unsafe => f.text("unsafe"),
            Self::Safe => Ok(()),
        }
    }
}

impl Fmt for Defaultness {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Default => f.text("default"),
            Self::Final => Ok(()),
        }
    }
}

impl Fmt for Mutability {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Mutable => f.text("mut"),
            Self::Immutable => Ok(()),
        }
    }
}

impl Fmt for Movability {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Static => f.text("static"),
            Self::Movable => Ok(()),
        }
    }
}

impl Fmt for RangeLimits {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Closed => f.text("..="),
            Self::HalfOpen => f.text(".."),
        }
    }
}

impl Fmt for TraitBoundModifier {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Maybe => f.text("?"),
            Self::None => Ok(()),
        }
    }
}

impl Fmt for BoundPolarity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Negative => f.text("!"),
            Self::Positive => Ok(()),
        }
    }
}
