use moxy_ast::*;

use super::path::format_qualified_path;
use crate::{FmtError, Format, Formatter};

impl Format for Type {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Never(_) => f.text("!"),
            Self::Infer(_) => f.text("_"),
            Self::Path(v) => v.format(f),
            Self::Tuple(v) => v.format(f),
            Self::Array(v) => v.format(f),
            Self::Slice(v) => v.format(f),
            Self::Reference(v) => v.format(f),
            Self::Pointer(v) => v.format(f),
            Self::BareFn(v) => v.format(f),
            Self::ImplTrait(v) => v.format(f),
            Self::TraitObject(v) => v.format(f),
            Self::Paren(v) => v.format(f),
            Self::Group(v) => v.elem.format(f),
            Self::Macro(v) => v.format(f),
            _ => Err(FmtError::unsupported("unsupported type variant")),
        }
    }
}

impl Format for TypePath {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        format_qualified_path(self.qself.as_ref(), &self.path, f)
    }
}

impl Format for TypeTuple {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.elems.inner.format(f)?;
        f.text(")")
    }
}

impl Format for TypeArray {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.content.inner.elem.format(f)?;
        f.text("; ")?;
        self.content.inner.len.format(f)?;
        f.text("]")
    }
}

impl Format for TypeSlice {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.elem.inner.format(f)?;
        f.text("]")
    }
}

impl Format for TypeReference {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("&")?;

        if let Some(lt) = &self.lifetime {
            lt.format(f)?;
            f.text(" ")?;
        }

        if self.mutability.is_some() {
            f.text("mut ")?;
        }

        self.elem.format(f)
    }
}

impl Format for TypePointer {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("*")?;

        match self.mutability {
            PointerMutability::Mut(_) => f.text("mut ")?,
            PointerMutability::Const(_) => f.text("const ")?,
        }

        self.elem.format(f)
    }
}

impl Format for TypeBareFn {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.format(f)?;
            f.text(" ")?;
        }

        self.unsafety.format(f)?;

        if self.unsafety.is_some() {
            f.text(" ")?;
        }

        if let Some(abi) = &self.abi {
            abi.format(f)?;
            f.text(" ")?;
        }

        f.text("fn(")?;
        self.params.inner.inputs.format(f)?;

        if self.params.inner.variadic.is_some() {
            if !self.params.inner.inputs.is_empty() {
                f.text(", ")?;
            }

            f.text("...")?;
        }

        f.text(")")?;
        self.output.format(f)
    }
}

impl Format for BareFnArg {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some((name, _)) = &self.name {
            name.format(f)?;
            f.text(": ")?;
        }

        self.ty.format(f)
    }
}

impl Format for TypeImplTrait {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("impl ")?;
        self.bounds.format(f)
    }
}

impl Format for TypeTraitObject {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.dyn_token.is_some() {
            f.text("dyn ")?;
        }

        self.bounds.format(f)
    }
}

impl Format for TypeParen {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.content.inner.format(f)?;
        f.text(")")
    }
}

impl Format for TypeMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.format(f)
    }
}
